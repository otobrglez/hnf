use std::error::Error;

use futures::{Future, Stream, StreamExt};
use log::debug;

use crate::hn_client::Item;
use crate::hn_client2::HNClient2;

mod config;
mod hn_client;
mod hn_client2;
mod hn_error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let logger_path = "config/log4rs.yml";
    log4rs::init_file(logger_path, Default::default()).unwrap();
    debug!("Booting with logger configuration from {}", logger_path);

    let stories: Vec<Item> = HNClient2::collect_with(10, HNClient2::top_stories).await;

    for (i, item) in stories.iter().enumerate() {
        println!(
            "{} - [{:?}] {:?} @ {:?}",
            i + 1,
            item.item_type,
            item.title,
            item.url
        )
    }

    Ok(())
}
