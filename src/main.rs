use directories::BaseDirs;
use tracing::info;
use tracing_subscriber::fmt as TracingSubscriberFmt;
// use zana::aws::credentials::AWSCredentialsFile;
use zana::cli;

#[tokio::main]
async fn main() {
    // Setup tracing
    TracingSubscriberFmt::init();
    info!("Running the program");

    // Load credentials file
    let base_dirs = BaseDirs::new().unwrap();
    let home_dir = base_dirs.home_dir().to_string_lossy().to_string();
    let default_credentials_path = format!("{}/.aws/credentials", home_dir);
    let credentials_path = std::env::var("CREDENTIALS_PATH").unwrap_or(default_credentials_path);

    // Handle CLI arguments
    cli::runner::run(credentials_path.as_str()).await;
}
