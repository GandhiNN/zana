use chrono::Local;
use directories::BaseDirs;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use zana::aws::config::AWSConfigFile;
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

    // Load Configuration file
    let base_dirs = BaseDirs::new().unwrap();
    let home_dir = base_dirs.home_dir().to_string_lossy().to_string();
    let default_path = format!("{}/.aws/credentials", home_dir);
    let config_path = std::env::var("CONFIG_PATH").unwrap_or(default_path);
    let aws_config_file = AWSConfigFile::new(config_path);

    // Handle CLI arguments
    cli::run(aws_config_file).await;
}
