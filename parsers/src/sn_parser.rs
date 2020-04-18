use chrono::prelude::*;
use domain::{Coordinates, Event, EventType, HazardType, Location, Report, Units};
use regex::Regex;

const REPORT_PATTERN: &str = r"Icon: (?P<lat>\d{2}\.\d{6}),(?P<lon>-\d{2,3}\.\d{6}),000,\d,(?P<hazard_code>\d),.Reported By: (?P<reporter>.+)\\n.+\\nTime: (?P<ts>.+) UTC(?:\\nSize: (?P<size>\d{1,2}\.\d{2}).+?)*(?:\\n(?P<mph>\d{1,3}) mph)*(?P<measured> \[Measured\])*.+otes: (?P<notes>.+).$";

lazy_static! {
    static ref REPORT_REGEX: Regex = Regex::new(REPORT_PATTERN).expect("Unable to compile regex");
}

pub fn parse(report: &str) -> Option<Event> {
    let captures = REPORT_REGEX.captures(report);

    if captures.is_none() {
        warn!("invalid spotter network report format: {}", report);
        return None;
    }

    let captures = captures.unwrap();
    let hazard = Hazard::get_by_code(captures.name("hazard_code").unwrap().as_str());
    let notes = captures.name("notes").unwrap().as_str();
    let reporter = captures.name("reporter").unwrap().as_str();

    // Skip Other/None reports since they're essentially worthless
    if hazard == Hazard::Other && notes == "None" {
        return None;
    }

    let mut report = Report {
        hazard: hazard.to_hazard_type(),
        magnitude: None,
        report_ts: None, // not set for SN reports
        reporter: reporter.to_string(),
        units: None,
        was_measured: None,
    };

    if captures.name("measured").is_some() {
        report.was_measured = Some(true);
    }

    let mph_cap = captures.name("mph");
    let size_cap = captures.name("size");
    let mut title = format!("Report: {}", hazard.to_string());

    if mph_cap.is_some() {
        let mph = mph_cap.unwrap().as_str().parse().unwrap_or_default();
        title = format!("Report: {}mph {}", mph, hazard.to_string());
        report.magnitude = Some(mph);
        report.units = Some(Units::Mph);
    } else if size_cap.is_some() {
        let size = size_cap.unwrap().as_str().parse().unwrap_or_default();
        title = format!("Report: {}\" {}", size, hazard.to_string());
        report.magnitude = Some(size);
        report.units = Some(Units::Inches);
    }

    let location = Some(Location {
        county: None,
        wfo: None,
        point: Some(Coordinates {
            lat: captures
                .name("lat")
                .unwrap()
                .as_str()
                .parse()
                .unwrap_or_default(),
            lon: captures
                .name("lon")
                .unwrap()
                .as_str()
                .parse()
                .unwrap_or_default(),
        }),
        poly: None,
    });

    let event_ts = Utc
        .datetime_from_str(captures.name("ts").unwrap().as_str(), "%Y-%m-%d %H:%M:%S")
        .unwrap()
        .timestamp() as u64
        * 1_000_000;

    let text = if notes == "None" {
        format!("{} reported by {}", hazard.to_string(), reporter)
    } else {
        format!("{} reported by {}. {}", hazard.to_string(), reporter, notes)
    };

    let event = Event {
        event_ts,
        event_type: EventType::SnReport,
        expires_ts: None,
        ext_uri: None,
        ingest_ts: 0, // set when storing
        location,
        md: None,
        outlook: None,
        report: Some(report),
        text: Some(text),
        title,
        valid_ts: None,
        warning: None,
        watch: None,
    };

    Some(event)
}

#[derive(Deserialize, Eq, PartialEq, Serialize, Clone)]
pub enum Hazard {
    Tornado = 0isize,
    Funnel,
    WallCloud,
    Hail,
    Wind,
    Flood,
    FlashFlood,
    Other,
    FreezingRain,
    Snow,
}

impl Hazard {
    pub fn get_by_code(code: &str) -> Hazard {
        match code {
            "1" => Hazard::Tornado,
            "2" => Hazard::Funnel,
            "3" => Hazard::WallCloud,
            "4" => Hazard::Hail,
            "5" => Hazard::Wind,
            "6" => Hazard::Flood,
            "7" => Hazard::FlashFlood,
            "8" => Hazard::Other,
            "9" => Hazard::FreezingRain,
            "10" => Hazard::Snow,
            _ => {
                warn!("sn_parser unknown code: {}", code.to_string());
                Hazard::Other
            }
        }
    }

