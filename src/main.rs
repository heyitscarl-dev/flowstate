use tracing::info;

use crate::{config::load_config, error::Fallible};

pub mod error;
pub mod config;

fn main() -> Fallible {
    tracing_subscriber::fmt::init();
    
    let config = load_config()?;
    let count = config.hosts.len();
    info!("Found {} host{}.", count, if count == 1 { "" } else { "s" });

    Ok(())
}
