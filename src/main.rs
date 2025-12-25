use tracing::info;
use flowstate::{config::{Host, load_config}, error::Fallible};

fn main() -> Fallible {
    tracing_subscriber::fmt::init();

    let config = load_config()?;
    let count = config.hosts.len();
    info!("Found {} host{}.", count, plural_suffix(count));

    for host in config.hosts {
        let ok = monitor(&host);
        info!("Host '{}' is {}.", host.label, if ok { "up" } else { "down" })
    }

    Ok(())
}

fn plural_suffix(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

fn monitor(host: &Host) -> bool {
    reqwest::blocking::get(&host.on)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pluralization_logic() {
        assert_eq!(plural_suffix(0), "s");
        assert_eq!(plural_suffix(1), "");
        assert_eq!(plural_suffix(2), "s");
        assert_eq!(plural_suffix(100), "s");
    }
}
