use directories::BaseDirs;
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::fmt as TracingSubscriberFmt;
use zana::cli;

const DEFAULT_CREDENTIALS_PATH: &str = ".aws_sso/credentials";

#[tokio::main]
async fn main() {
    // Setup tracing
    TracingSubscriberFmt::init();
    info!("Running the program");

    // Load credentials file
    let base_dirs = BaseDirs::new().unwrap();
    let home_dir = base_dirs.home_dir().to_string_lossy().to_string();
    let default_credentials_path = format!("{}/{}", home_dir, DEFAULT_CREDENTIALS_PATH);
    let credentials_path = std::env::var("CREDENTIALS_PATH").unwrap_or(default_credentials_path);

    // Quit program if credentials file does not exist
    if !PathBuf::from(&credentials_path).exists() {
        info!(
            "Credentials file {} does not exist. Exiting program",
            &credentials_path
        );
        std::process::exit(1);
    }

    // Handle CLI arguments
    cli::runner::run().await;
}
