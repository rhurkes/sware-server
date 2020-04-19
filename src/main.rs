#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use log::LevelFilter;
use std::sync::Arc;
use std::thread;
use store::Store;
use warp::Filter;

mod http_client;
mod nws_loader;
mod sn_loader;
mod store;

#[tokio::main]
async fn main() {
    let log_level = std::env::var("SWARE_LOG_LEVEL").unwrap_or_default();
    env_logger::builder()
        .filter_level(get_log_level(&log_level))
        .init();

    let mut threads = vec![];
    let store = Arc::new(Store::new());
    let sn_store = store.clone();
    let nws_api_store = store.clone();
    let with_store = warp::any().map(move || store.clone());

    // Run SpotterNetwork loader
    threads.push(
        thread::Builder::new()
            .name("sn_loader".to_string())
            .spawn(move || {
                sn_loader::run(&sn_store);
            }),
    );

    // Run NWS API loader
    threads.push(
        thread::Builder::new()
            .name("nws_api_loader".to_string())
            .spawn(move || {
                nws_loader::run(&nws_api_store);
            }),
    );

    // GET /events/:u128
    let events_route = warp::path!("events" / u128)
        .and(warp::get())
        .and(with_store)
        .map(get_events_handler);

    warp::serve(events_route).run(([127, 0, 0, 1], 8080)).await;
}

fn get_events_handler(id: u128, store: Arc<Store>) -> impl warp::Reply {
    let events = store.get_events(id);
    warp::reply::json(&events)
}

fn get_log_level(input: &str) -> LevelFilter {
    match input.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        _ => LevelFilter::Info,
    }
}
