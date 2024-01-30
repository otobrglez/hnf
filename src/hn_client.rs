use std::result::Result;
use std::time::Duration;

use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value as JSON;
use ureq::{Agent, Response};

use crate::config::hn_api_url;
use crate::hn_error::HNError;

pub type ItemId = u32;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Story,
    Comment,
    Job,
    Poll,
    PollOpt,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Item {
    pub by: String,
    pub id: ItemId,
    pub title: String,
    pub url: Option<String>,
    #[serde(alias = "type")]
    pub item_type: ItemType,
    #[serde(default = "default_kids")]
    pub kids: Vec<ItemId>,
    pub score: u32,
    #[serde(with = "ts_seconds")]
    pub time: DateTime<Utc>,
    pub parent: Option<ItemId>,
}

fn default_kids() -> Vec<ItemId> {
    vec![]
}

#[derive(Clone)]
pub struct HNClient {
    agent: Agent,
}
impl HNClient {
    pub fn new() -> Self {
        HNClient {
            agent: ureq::AgentBuilder::new()
                .timeout_connect(Duration::from_secs(2))
                .timeout_read(Duration::from_secs(2))
                .timeout_write(Duration::from_secs(2))
                .build(),
        }
    }

    pub fn new_with_agent(agent: Agent) -> Self {
        HNClient { agent }
    }

    fn parse_json(response: Response) -> Result<JSON, HNError> {
        response
            .into_json()
            .map_err(|je| HNError::Json(je.to_string()))
    }

    fn convert<O: for<'de> ureq::serde::Deserialize<'de>>(value: JSON) -> Result<O, HNError> {
        serde_json::from_value(value).map_err(HNError::from)
    }

    fn stories(&mut self, path: &str) -> Result<Vec<ItemId>, HNError> {
        self.agent
            .get(&hn_api_url(path))
            .call()
            .map_err(HNError::from)
            .and_then(Self::parse_json)
            .and_then(Self::convert)
    }

    pub fn top_stories(&mut self) -> Result<Vec<ItemId>, HNError> {
        self.stories("/topstories.json")
    }

    pub fn best_stories(&mut self) -> Result<Vec<ItemId>, HNError> {
        self.stories("/beststories.json")
    }

    pub fn item(&mut self, item_id: &ItemId) -> Result<Item, HNError> {
        self.agent
            .get(&hn_api_url(&format!("/item/{}.json", item_id)))
            .call()
            .map_err(HNError::from)
            .and_then(Self::parse_json)
            .and_then(Self::convert)
    }
}
