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
