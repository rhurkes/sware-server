use crate::http_client::HttpClient;
use crate::store::Store;
use domain::{ListProduct, Product, ProductsResult};
use parsers::nws_parser;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use util;

const API_HOST: &str = "https://api.weather.gov";
const POLL_INTERVAL_SECONDS: u64 = 60;

lazy_static! {
    static ref HTTP_CLIENT: HttpClient = HttpClient::new();
    static ref PRODUCT_CODES: Vec<&'static str> =
        vec!["afd", "ffw", "lsr", "sel", "svr", "svs", "swo", "tor"];
}

pub fn run(writer: &Arc<Store>) {
    info!("starting");

    PRODUCT_CODES.iter().for_each(|code| {
        let product_writer = writer.clone();
        thread::Builder::new()
            .name(format!("{}_fetcher", code))
            .spawn(move || {
                let url = format!("{}/products/types/{}", API_HOST, code);
                let mut last_product_ts = util::get_system_micros();

                loop {
                    let start = util::get_system_secs();

                    // Get the list of all events for this product
                    if let Ok(product_list) = HTTP_CLIENT.fetch_json::<ProductsResult>(&url) {
                        let new_products = get_new_products(last_product_ts, product_list);

                        if !new_products.is_empty() {
                            // info!("found {} new {} event[s]", new_products.len(), code);
                            last_product_ts =
                                util::ts_to_ticks(&new_products[0].issuance_time).unwrap();
                        }

                        // Fetch all new events, run each through the parser, and store in the db
                        new_products
                            .iter()
                            .map(|x| match HTTP_CLIENT.fetch_json::<Product>(&x._id) {
                                Ok(value) => Some(value),
                                Err(_) => None,
                            })
                            .filter(Option::is_some)
                            .map(|x| nws_parser::parse(&x.unwrap()))
                            .filter(Option::is_some)
                            .for_each(|event| product_writer.put_event(&mut event.unwrap()));
                    }

                    let elapsed_seconds = util::get_system_secs() - start;
                    let delay = POLL_INTERVAL_SECONDS.saturating_sub(elapsed_seconds);
                    thread::sleep(Duration::from_secs(delay));
                }
            })
            .expect("Unable to create thread");
    });
}

/**
 * Returns products newer than the latest seen. A simple take_while could suffice, but that
 * carries the possibility of missing products due to an unparseable datetime string.
 */
fn get_new_products(last_ts: u64, products_result: ProductsResult) -> Vec<ListProduct> {
    let mut new_products: Vec<ListProduct> = vec![];

    for product in products_result.products {
        if let Ok(ticks) = util::ts_to_ticks(&product.issuance_time) {
            if ticks <= last_ts {
                break;
            }
            new_products.push(product);
        }
    }

    new_products
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::fs::File;
    // use std::io::Read;
}
