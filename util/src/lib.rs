#[macro_use]
extern crate log;

use chrono::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

/**
 * Safely unwraps a Result<T, E> if Ok, otherwise returns the calling function with None.
 * Useful for early exiting in functions that return Option<T>.
 */
#[macro_export]
macro_rules! safe_result {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return None,
        }
    };
}

/**
 * Safely unwraps an Option<T> if Some, otherwise returns the calling function with None.
 * Useful for early exiting in functions that return Option<T>.
 */
#[macro_export]
macro_rules! safe_option {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => return None,
        }
    };
}

/**
 * Converts an RFC3339 timestamp to microsecond ticks.
 */
pub fn ts_to_ticks(input: &str) -> Result<u64, ()> {
    match Utc.datetime_from_str(input, "%Y-%m-%dT%H:%M:%S+00:00") {
        Ok(dt) => Ok(dt.timestamp() as u64 * 1_000_000),
        Err(_) => {
            warn!("Unable to convert ts {}", input);
            Err(())
        }
    }
}

pub fn get_system_micros() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    since_the_epoch.as_secs() * 1_000_000 + u64::from(since_the_epoch.subsec_micros())
}

pub fn get_system_secs() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    since_the_epoch.as_secs()
}

pub fn get_system_millis() -> u64 {
    get_system_micros() / 1000
}

pub fn tz_to_offset(input: &str) -> Result<&str, ()> {
    match input {
        "HST" => Ok("-1000"),
        "HDT" => Ok("-0900"),
        "AKST" => Ok("-0900"),
        "AKDT" => Ok("-0800"),
        "PST" => Ok("-0800"),
        "PDT" => Ok("-0700"),
        "MST" => Ok("-0700"),
        "MDT" => Ok("-0600"),
        "CST" => Ok("-0600"),
        "CDT" => Ok("-0500"),
        "EST" => Ok("-0500"),
        "EDT" => Ok("-0400"),
        "AST" => Ok("-0400"),
        "ADT" => Ok("-0300"),
        _ => {
            warn!("Unknown timezone {}", input);
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_system_micros_should_return_value_in_correct_range() {
        let result = get_system_micros();
        assert!(result > 1551209606990457); // ts when test was first written
        assert!(result < 1900000000000000); // year 2030
    }

    #[test]
    fn ts_to_ticks_should_return_ticks() {
        let ts = "2018-11-25T22:46:23+00:00";
        let result = ts_to_ticks(&ts).unwrap();
        assert_eq!(result, 1543185983000000);
    }
}
