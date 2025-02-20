use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_types::region::Region;
use directories::BaseDirs;
use inquire::{Select, Text};
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::fmt as TracingSubscriberFmt;
use zana::aws::region::REGIONS;
use zana::aws::sso::{AccountInfoProvider, SsoConfig};
use zana::aws::sso_token::SsoAccessTokenProvider;
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

async fn check_credentials_validity(path: &str) -> Result<bool> {
    // Load config file
    let base_dirs = BaseDirs::new().unwrap();
    let home_dir = base_dirs.home_dir().to_string_lossy().to_string();
    let aws_config_dir = format!("{}/{}", home_dir, ".aws");

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
            std::process::exit(1); // exiting early from the program
        } else {
            info!("Configuring credentials...");
            let sso_config = configure_sso()?;
            let config = aws_config::SdkConfig::builder()
                .region(Region::new(sso_config.region.clone()))
                .behavior_version(BehaviorVersion::latest())
                .build();
            let account_info_provider = AccountInfoProvider::new(&config);
            let session_name = session_name(sso_config.start_url.as_str());
            let token_provider = SsoAccessTokenProvider::new(
                &config,
                session_name.as_str(),
                &PathBuf::from(aws_config_dir),
            )?;
            let access_token = token_provider
                .get_access_token(&sso_config.start_url)
                .await?;
            let mut sso_accounts = account_info_provider
                .get_account_list(&access_token)
                .await?;
            sso_accounts.sort();
            let selected_account = Select::new("Select account:", sso_accounts).prompt()?;
            let mut roles = account_info_provider
                .get_roles_for_account(&access_token, &selected_account)
                .await?;
            roles.sort();
            let selected_role = Select::new("Select role:", roles).prompt()?;

            // Get role credentials
            let role_credentials = account_info_provider
                .get_role_credentials(&selected_role, &selected_account, &access_token)
                .await?;
            println!("{:#?}", role_credentials.role_credentials);
            let (mut access_key_id, mut secret_access_key, mut session_token, mut expiration) =
                (String::new(), String::new(), String::new(), String::new());
            let _ = role_credentials
                .role_credentials
                .into_iter()
                .map(|x| {
                    access_key_id = x.access_key_id().unwrap().to_string();
                    secret_access_key = x.secret_access_key().unwrap().to_string();
                    session_token = x.session_token().unwrap().to_string();
                    expiration = x.expiration().to_string();
                })
                .collect::<Vec<()>>();
            println!("Access key ID: {}", access_key_id);
            println!("Secret access key: {}", secret_access_key);
            println!("Session token: {}", session_token);
            println!("Expiration: {}", expiration);
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

fn configure_sso() -> Result<SsoConfig> {
    let start_url = Text::new("SSO start-url:").prompt()?;
    let regions: Vec<String> = REGIONS.iter().map(|x| x.to_string()).collect();
    let sso_region = Select::new("SSO region:", regions).prompt()?;
    Ok(SsoConfig {
        start_url,
        region: sso_region,
    })
}

fn session_name(start_url: &str) -> String {
    let start_url_without_schema = start_url.replace("https://", "");
    let (subdomain, _) = start_url_without_schema.split_once(".").unwrap();
    format!("sso-{}", &subdomain)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup tracing
    TracingSubscriberFmt::init();
    info!("Running the program");

    // Credentials validity check
    let credentials = load_credentials_file();
    let is_cred_valid = check_credentials_validity(&credentials).await?;
    println!("{}", is_cred_valid);

    // Handle CLI arguments
    cli::runner::run(PathBuf::from(credentials)).await;
    Ok(())
}
