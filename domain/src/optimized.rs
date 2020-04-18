use crate::{
    Coordinates, EventType, HazardType, MdConcerning, OutlookRisk, SwoType, Units, WatchStatus,
    WatchType,
};
use std::collections::HashMap;

/**
 * Domain objects that do not deserialize null fields for transport over the wire
 */
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct OptimizedEvent {
    pub event_ts: u64,
    pub event_type: EventType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_ts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_uri: Option<String>,
    pub ingest_ts: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md: Option<MesoscaleDiscussion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlook: Option<Outlook>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report: Option<Report>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_ts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<Warning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch: Option<Watch>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<Coordinates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poly: Option<Vec<Coordinates>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Report {
    pub reporter: String,
    pub hazard: HazardType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnitude: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<Units>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_measured: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_ts: Option<u64>, // only populated for LSRs
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Watch {
    pub is_pds: bool,
    pub id: u16,
    pub watch_type: WatchType,
    pub status: WatchStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_for: Option<String>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Warning {
    pub is_pds: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tor_emergency: Option<bool>, // TOR only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_observed: Option<bool>, // TOR only
    pub issued_for: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_deg: Option<u16>, // TOR and SVR only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_kt: Option<u16>, // TOR and SVR only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>, // TOR and SVR only
    pub time: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Outlook {
    pub swo_type: SwoType,
    pub max_risk: OutlookRisk,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polys: Option<HashMap<OutlookRisk, Vec<Coordinates>>>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MesoscaleDiscussion {
    pub id: u16,
    pub affected: String,
    pub concerning: MdConcerning,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watch_issuance_probability: Option<u16>,
    pub wfos: Vec<String>,
}
