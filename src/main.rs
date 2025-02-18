use directories::BaseDirs;
use tracing::info;
use tracing_subscriber::fmt as TracingSubscriberFmt;
use zana::aws::config::{get_config_file_age, AWSCredentialsFile};
use zana::cli;

#[tokio::main]
async fn main() {
    // Setup tracing
    TracingSubscriberFmt::init();
    info!("Running the program");

    // Load Configuration file
    let base_dirs = BaseDirs::new().unwrap();
    let home_dir = base_dirs.home_dir().to_string_lossy().to_string();
    let default_path = format!("{}/.aws/credentials", home_dir);
    let config_path = std::env::var("CONFIG_PATH").unwrap_or(default_path);

    // Conditional check for age of the file
    let config_file_age = get_config_file_age(&config_path);
    let age_seconds = &config_file_age.as_secs();
    let age_hours = age_seconds / 3600;
    let age_minutes = age_seconds % 3600 / 60;
    let age_seconds = age_seconds % 3600 % 60;
    info!(
        "Config file age: {}h {}m {}s",
        age_hours, age_minutes, age_seconds
    );
    if config_file_age > time::Duration::hours(6) {
        info!("Config file is older than 6 hours");
        println!("The program is not guaranteed to run with an outdated config file");
        println!("Would you like to continue? (y/n)");
        let mut input_string = String::new();
        while input_string.trim() != "y" && input_string.trim() != "n" {
            std::io::stdin().read_line(&mut input_string).unwrap();
            println!("Would you like to continue? (y/n)");
        }
        if input_string.trim() == "n" {
            println!("Exiting the program!");
            std::process::exit(1);
        } else {
            println!("Continuing with outdated config file");
        }
    }

    // Load config file
    let aws_config = AWSCredentialsFile::new(config_path);

    // Handle CLI arguments
    cli::runner::run(aws_config).await;
}
