use tracing::info;
use tracing_subscriber::fmt as TracingSubscriberFmt;
use zana::cli;

#[tokio::main]
async fn main() {
    // Setup tracing
    TracingSubscriberFmt::init();
    info!("Running the program");

    // Handle CLI arguments
    cli::runner::run().await;
}
