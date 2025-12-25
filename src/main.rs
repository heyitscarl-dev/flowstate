use tracing::info;

use crate::{config::{Host, load_config}, error::Fallible};

pub mod error;
pub mod config;

fn main() -> Fallible {
    tracing_subscriber::fmt::init();
    
    let config = load_config()?;
    let count = config.hosts.len();
    info!("Found {} host{}.", count, if count == 1 { "" } else { "s" });

    for host in config.hosts {
        let ok = monitor(&host);
        info!("Host '{}' is {}.", host.label, if ok { "up" } else { "down" })
    }

    Ok(())
}

fn monitor(host: &Host) -> bool {
    reqwest::blocking::get(&host.on)
        .is_ok()
}
