#![allow(nonstandard_style)]
use dotenv::dotenv;
use reqwest::{Client, Error};
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let ALIENVAULT_API_TOKEN = std::env::var("ALIENVAULT_APIKEY").expect("No token found");

    // TODO turn this into potentially a query builder
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

    let r: alienvault_rust::OtxResponse = serde_json::from_str(&response_body).unwrap();

    println!("Status: {:?}\nHeaders: {:#?}\n", status, headers);

    println!("Serialized: {:#?}", r);

    Ok(())
}
