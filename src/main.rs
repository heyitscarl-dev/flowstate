use tracing::info;
use flowstate::{config::load_config, error::Fallible, model::host::Host, services::monitor};

#[tokio::main]
async fn main() -> Fallible {
    tracing_subscriber::fmt::init();

    let config = load_config()?;
    let count = config.hosts.len();
    info!("Found {} host{}.", count, plural_suffix(count));

    for host in config.hosts {
        match host {
            Host::HTTP(host) => {
                let result = monitor::monitor_host(&host).await;
                info!("Host {:?}'a status: {:?}", &host.label, result);
            },
        };
    }

    Ok(())
}

fn plural_suffix(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}
