#![allow(nonstandard_style)]
use dotenv::dotenv;
use reqwest::{
    header::{self, HeaderMap},
    Client, Error,
};
use serde_json;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    // TODO turn this into potentially a query builder
    let request_url =
        "https://otx.alienvault.com/api/v1/pulses/user/AlienVault?2023-01-01T12:35:00+00:00";

    let client = Client::builder()
        .default_headers(build_header())
        .user_agent(USER_AGENT)
        .build()?;

    let response = client.get(request_url).send().await?;

    let status = response.status().clone();
    let headers = response.headers().clone();

    let response_body = response.text().await?;

    let r: alienvault_rust::OtxResponse = serde_json::from_str(&response_body).unwrap();

    println!("Status: {:?}\nHeaders: {:#?}\n", status, headers);

    println!("Serialized: {:#?}", r);

    Ok(())
}

/// Builds default headers for the client based on the API token
fn build_header() -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    let ALIENVAULT_API_TOKEN = std::env::var("ALIENVAULT_APIKEY").expect("No token found");
    headers.insert("X-OTX-API-KEY", ALIENVAULT_API_TOKEN.parse().unwrap());
    headers
}
