use futures::{stream, StreamExt};
use reqwest::get;

use crate::config::hn_api_url;
use crate::hn_client::{Item, ItemId};

pub struct HNClient2 {}
impl HNClient2 {
    pub async fn item(item_id: &ItemId) -> Item {
        get(hn_api_url(&format!("/item/{}.json", item_id)))
            .await.unwrap().json().await.unwrap()
    }

    async fn stories(path: &str) -> Vec<ItemId> {
        get(hn_api_url(path)).await.unwrap().json().await.unwrap()
    }

    pub async fn top_stories() -> Vec<ItemId> {
        Self::stories("/topstories.json").await
    }
    pub async fn best_stories() -> Vec<ItemId> {
        Self::stories("/beststories.json").await
    }

    pub async fn collect_with<F, Fut>(n: usize, collect_stories: F) -> Vec<Item>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Vec<ItemId>> + Send + 'static,
    {
        let buffer_size = 4;
        let all_top_stories: Vec<ItemId> = collect_stories().await;
        let top_stories: Vec<&ItemId> = all_top_stories.iter().clone().take(n).collect();
        stream::iter(top_stories)
            .map(Self::item)
            .buffered(buffer_size)
            .collect()
            .await
    }

    pub async fn collect_top_stories_n(n: usize) -> Vec<Item> {
        Self::collect_with(n, Self::top_stories).await
    }

    pub async fn collect_best_stories_n(n: usize) -> Vec<Item> {
        Self::collect_with(n, Self::best_stories).await
    }
}
