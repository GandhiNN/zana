use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use zana::cli;

#[tokio::main]
async fn main() {
    // Setup logger
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] = {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    // Handle CLI arguments
    cli::run().await;
}
