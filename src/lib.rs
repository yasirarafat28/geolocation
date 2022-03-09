//! # geolocation
//! Get geolocation information of an IP. Its simple.
//!
//! ```
//! geolocation = "0.2.1"
//! ```
//! Add to `Cargo.toml`.
//!
//! ## Example
//!
//! Using `geolocation` is really quite easy:
//! ```
//! use geolocation;
//!
//! fn main() {
//!     let ip = "<Put your IP address Here>";
//!     let info = geolocation::find(ip).unwrap();
//!
//!     println!("{:?}", info.city);
//! }
//! ```
//!
//! This and more examples are found in the examples directory.
//!
//! ## Query Limits
//! You can send 45 requests per minute.
//!
//! ## Fields
//! The API can get these fields about IP addresses.
//!
//! - [`ip`](crate::Locator::ip)
//! - [`latitude`](crate::Locator::latitude)
//! - [`longitude`](crate::Locator::longitude)
//! - [`city`](crate::Locator::city)
//! - [`region`](crate::Locator::region)
//! - [`country`](crate::Locator::country)
//! - [`timezone`](crate::Locator::timezone)
//! - [`location`](crate::Locator::location)
//!
//! Written with love, in Rust.

use isahc::prelude::*;
use serde_json::Value;
pub struct Locator {
    pub ip: String,
    pub latitude: String,
    pub longitude: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub timezone: String,
    pub location: String,
}

pub fn find(ip: &str) -> Result<Locator, isahc::Error> {
    let uri = format!("http://ip-api.com/json/{}", &ip);
    let mut local_data_response = isahc::get(uri)?;

    let local_data = local_data_response.text().unwrap();
    let local_body: Value = serde_json::from_str(&local_data).unwrap();
    let result = Locator {
        ip: local_body["query"].to_string(),
        latitude: local_body["lat"].to_string(),
        longitude: local_body["lon"].to_string(),
        city: local_body["city"].to_string(),
        region: local_body["regionName"].to_string(),
        country: local_body["country"].to_string(),
        timezone: local_body["timezone"].to_string(),
        location: format!(
            "{:?}, {:?}, {:?}",
            local_body["city"].to_string(),
            local_body["regionName"].to_string(),
            local_body["country"].to_string()
        ),
    };

    Ok(result)
}
