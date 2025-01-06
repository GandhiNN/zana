use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use zana::aws::config::AWSConfigFile;
use zana::cli;

const PATH: &str = "config.ini";

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

    // Load Configuration file
    let aws_config_file = AWSConfigFile::from_file(PATH);

    // Handle CLI arguments
    cli::run(aws_config_file).await;
}
