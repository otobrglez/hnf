use std::error::Error;

use log::debug;

use lamecore::hn_client2::HNClient2;
use lamecore::models::Item;

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
