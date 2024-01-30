use log::debug;

use crate::hn_client::HNClient;

mod config;
mod hn_client;
mod hn_error;

fn main() {
    let logger_path = "config/log4rs.yml";
    log4rs::init_file(logger_path, Default::default()).unwrap();

    debug!("Booting with logger configuration from {}", logger_path);

    let size = 10;
    let mut ht_client = HNClient::new();
    let item_ids: Vec<_> = ht_client
        .top_stories()
        .expect("Failed getting stories")
        .into_iter()
        .clone()
        .take(size)
        .collect();

    for (i, item) in item_ids
        .into_iter()
        .clone()
        .map(|item_id| ht_client.clone().item(&item_id).unwrap())
        .enumerate()
    {
        println!(
            "{} - [{:?}] {:?} @ {:?}",
            i + 1,
            item.item_type,
            item.title,
            item.url.ok_or("none")
        )
    }
}
