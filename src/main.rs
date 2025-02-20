use anyhow::Result;
use directories::BaseDirs;
use inquire::Select;
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::fmt as TracingSubscriberFmt;
use zana::cli;
use zana::utils::fileutil::get_file_age;

const DEFAULT_CREDENTIALS_PATH: &str = ".aws_sso/credentials";

fn load_credentials_file() -> String {
    // Load credentials file
    let base_dirs = BaseDirs::new().unwrap();
    let home_dir = base_dirs.home_dir().to_string_lossy().to_string();
    let default_credentials_path = format!("{}/{}", home_dir, DEFAULT_CREDENTIALS_PATH);
    std::env::var("CREDENTIALS_PATH").unwrap_or(default_credentials_path)
}

fn check_credentials_validity(path: &str) -> Result<bool> {
    // Quit program if credentials file does not exist
    if !PathBuf::from(path).exists() {
        info!("Credentials file {} does not exist", &path);
        let is_configure = Select::new(
            "Would you like to configure it? [yes/no]",
            vec!["yes", "no"],
        )
        .prompt()?;
        if is_configure == "no" {
            info!("Exiting the program");
            std::process::exit(1);
        }
    } else {
        let credentials_age = get_file_age(PathBuf::from(&path));
        info!("Credentials file age: {}", credentials_age);
        if credentials_age.age_duration > time::Duration::hours(6) {
            info!("Credentials are older than 6 hours");
            info!("The program is not guaranteed to run with an outdated credentials");
            let is_continue =
                Select::new("Would you like to continue? [yes/no]", vec!["yes", "no"]).prompt()?;
            if is_continue == "no" {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup tracing
    TracingSubscriberFmt::init();
    info!("Running the program");

    // Credentials validity check
    let credentials = load_credentials_file();
    let cred_check = check_credentials_validity(&credentials)?;
    println!("{}", cred_check);

    // // Load credentials file
    // let base_dirs = BaseDirs::new().unwrap();
    // let home_dir = base_dirs.home_dir().to_string_lossy().to_string();
    // let default_credentials_path = format!("{}/{}", home_dir, DEFAULT_CREDENTIALS_PATH);
    // let credentials_path = std::env::var("CREDENTIALS_PATH").unwrap_or(default_credentials_path);

    // // Quit program if credentials file does not exist
    // if !PathBuf::from(&credentials_path).exists() {
    //     info!(
    //         "Credentials file {} does not exist. Exiting program",
    //         &credentials_path
    //     );
    //     std::process::exit(1);
    // } else {
    //     let credentials_age = get_file_age(PathBuf::from(&credentials_path));
    //     info!("Credentials file age: {}", credentials_age);
    //     if credentials_age.age_duration > time::Duration::hours(6) {
    //         info!("Credentials are older than 6 hours");
    //         info!("The program is not guaranteed to run with an outdated credentials");
    //         let is_continue =
    //             Select::new("Would you like to continue? [yes/no]", vec!["yes", "no"]).prompt()?;
    //         if is_continue == "no" {
    //             info!("Exiting the program");
    //             std::process::exit(1);
    //         } else {
    //             info!("Continuing the program");
    //         }
    //     }
    // }

    // Handle CLI arguments
    cli::runner::run(PathBuf::from(credentials)).await;
    Ok(())
}
