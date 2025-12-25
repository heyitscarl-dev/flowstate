use thiserror::Error;

use crate::model::host::HTTPHost;

pub type HTTPResult = std::result::Result<(), HTTPError>;

#[derive(Debug, Error)]
pub enum HTTPError {
    #[error("Host is unresponsive.")]
    Unresponsive,
    #[error("Host responded with invalid status.")]
    InvalidStatus,
    #[error("Host responded with invalid body.")]
    InvalidBody,
    #[error("Host responded with invalid headers.")]
    InvalidHeaders,
}

pub async fn monitor_host(host: &HTTPHost) -> HTTPResult {
    // 1. Host responds

    let response = reqwest::get(&host.url).await
        .map_err(|_| HTTPError::Unresponsive)?;

    // 2. ~ with specified Status code

    if host.status.is_some() && response.status() != host.status.unwrap() {
        return Err(HTTPError::InvalidStatus);
    }

    // 3. ~ with specified body (regex)

    let text = response.text().await
        .map_err(|_| HTTPError::InvalidBody)?;

    if host.regex.is_some() && !host.regex.clone().unwrap().is_match(&text) {
        return Err(HTTPError::InvalidBody);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use reqwest::StatusCode;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::method;

    #[tokio::test]
    async fn test_monitor_host_successful() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string("OK"))
            .mount(&mock_server)
            .await;

        let host = HTTPHost {
            label: "Test Host".to_string(),
            url: mock_server.uri(),
            status: None,
            regex: None,
        };

        let result = monitor_host(&host).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_monitor_host_with_status_check_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string("OK"))
            .mount(&mock_server)
            .await;

        let host = HTTPHost {
            label: "Test Host".to_string(),
            url: mock_server.uri(),
            status: Some(StatusCode::from_u16(200).unwrap()),
            regex: None,
        };

        let result = monitor_host(&host).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_monitor_host_with_status_check_failure() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
            .mount(&mock_server)
            .await;

        let host = HTTPHost {
            label: "Test Host".to_string(),
            url: mock_server.uri(),
            status: Some(StatusCode::from_u16(200).unwrap()),
            regex: None,
        };

        let result = monitor_host(&host).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HTTPError::InvalidStatus));
    }

    #[tokio::test]
    async fn test_monitor_host_with_regex_check_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string("success message"))
            .mount(&mock_server)
            .await;

        let host = HTTPHost {
            label: "Test Host".to_string(),
            url: mock_server.uri(),
            status: None,
            regex: Some(Regex::new("success").unwrap()),
        };

        let result = monitor_host(&host).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_monitor_host_with_regex_check_failure() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string("error message"))
            .mount(&mock_server)
            .await;

        let host = HTTPHost {
            label: "Test Host".to_string(),
            url: mock_server.uri(),
            status: None,
            regex: Some(Regex::new("success").unwrap()),
        };

        let result = monitor_host(&host).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HTTPError::InvalidBody));
    }

    #[tokio::test]
    async fn test_monitor_host_with_both_checks_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(201).set_body_string("created successfully"))
            .mount(&mock_server)
            .await;

        let host = HTTPHost {
            label: "Test Host".to_string(),
            url: mock_server.uri(),
            status: Some(StatusCode::from_u16(201).unwrap()),
            regex: Some(Regex::new("created").unwrap()),
        };

        let result = monitor_host(&host).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_monitor_host_with_both_checks_status_fails() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string("created successfully"))
            .mount(&mock_server)
            .await;

        let host = HTTPHost {
            label: "Test Host".to_string(),
            url: mock_server.uri(),
            status: Some(StatusCode::from_u16(201).unwrap()),
            regex: Some(Regex::new("created").unwrap()),
        };

        let result = monitor_host(&host).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HTTPError::InvalidStatus));
    }

    #[tokio::test]
    async fn test_monitor_host_with_both_checks_regex_fails() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(201).set_body_string("error message"))
            .mount(&mock_server)
            .await;

        let host = HTTPHost {
            label: "Test Host".to_string(),
            url: mock_server.uri(),
            status: Some(StatusCode::from_u16(201).unwrap()),
            regex: Some(Regex::new("created").unwrap()),
        };

        let result = monitor_host(&host).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HTTPError::InvalidBody));
    }

    #[tokio::test]
    async fn test_monitor_host_unresponsive() {
        let host = HTTPHost {
            label: "Test Host".to_string(),
            url: "http://localhost:99999".to_string(), // Invalid port
            status: None,
            regex: None,
        };

        let result = monitor_host(&host).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HTTPError::Unresponsive));
    }

    #[tokio::test]
    async fn test_monitor_host_complex_regex() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"status":"ok","code":123}"#))
            .mount(&mock_server)
            .await;

        let host = HTTPHost {
            label: "Test Host".to_string(),
            url: mock_server.uri(),
            status: None,
            regex: Some(Regex::new(r#""status":"ok""#).unwrap()),
        };

        let result = monitor_host(&host).await;
        assert!(result.is_ok());
    }
}
