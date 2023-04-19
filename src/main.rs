use chrono::prelude::*;
use dotenv::dotenv;
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{env, iter::Map};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_BACKTRACE", "1");
    dotenv().ok();

    let ALIENVAULT_API_TOKEN = std::env::var("ALIENVAULT_APIKEY").expect("No token found");

    let request_url =
        "https://otx.alienvault.com/api/v1/pulses/user/AlienVault?2023-01-01T12:35:00+00:00";

    let response = Client::new()
        .get(request_url)
        .header("X-OTX-API-KEY", ALIENVAULT_API_TOKEN)
        .send()
        .await?;

    let status = response.status().clone();
    let headers = response.headers().clone();

    let response_body = response.text().await?;

    let v: Value = serde_json::from_str(&response_body).unwrap();

    let r: OtxResponse = serde_json::from_str(&response_body).unwrap();

    println!(
        "Status: {:?}\nHeaders: {:#?}\nBody: {:#?}",
        status, headers, v
    );

    println!("Serialized: {:#?}", r);

    Ok(())
}

#[derive(Deserialize, Debug)]
struct OtxResponse {
    count: u16,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<Pulse>,
}

#[derive(Deserialize, Debug)]
struct Pulse {
    id: String,
    name: String,
    created: String,
    modified: String,
    pulse_source: PulseSource,
    TLP: Tlp,
    adversary: String,
    description: String,
    attack_ids: Vec<AttackID>,
}

#[derive(Deserialize, Debug)]
struct AttackID {
    id: String,
    display_name: String,
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum PulseSource {
    Web,
    Api,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Tlp {
    White,
    Green,
    Amber,
    Red,
}

#[derive(Deserialize, Debug)]
struct Indicator {
    name: IndicatorType,
    description: String,
    api_support: bool,
    sections: Option<Vec<String>>,
    slug: Option<String>,
}

#[derive(Deserialize, Debug)]
enum IndicatorType {
    IPv4,
    IPv6,
    DOMAIN,
    HOSTNAME,
    EMAIL,
    URL,
    URI,
    FILE_HASH_MD5,
    FILE_HASH_SHA1,
    FILE_HASH_SHA256,
    FILE_HASH_PEHASH,
    FILE_HASH_IMPHASH,
    CIDR,
    FILE_PATH,
    MUTEX,
    CVE,
}
