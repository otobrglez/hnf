use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::Deserialize;

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