    fn to_hazard_type(&self) -> HazardType {
        match self {
            Hazard::Tornado => HazardType::Tornado,
            Hazard::Funnel => HazardType::Funnel,
            Hazard::WallCloud => HazardType::WallCloud,
            Hazard::Hail => HazardType::Hail,
            Hazard::Wind => HazardType::Wind,
            Hazard::Flood => HazardType::Flood,
            Hazard::FlashFlood => HazardType::Flood,
            Hazard::Other => HazardType::Other,
            Hazard::FreezingRain => HazardType::FreezingRain,
            Hazard::Snow => HazardType::Snow,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Hazard::Tornado => "Tornado",
            Hazard::Funnel => "Funnel",
            Hazard::WallCloud => "Wall Cloud",
            Hazard::Hail => "Hail",
            Hazard::Wind => "Wind",
            Hazard::Flood => "Flood",
            Hazard::FlashFlood => "Flash Flood",
            Hazard::Other => "Other",
            Hazard::FreezingRain => "Freezing Rain",
            Hazard::Snow => "Snow",
        }
        .to_string()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::domain::HazardType;
//     use std::fs::File;
//     use std::io::{BufRead, BufReader};

//     #[test]
//     fn parse_should_skip_empty_other_reports() {
//         let reports_file = File::open("data/reports-other-none").unwrap();
//         let reader = BufReader::new(reports_file);

//         reader
//             .lines()
//             .map(|x| x.unwrap())
//             .filter(|x| x.starts_with("Icon:"))
//             .for_each(|x| {
//                 let message = parse(&x);
//                 assert!(message.unwrap().is_none());
//             });
//     }

//     #[test]
//     fn parse_should_return_an_event_with_all_required_fields() {
//         let report = r#"Icon: 43.112000,-94.639999,000,3,5,"Reported By: Test Human\nHigh Wind\nTime: 2018-09-20 22:52:00 UTC\n60 mph [Measured]\nNotes: Strong winds measured at 60mph with anemometer""#;
//         let event = parse(report).unwrap().unwrap();

//         assert_eq!(
//             event,
//             Event {
//                 event_ts: 1537483920000000,
//                 event_type: EventType::SnReport,
//                 expires_ts: None,
//                 ext_uri: None,
//                 ingest_ts: 0,
//                 location: Some(Location {
//                     county: None,
//                     wfo: None,
//                     point: Some(Coordinates {
//                         lat: 43.112,
//                         lon: -94.64
//                     }),
//                     poly: None
//                 }),
//                 md: None,
//                 outlook: None,
//                 report: Some(Report {
//                     reporter: "Test Human".to_string(),
//                     hazard: HazardType::Wind,
//                     magnitude: Some(60.0),
//                     units: Some(Units::Mph),
//                     was_measured: Some(true),
//                     report_ts: None
//                 }),
//                 text: Some(
//                     "Wind reported by Test Human. Strong winds measured at 60mph with anemometer"
//                         .to_string()
//                 ),
//                 title: "Report: 60mph Wind".to_string(),
//                 valid_ts: None,
//                 warning: None,
//                 watch: None
//             }
//         );
//     }

//     #[test]
//     fn report_should_not_blow_up_with_non_utf8_characters() {
//         let report = r#"Icon: 43.112000,-94.639999,000,3,5,"Reported By: Test Human\nHigh Wind\nTime: 2018-09-20 22:52:00 UTC\n60 mph [Measured]\nNotes: Strong �������������������������������������������������������������������� measured at 60mph with anemometer""#;
//         let event = parse(report);
//         assert!(event.is_ok());
//     }

//     #[test]
//     fn report_should_parse_optional_mph() {
//         let report = r#"Icon: 43.112000,-94.639999,000,3,5,"Reported By: Test Human\nHigh Wind\nTime: 2018-09-20 22:52:00 UTC\n60 mph [Measured]\nNotes: Strong winds measured at 60mph with anemometer""#;
//         let parsed_report = parse(report).unwrap().unwrap().report.unwrap();
//         assert_eq!(parsed_report.magnitude, Some(60.0));
//         assert_eq!(parsed_report.units, Some(Units::Mph));
//     }

//     #[test]
//     fn report_should_parse_optional_measured() {
//         let report = r#"Icon: 43.112000,-94.639999,000,3,5,"Reported By: Test Human\nHigh Wind\nTime: 2018-09-20 22:52:00 UTC\n60 mph [Measured]\nNotes: Strong winds measured at 60mph with anemometer""#;
//         let parsed_report = parse(report).unwrap().unwrap().report.unwrap();
//         assert_eq!(parsed_report.was_measured, Some(true));
//     }

//     #[test]
//     fn report_should_parse_optional_size() {
//         let report = r#"Icon: 47.617706,-111.215248,000,4,4,"Reported By: Test Human\nHail\nTime: 2018-09-20 22:49:29 UTC\nSize: 0.75" (Penny)\nNotes: None""#;
//         let parsed_report = parse(report).unwrap().unwrap().report.unwrap();
//         assert_eq!(parsed_report.magnitude, Some(0.75));
//         assert_eq!(parsed_report.units, Some(Units::Inches));
//     }
// }
