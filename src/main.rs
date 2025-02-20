use anyhow::Result;
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::fmt as TracingSubscriberFmt;
use zana::aws::credentials::load_credentials_file;
use zana::cli;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup tracing
    TracingSubscriberFmt::init();
    info!("Running the program");

    // Credentials validity check
    let credentials = load_credentials_file();

    // Handle CLI arguments
    cli::runner::run(PathBuf::from(credentials)).await;
    Ok(())
}
