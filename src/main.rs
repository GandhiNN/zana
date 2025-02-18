use directories::BaseDirs;
use tracing::info;
use tracing_subscriber::fmt as TracingSubscriberFmt;
use zana::aws::config::AWSCredentialsFile;
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
    let mut aws_credentials = AWSCredentialsFile::new(credentials_path);

    // Conditional check for age of the file
    aws_credentials.get_credentials_file_age();
    info!("Config file age: {}", aws_credentials.age);
    if aws_credentials.age.age_duration > time::Duration::hours(6) {
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

    // Handle CLI arguments
    cli::runner::run(aws_credentials).await;
}
