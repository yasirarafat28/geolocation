

use serde_json::Value;
// use std::fmt;
// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use isahc::prelude::*;
pub struct Locator {
    pub ip: String,
    pub latitude: String,
    pub longitude: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub timezone: String,
    pub location:String
}

fn find(ip: Option<&str>)-> Result<Locator, isahc::Error>{
    // let ip: &str = "103.35.168.86";

    let uri = format!("http://ip-api.com/json/{:?}",&ip);
    // https://ipapi.co/103.35.168.86/json/

    println!("URI : {}",uri);
    let mut local_data_response = isahc::get(uri)?;

    let local_data = local_data_response.text().unwrap();
    let local_body:Value = serde_json::from_str(&local_data).unwrap() ;
    println!("{}",&local_body["city"].to_string());

    let result = Locator {

            ip:"".to_string(),
            latitude:"".to_string(),
            longitude:"".to_string(),
            city:"".to_string(),
            region:"".to_string(),
            country:"".to_string(),
            timezone:"".to_string(),
            location:"".to_string(),
    };

    Ok(result)
}