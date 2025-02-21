use supabase_rs::tests::methods::init;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tokio::runtime::Builder;
use tokio::task;

use aurora::api::server::api;

#[tokio::main]
async fn main() {
    init_tracing();
    println!("Hello, world!");

    if let Err(e) = api().await {
        eprintln!("Failed to start server: {}", e);
    }
}

/// ## Initialize Tracing
///
/// This function sets up the tracing subscriber for logging and monitoring.
///
/// ### Example
///
/// ```
/// init_tracing();
/// ```
fn init_tracing() {
    let filter: EnvFilter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt().with_env_filter(filter).init()
}
