use chrono::prelude::*;
use regex::Match;

pub fn short_time_to_ticks(input: &str) -> Result<u64, ()> {
    match Utc.datetime_from_str(input, "%y%m%dT%H%MZ") {
        Ok(dt) => Ok((dt.timestamp() as u64) * 1_000_000),
        Err(_) => {
            warn!("Unable to parse short time {}", input);
            Err(())
        }
    }
}

pub fn cap(m: Option<Match>) -> &str {
    m.unwrap().as_str()
}

pub fn str_to_latlon(input: &str, invert: bool) -> f32 {
    let sign = if invert { -1.0 } else { 1.0 };
    let mut value = input.parse::<f32>().unwrap();
    // longitudes are inverted, and values over 100 can drop the '1'
    if invert && value < 5000.0 {
        value += 10000.0;
    }
    value / 100.0 * sign
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_latlon_should_parse_correctly() {
        let tests = vec![
            // input, invert, expected
            ("3000", false, 30.0),
            ("3156", false, 31.56),
            ("9234", true, -92.34),
            ("9000", true, -90.0),
            ("0156", true, -101.56),
            ("10156", true, -101.56),
        ];

        tests.iter().for_each(|x| {
            let result = str_to_latlon(x.0, x.1);
            assert_eq!(x.2, result);
        });
    }

    #[test]
    fn short_time_to_ticks_should_return_correct_ticks() {
        let short_time = "190522T2100Z";
        let result = short_time_to_ticks(short_time).unwrap();
        assert_eq!(result, 1558558800000000);
    }
}
