use reqwest::blocking::{Client, Response};
use reqwest::header::{ACCEPT, USER_AGENT};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::thread;
use std::time::Duration;

const APP_USER_AGENT: &str = "sigtor.org";
const MAX_RETRIES: usize = 3;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> HttpClient {
        let client = Client::new();
        HttpClient { client }
    }

    pub fn fetch_text(&self, url: &str) -> Result<String, ()> {
        match self.fetch(url, false) {
            Ok(resp) => match resp.text() {
                Ok(value) => Ok(value),
                Err(e) => {
                    warn!("Unable to consume body: {}", e);
                    Err(())
                }
            },
            Err(_) => Err(()),
        }
    }

    pub fn fetch_json<T: DeserializeOwned>(&self, url: &str) -> Result<T, ()> {
        match self.fetch(url, true) {
            Ok(resp) => match resp.json() {
                Ok(value) => Ok(value),
                Err(e) => {
                    warn!("Unable to deserialize: {}, {}", e, url);
                    Err(())
                }
            },
            Err(_) => Err(()),
        }
    }

    fn fetch(&self, url: &str, is_json: bool) -> Result<Response, ()> {
        self.fetch_with_retry(url, is_json, 0)
    }

    fn fetch_with_retry(&self, url: &str, is_json: bool, attempts: usize) -> Result<Response, ()> {
        let accept_header = if is_json {
            "application/json"
        } else {
            "text/plain"
        };

        // Backoff just a bit when retrying
        thread::sleep(Duration::from_secs(attempts as u64));

        match self
            .client
            .get(url)
            .header(ACCEPT, accept_header)
            .header(USER_AGENT, APP_USER_AGENT)
            .send()
        {
            Ok(resp) => {
                if resp.status() != StatusCode::OK {
                    warn!("Unsuccessful HTTP call {}: {}", resp.url(), resp.status());
                    return Err(());
                }
                Ok(resp)
            }
            Err(_) => {
                if attempts < MAX_RETRIES {
                    info!("Retrying {}", url);
                    self.fetch_with_retry(url, is_json, attempts + 1)
                } else {
                    warn!("Max number of retries for {}", url);
                    Err(())
                }
            }
        }
    }
}
