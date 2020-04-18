use crate::http_client::HttpClient;
use crate::store::Store;
use fnv::FnvHashSet;
use parsers::sn_parser;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use util;

const API_URL: &str = "http://www.spotternetwork.org/feeds/reports.txt";
const POLL_INTERVAL_SECONDS: u64 = 60;

lazy_static! {
    static ref HTTP_CLIENT: HttpClient = HttpClient::new();
}

#[derive(Debug)]
pub struct Comparison {
    latest_set: FnvHashSet<String>,
    new: Vec<String>,
}

pub fn run(writer: &Arc<Store>) {
    let mut seen: FnvHashSet<String> = FnvHashSet::default();
    info!("starting");

    loop {
        let start = util::get_system_secs();

        if let Ok(body) = HTTP_CLIENT.fetch_text(API_URL) {
            let comparison = get_comparison(&body, seen);
            seen = comparison.latest_set;
            comparison
                .new
                .iter()
                .map(|report| sn_parser::parse(report))
                .for_each(|event| {
                    if let Some(mut event) = event {
                        writer.put_event(&mut event)
                    }
                });
        };

        let elapsed_seconds = util::get_system_secs() - start;
        let delay = POLL_INTERVAL_SECONDS.saturating_sub(elapsed_seconds);
        thread::sleep(Duration::from_secs(delay));
    }
}

fn get_comparison(body: &str, seen: FnvHashSet<String>) -> Comparison {
    let latest_set: FnvHashSet<String> = body
        .lines()
        .filter(|x| x.starts_with("Icon:"))
        .map(|x| normalize_line(x))
        .collect();

    let new: Vec<String> = latest_set
        .iter()
        .filter_map(|x| {
            if !seen.contains(x) {
                Some(x.to_string())
            } else {
                None
            }
        })
        .collect();

    Comparison { latest_set, new }
}

/**
 * Normalizes raw report lines as returned by the SpotterNetwork API. Since there is no offset,
 * you will see the same report multiple times and need to de-dupe. Unfortunately, the same
 * report will have the icon image digit change as the report ages so we need to normalize.
 */
fn normalize_line(line: &str) -> String {
    line.replace(",000,3", ",000,0")
        .replace(",000,4", ",000,0")
        .replace(",000,5", ",000,0")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn normalize_line_should_zero_icon_digit() {
        let line = r#"Icon: 47.617706,-111.215248,000,4,4,"Reported By: Test User\nHail\nTime: 2018-09-20 22:49:29 UTC\nSize: 0.75" (Penny)\nNotes: None""#;
        let expected = r#"Icon: 47.617706,-111.215248,000,0,4,"Reported By: Test User\nHail\nTime: 2018-09-20 22:49:29 UTC\nSize: 0.75" (Penny)\nNotes: None""#;
        let normalized = normalize_line(line);
        assert_eq!(normalized, expected);
    }

    #[test]
    fn empty_report_should_return_no_seen_or_unseen() {
        let mut file = File::open("data/reports-empty").expect("unable to open file");
        let mut body = String::new();
        file.read_to_string(&mut body).expect("unable to read file");
        let comparison = get_comparison(&body, FnvHashSet::default());
        assert_eq!(comparison.latest_set.len(), 0);
        assert_eq!(comparison.new.len(), 0);
    }

    #[test]
    fn no_current_seen_should_return_all_reports() {
        let mut file = File::open("data/reports").expect("unable to open file");
        let mut body = String::new();
        file.read_to_string(&mut body).expect("unable to read file");
        let comparison = get_comparison(&body, FnvHashSet::default());
        assert_eq!(comparison.latest_set.len(), 23);
        assert_eq!(comparison.new.len(), 23);
    }

    #[test]
    fn same_report_different_age_digit_should_be_deduped() {
        let body = r#"Icon: 47.617706,-111.215248,000,4,4,"Reported By: Test User\nHail\nTime: 2018-09-20 22:39:00 UTC\nSize: 0.75" (Penny)\nNotes: None""#;
        let comparison = get_comparison(&body, FnvHashSet::default());
        assert_eq!(comparison.latest_set.len(), 1);
        assert_eq!(comparison.new.len(), 1);

        let body = r#"Icon: 47.617706,-111.215248,000,5,4,"Reported By: Test User\nHail\nTime: 2018-09-20 22:39:00 UTC\nSize: 0.75" (Penny)\nNotes: None"
            Icon: 47.617706,-111.215248,000,6,4,"Reported By: Test User\nHail\nTime: 2018-09-20 22:39:00 UTC\nSize: 0.75" (Penny)\nNotes: None""#;
        let comparison = get_comparison(&body, comparison.latest_set);
        assert_eq!(comparison.latest_set.len(), 1);
        assert_eq!(comparison.new.len(), 0);
    }

    #[test]
    fn get_comparison_should_handle_previously_seen_reports() {
        let mut file = File::open("data/reports").expect("unable to open file");
        let mut body = String::new();
        file.read_to_string(&mut body).expect("unable to read file");

        let seen: FnvHashSet<String> = vec![
            "Icon: 41.338901,-96.059708,000,0,5,\"Reported By: Will Dupe\\nHigh Wind\\nTime: 2018-09-21 00:26:06 UTC\\n50 mphNotes: None\"".to_string(),
            "Icon: 47.617706,-111.215248,000,0,4,\"Reported By: Will Dupe\\nHail\\nTime: 2018-09-20 22:49:29 UTC\\nSize: 0.75\" (Penny)\\nNotes: None\"".to_string(),
            "Icon: 43.112000,-94.610001,000,0,6,\"Reported By: Will Dupe\\nFlooding\\nTime: 2018-09-20 22:58:00 UTC\\nNotes: Water over road on US 18\"".to_string(),
            "Icon: 41.338715,-96.059563,000,0,5,\"Reported By: Will Dupe\\nHigh Wind\\nTime: 2018-09-21 00:34:00 UTC\\n60 mphNotes: Wind gusting to 63mph\"".to_string(),
            "Icon: 35.851399,-90.708198,000,0,8,\"Reported By: Will Dupe\\nOther - See Note\\nTime: 2018-11-14 20:22:00 UTC\\nNotes: i got snow and a little of sleet\"".to_string(),
            "Icon: 41.230400,-95.850403,000,0,3,\"Reported By: Will Dupe\\nNot Rotating Wall Cloud\\nTime: 2018-09-21 00:34:00 UTC\\nNotes: None\"".to_string(),
        ].into_iter().collect();

        let seen_length = seen.len();
        let comparison = get_comparison(&body, seen);

        assert_eq!(comparison.latest_set.len(), 23);
        assert_eq!(
            comparison.new.len(),
            comparison.latest_set.len() - seen_length
        );
    }
}
