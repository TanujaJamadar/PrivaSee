use serde::{Deserialize, Serialize};

pub struct AppState {
    pub geo_reader: maxminddb::Reader<Vec<u8>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum NodeMessage {
    #[serde(rename = "status")]
    Status { message: String },
    #[serde(rename = "error")]
    Error { message: String },
    #[serde(rename = "traffic")]
    Traffic { data: RawTraffic },
}

#[derive(Debug, Deserialize, Clone)]
pub struct RawTraffic {
    pub url: String,
    pub method: String,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "postData")]
    pub post_data: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Violation {
    pub issue: String,
    pub severity: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct GeoData {
    pub lat: f64,
    pub lon: f64,
    pub country: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct TrafficUpdate {
    pub url: String,
    pub method: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub domain: String,
    pub violations: Vec<Violation>,
    pub geo: GeoData,
    #[serde(rename = "isTracker")]
    pub is_tracker: bool,
}
