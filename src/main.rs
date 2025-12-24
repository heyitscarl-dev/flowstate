use tracing::info;

pub mod error;

const VERSION: &str = "0";

fn main() {
    tracing_subscriber::fmt::init();

    info!(
        VERSION,
        "hello, world"
    )
}
