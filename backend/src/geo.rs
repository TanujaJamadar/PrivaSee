use crate::types::GeoData;
use dns_lookup::lookup_host;
use maxminddb::geoip2;
use url::Url;

pub fn get_real_geo(url_str: &str, reader: &maxminddb::Reader<Vec<u8>>) -> GeoData {
    let default = GeoData {
        lat: 20.5937,
        lon: 78.9629,
        country: "Unknown".to_string(),
    };

    let domain = match Url::parse(url_str)
        .ok()
        .and_then(|u| u.host_str().map(|s| s.to_string()))
    {
        Some(d) => d,
        None => return default,
    };

    let ips = match lookup_host(&domain) {
        Ok(ips) => ips,
        Err(_) => return default,
    };

    if let Some(ip) = ips.first() {
        match reader.lookup::<geoip2::City>(*ip) {
            Ok(city) => {
                let lat = city.location.clone().and_then(|l| l.latitude).unwrap_or(0.0);
                let lon = city.location.and_then(|l| l.longitude).unwrap_or(0.0);
                let country = city
                    .country
                    .and_then(|c| c.names)
                    .and_then(|n| n.get("en").map(|s| s.to_string()))
                    .unwrap_or("Unknown".to_string());

                return GeoData { lat, lon, country };
            }
            Err(_) => return default,
        }
    }

    default
}
