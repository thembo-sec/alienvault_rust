#![allow(nonstandard_style, dead_code)]

use derive_getters::Getters; //easy getting for structs
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Getters)]
pub struct OtxResponse {
    count: u16,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<Pulse>,
}

#[derive(Deserialize, Debug, Getters)]
pub struct Pulse {
    id: String,
    name: String,
    created: String,
    modified: String,
    pulse_source: PulseSource,
    TLP: Tlp,
    adversary: String,
    description: String,
    attack_ids: Vec<AttackID>,
    indicator_count: u8,
    indicator_type_counts: Value,
    malware_families: Vec<Value>,
    industries: Vec<String>,
    targeted_countries: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct AttackID {
    id: String,
    display_name: String,
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PulseSource {
    Web,
    Api,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Tlp {
    White,
    Green,
    Amber,
    Red,
}

#[derive(Deserialize, Debug)]
pub struct Indicator {
    name: IndicatorType,
    description: String,
    api_support: bool,
    sections: Option<Vec<String>>,
    slug: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum IndicatorType {
    IPv4,
    IPv6,
    domain,
    hostname,
    email,
    URL,
    URI,
    #[serde(rename = "FileHash-MD5")]
    FILE_HASH_MD5,
    #[serde(rename = "FileHash-SHA1")]
    FILE_HASH_SHA1,
    #[serde(rename = "FileHash-SHA256")]
    FILE_HASH_SHA256,
    #[serde(rename = "FileHash-PEHASH")]
    FILE_HASH_PEHASH,
    #[serde(rename = "FileHash-IMPHASH")]
    FILE_HASH_IMPHASH,
    CIDR,
    FilePath,
    Mutex,
    CVE,
    YARA,
    JA3,
    osquery,
    SSLCertFingerprint,
    BitcoinAddress,
}
