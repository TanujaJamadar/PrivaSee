use crate::geo::get_real_geo;
use crate::types::{AppState, RawTraffic, TrafficUpdate, Violation};
use regex::Regex;
use std::sync::Arc;
use url::Url;

pub fn analyze_traffic(req: &RawTraffic, main_domain: &str, state: &Arc<AppState>) -> TrafficUpdate {
    let req_domain = extract_domain(&req.url);
    let mut violations = Vec::new();
    let is_tracker_detected = is_tracker(&req.url);

    if req.url.starts_with("http://") {
        violations.push(Violation {
            issue: "Unencrypted (HTTP)".to_string(),
            severity: "high".to_string(),
        });
    }

    if is_tracker_detected {
        if !req_domain.contains(main_domain) {
            violations.push(Violation {
                issue: "3rd Party Tracker".to_string(),
                severity: "medium".to_string(),
            });
        } else {
            violations.push(Violation {
                issue: "Hidden 1st Party Tracker".to_string(),
                severity: "medium".to_string(),
            });
        }
    }

    if has_pii(&req.url) {
        violations.push(Violation {
            issue: "PII (Email) Leak".to_string(),
            severity: "critical".to_string(),
        });
    }

    let geo = get_real_geo(&req.url, &state.geo_reader);

    TrafficUpdate {
        url: req.url.clone(),
        method: req.method.clone(),
        type_: req.resource_type.clone(),
        domain: req_domain,
        violations,
        geo,
        is_tracker: is_tracker_detected,
    }
}

pub fn extract_domain(url_str: &str) -> String {
    Url::parse(url_str)
        .ok()
        .and_then(|u| u.host_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "unknown".to_string())
}

fn is_tracker(url: &str) -> bool {
    let keywords = [
        "analytics", "pixel", "tracker", "telemetry", "collect",
        "facebook", "tiktok", "adsystem", "googleads", "doubleclick",
        "clarity", "hotjar", "fbevents", "measure", "beacon",
    ];
    let lower = url.to_lowercase();
    keywords.iter().any(|&k| lower.contains(k))
}

fn has_pii(text: &str) -> bool {
    let re = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
    let decoded = urlencoding::decode(text).unwrap_or(std::borrow::Cow::Borrowed(text));
    re.is_match(&decoded)
}
