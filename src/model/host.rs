use regex::Regex;
use reqwest::StatusCode;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Host {
    #[serde(rename = "http")]
    HTTP(HTTPHost),
}

#[derive(Deserialize)]
pub struct HTTPHost {
    pub label: String,
    pub url: String,
    #[serde(default, deserialize_with = "deserialize_optional_status_code")]
    pub status: Option<StatusCode>,
    #[serde(default, deserialize_with = "deserialize_optional_regex")]
    pub regex: Option<Regex>,
}

fn deserialize_optional_status_code<'de, D>(deserializer: D) -> Result<Option<StatusCode>, D::Error>
where
    D: Deserializer<'de>,
{
    let code: Option<u16> = Option::deserialize(deserializer)?;
    code.map(|c| StatusCode::from_u16(c).map_err(serde::de::Error::custom))
        .transpose()
}

fn deserialize_optional_regex<'de, D>(deserializer: D) -> Result<Option<Regex>, D::Error>
where
    D: Deserializer<'de>,
{
    let pattern: Option<String> = Option::deserialize(deserializer)?;
    pattern
        .map(|p| Regex::new(&p).map_err(serde::de::Error::custom))
        .transpose()
}
