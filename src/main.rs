use chrono::prelude::*;
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_headers = json!({
        "X-OTX-API-KEY": "77b957820d05feadb7eff41a2fe124f3c832b76bb307724a9f2391f1e33c6e29",
        "User-Agent": "OTX RUST/0.0.1",
        "Content-Type": "application/json"
    });

    let request_url =
        "https://otx.alienvault.com/api/v1/pulses/user/AlienVault?2023-01-01T12:35:00+00:00";
    let response = Client::new()
        .get(request_url)
        .header(
            "X-OTX-API-KEY",
            "77b957820d05feadb7eff41a2fe124f3c832b76bb307724a9f2391f1e33c6e29",
        )
        .send()
        .await?;

    let status = response.status().clone();
    let headers = response.headers().clone();

    let response_body = response.text().await?;
    let v: Value = serde_json::from_str(&response_body).unwrap();

    print!(
        "Status: {:?}\nHeaders: {:#?}\nBody:{:#?}",
        status, headers, v
    );

    Ok(())
}

#[derive(Deserialize, Debug)]
struct Pulse {
    id: String,
    name: String,
    created: String,
    modified: String,
    pulse_source: PulseSource,
    tlp: TLP,
    description: String,
}

#[derive(Deserialize, Debug)]
enum PulseSource {
    web,
    api,
}

#[derive(Deserialize, Debug)]
enum TLP {
    white,
    green,
    amber,
    red,
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
