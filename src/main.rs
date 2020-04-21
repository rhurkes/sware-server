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
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let store = Arc::new(Store::new());
    let mut threads = vec![];
    let sn_store = store.clone();
    let nws_api_store = store.clone();

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

    warp::serve(filters(store))
        .run(([127, 0, 0, 1], 8080))
        .await;
}

fn with_store(
    store: Arc<Store>,
) -> impl Filter<Extract = (Arc<Store>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || store.clone())
}

fn filters(
    store: Arc<Store>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    events_filter(store.clone()).or(stats_filter(store))
}

// GET /events/:u128
fn events_filter(
    store: Arc<Store>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("events" / u128)
        .and(warp::get())
        .and(with_store(store))
        .map(events_handler)
        .with(warp::cors().allow_any_origin())
}

// GET /stats
fn stats_filter(
    store: Arc<Store>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("stats")
        .and(warp::get())
        .and(with_store(store))
        .map(stats_handler)
}

fn events_handler(id: u128, store: Arc<Store>) -> impl warp::Reply {
    let events = store.get_events(id);
    warp::reply::json(&events)
}

fn stats_handler(store: Arc<Store>) -> impl warp::Reply {
    let stats = store.get_stats();
    warp::reply::json(&stats)
}
