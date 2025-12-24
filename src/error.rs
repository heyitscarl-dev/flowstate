use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Could not load config: {0}")]
    ConfigLoad(std::io::Error),

    #[error("Could not parse config: {0}")]
    ConfigParse(serde_yaml::Error)
}

pub type Result<T> = std::result::Result<T, Error>;
pub type Fallible = Result<()>;
