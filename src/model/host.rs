use regex::Regex;
use reqwest::StatusCode;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Host {
    #[serde(rename = "http")]
    HTTP(HTTPHost),
}

#[derive(Debug, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_deserialize_basic_http_host() {
        let yaml = r#"
type: http
label: Test Host
url: http://example.com
"#;
        let host: Host = serde_yaml::from_str(yaml).unwrap();
        match host {
            Host::HTTP(http_host) => {
                assert_eq!(http_host.label, "Test Host");
                assert_eq!(http_host.url, "http://example.com");
                assert!(http_host.status.is_none());
                assert!(http_host.regex.is_none());
            }
        }
    }

    #[test]
    fn test_deserialize_http_host_with_status() {
        let yaml = r#"
type: http
label: Test Host
url: http://example.com
status: 200
"#;
        let host: Host = serde_yaml::from_str(yaml).unwrap();
        match host {
            Host::HTTP(http_host) => {
                assert_eq!(http_host.label, "Test Host");
                assert_eq!(http_host.url, "http://example.com");
                assert_eq!(http_host.status.unwrap().as_u16(), 200);
                assert!(http_host.regex.is_none());
            }
        }
    }

    #[test]
    fn test_deserialize_http_host_with_regex() {
        let yaml = r#"
type: http
label: Test Host
url: http://example.com
regex: "^success.*"
"#;
        let host: Host = serde_yaml::from_str(yaml).unwrap();
        match host {
            Host::HTTP(http_host) => {
                assert_eq!(http_host.label, "Test Host");
                assert_eq!(http_host.url, "http://example.com");
                assert!(http_host.status.is_none());
                let regex = http_host.regex.unwrap();
                assert!(regex.is_match("success message"));
                assert!(!regex.is_match("failure message"));
            }
        }
    }

    #[test]
    fn test_deserialize_http_host_with_status_and_regex() {
        let yaml = r#"
type: http
label: Test Host
url: http://example.com
status: 201
regex: "created"
"#;
        let host: Host = serde_yaml::from_str(yaml).unwrap();
        match host {
            Host::HTTP(http_host) => {
                assert_eq!(http_host.label, "Test Host");
                assert_eq!(http_host.url, "http://example.com");
                assert_eq!(http_host.status.unwrap().as_u16(), 201);
                assert!(http_host.regex.is_some());
            }
        }
    }

    #[test]
    fn test_deserialize_invalid_status_code() {
        let yaml = r#"
type: http
label: Test Host
url: http://example.com
status: 1000
"#;
        let result: Result<Host, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_invalid_regex() {
        let yaml = r#"
type: http
label: Test Host
url: http://example.com
regex: "[invalid(regex"
"#;
        let result: Result<Host, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_missing_required_label() {
        let yaml = r#"
type: http
url: http://example.com
"#;
        let result: Result<Host, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_missing_required_url() {
        let yaml = r#"
type: http
label: Test Host
"#;
        let result: Result<Host, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_various_status_codes() {
        let test_cases = vec![200, 201, 204, 301, 302, 400, 404, 500, 503];

        for status_code in test_cases {
            let yaml = format!(
                r#"
type: http
label: Test Host
url: http://example.com
status: {}
"#,
                status_code
            );
            let host: Host = serde_yaml::from_str(&yaml).unwrap();
            match host {
                Host::HTTP(http_host) => {
                    assert_eq!(http_host.status.unwrap().as_u16(), status_code);
                }
            }
        }
    }

    #[test]
    fn test_deserialize_complex_regex_patterns() {
        let test_cases = vec![
            ("^[0-9]{3}$", "123", true),
            ("^[0-9]{3}$", "12", false),
            ("success|ok", "success", true),
            ("success|ok", "ok", true),
            ("success|ok", "fail", false),
        ];

        for (pattern, test_string, should_match) in test_cases {
            let yaml = format!(
                r#"
type: http
label: Test Host
url: http://example.com
regex: "{}"
"#,
                pattern
            );
            let host: Host = serde_yaml::from_str(&yaml).unwrap();
            match host {
                Host::HTTP(http_host) => {
                    let regex = http_host.regex.unwrap();
                    assert_eq!(regex.is_match(test_string), should_match);
                }
            }
        }
    }
}
