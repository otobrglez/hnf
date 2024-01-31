use std::result::Result;
use std::time::Duration;
use serde_json::Value as JSON;
use ureq::{Agent, Response};

use crate::config::hn_api_url;
use crate::hn_error::HNError;
use crate::models::{Item, ItemId};


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
