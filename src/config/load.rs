use std::env;
use std::fs;

use serde::Deserialize;

use crate::error::{Error, Result};
use crate::model;

const CONFIG_ENVIRONMENT: &str = "FLOW_CONFIG";
const CONFIG_LOCATION: &str = "Flow.yaml";

#[derive(Deserialize)]
pub struct Configuration {
    pub hosts: Vec<model::host::Host>
}

pub fn load_config() -> Result<Configuration> {
    let location: String = env::var(CONFIG_ENVIRONMENT).ok()
        .unwrap_or(CONFIG_LOCATION.to_string());
    
    let content = fs::read_to_string(location)
        .map_err(|e| Error::ConfigLoad(e))?;

    serde_yaml::from_str(&content)
        .map_err(|e| Error::ConfigParse(e))
}
