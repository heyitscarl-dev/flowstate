use std::env;
use std::fs;

use serde::Deserialize;

use crate::error::{Error, Result};
use crate::model;

const CONFIG_ENVIRONMENT: &str = "FLOW_CONFIG";
const CONFIG_LOCATION: &str = "Flow.yaml";

#[derive(Debug, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_config() {
        let yaml_content = r#"
hosts:
  - type: http
    label: Test Host
    url: http://test.com
"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        unsafe {
            env::set_var(CONFIG_ENVIRONMENT, temp_file.path());
        }

        let config = load_config().expect("Should load valid config");
        assert_eq!(config.hosts.len(), 1);
    }

    #[test]
    fn test_load_config_with_multiple_hosts() {
        let yaml_content = r#"
hosts:
  - type: http
    label: Host 1
    url: http://host1.com
  - type: http
    label: Host 2
    url: http://host2.com
    status: 200
  - type: http
    label: Host 3
    url: http://host3.com
    regex: "success"
"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        unsafe {
            env::set_var(CONFIG_ENVIRONMENT, temp_file.path());
        }

        let config = load_config().expect("Should load config with multiple hosts");
        assert_eq!(config.hosts.len(), 3);
    }

    #[test]
    fn test_load_config_empty_hosts() {
        let yaml_content = r#"
hosts: []
"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        unsafe {
            env::set_var(CONFIG_ENVIRONMENT, temp_file.path());
        }

        let config = load_config().expect("Should load config with empty hosts");
        assert_eq!(config.hosts.len(), 0);
    }

    #[test]
    fn test_load_config_invalid_yaml() {
        let yaml_content = "not valid: yaml: content: [";
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        unsafe {
            env::set_var(CONFIG_ENVIRONMENT, temp_file.path());
        }

        let result = load_config();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::ConfigParse(_)));
    }

    #[test]
    fn test_load_config_missing_file() {
        unsafe {
            env::set_var(CONFIG_ENVIRONMENT, "/nonexistent/path/to/config.yaml");
        }

        let result = load_config();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::ConfigLoad(_)));
    }

    #[test]
    fn test_load_config_missing_required_fields() {
        let yaml_content = r#"
hosts:
  - type: http
    label: Missing URL
"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(yaml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        unsafe {
            env::set_var(CONFIG_ENVIRONMENT, temp_file.path());
        }

        let result = load_config();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::ConfigParse(_)));
    }
}
