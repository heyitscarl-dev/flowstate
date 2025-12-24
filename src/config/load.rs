use std::env;
use std::fs;

use crate::config::model;
use crate::error::{Error, Result};

const CONFIG_ENVIRONMENT: &str = "FLOW_CONFIG";
const CONFIG_LOCATION: &str = "Flow.yaml";

pub fn load_config() -> Result<model::Configuration> {
    let location: String = env::var(CONFIG_ENVIRONMENT).ok()
        .unwrap_or(CONFIG_LOCATION.to_string());
    
    let content = fs::read_to_string(location)
        .map_err(|e| Error::ConfigLoad(e))?;

    serde_yaml::from_str(&content)
        .map_err(|e| Error::ConfigParse(e))
}
