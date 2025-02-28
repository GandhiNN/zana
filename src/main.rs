use anyhow::Result;
use aws_config::timeout::TimeoutConfig;
use aws_config::BehaviorVersion;
use aws_sdk_ssooidc::config::StalledStreamProtectionConfig;
use aws_types::region::Region;
use aws_types::SdkConfig;
use configparser::ini::Ini;
use directories::BaseDirs;
use inquire::{Select, Text};
use std::path::PathBuf;
use std::time;
use tracing::info;
use tracing_subscriber::fmt as TracingSubscriberFmt;
use zana::aws::credentials::get_credentials_file_path;
use zana::aws::defaults;
use zana::aws::region;
use zana::aws::sso::AccountInfoProvider;
use zana::aws::sso_token::SsoAccessTokenProvider;
use zana::cli;
use zana::cli::prompt::Prompt;
use zana::utils::common::get_home_dir;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup tracing
    TracingSubscriberFmt::init();
    info!("Running the program");

    // Get path to credentials file
    let credentials_path = get_credentials_file_path();

    // Check if credentials file exist
    let is_exists = check_credentials_file_existence(&credentials_path)?;
    if !is_exists {
        info!(
            "Credentials file {} does not exist. Creating the file",
            credentials_path
        );
        let is_configure =
            Select::new(Prompt::CONFIGURE_CREDENTIALS, vec!["yes", "no"]).prompt()?;
        if is_configure == "yes" {
            let _res = create_credentials().await;
        } else {
            info!("Program cannot work without credentials file!");
            std::process::exit(exitcode::OK)
        }
    } else {
        // Handle CLI arguments
        cli::runner::run(PathBuf::from(credentials_path)).await;
    }
    Ok(())
}

fn check_credentials_file_existence(path: &str) -> Result<bool> {
    if !PathBuf::from(path).exists() {
        return Ok(false);
    }
    Ok(true)
}

fn set_default_config() -> Result<SdkConfig> {
    // Set default timeout config
    let default_timeout_config = TimeoutConfig::builder()
        .connect_timeout(time::Duration::from_secs(10))
        .operation_timeout(time::Duration::from_secs(10 * 3))
        .operation_attempt_timeout(time::Duration::from_secs(10 * 3 * 3))
        .build();
    let config: aws_types::SdkConfig = aws_config::SdkConfig::builder()
        .region(Region::new("eu-west-1"))
        .behavior_version(BehaviorVersion::latest())
        .stalled_stream_protection(StalledStreamProtectionConfig::disabled())
        .timeout_config(default_timeout_config)
        .build();
    Ok(config)
}

// TODO
async fn create_credentials() -> Result<()> {
    info!("Configuring credentials...");
    info!("Building default config...");
    let default_config = set_default_config()?;
    info!("Retrieving account info...");
    let account_info_provider = AccountInfoProvider::new(&default_config);
    info!("Retrieving session name...");
    let start_url = Text::new("SSO start-url:").prompt()?;
    let regions: Vec<String> = region::REGIONS.iter().map(|x| x.to_string()).collect();
    let sso_region = Select::new("SSO region:", regions).prompt()?;
    let session_name = session_name(start_url.as_str());
    info!("Retrieving access token...");
    let token_provider = SsoAccessTokenProvider::new(
        &default_config,
        session_name.as_str(),
        &PathBuf::from(load_cache_file()),
    )?;
    let access_token = token_provider.get_access_token(&start_url).await?;
    info!("Please close the browser once you are successfully authenticated.");
    info!("Retrieving sso accounts...");
    let mut sso_accounts = account_info_provider
        .get_account_list(&access_token)
        .await?;
    sso_accounts.sort();
    let selected_account = Select::new(Prompt::SELECT_ACCOUNT, sso_accounts).prompt()?;
    let mut roles = account_info_provider
        .get_roles_for_account(&access_token, &selected_account)
        .await?;
    roles.sort();
    let selected_role = Select::new(Prompt::SELECT_ROLE, roles).prompt()?;
    // Get role credentials
    let role_credentials = account_info_provider
        .get_role_credentials(&selected_role, &selected_account, &access_token)
        .await?;
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
    let new_profile = Text::new(Prompt::INPUT_PROFILE).prompt()?;
    // Write credentials to file
    println!("Updating credentials file using profile: {}", new_profile);
    let credentials_path = format!(
        "{}/{}",
        get_home_dir().display(),
        defaults::CREDENTIALS_PATH
    );
    info!("Writing credentials to {}", credentials_path);
    write_to_credentials_file(
        new_profile.as_str(),
        sso_region.as_str(),
        selected_account.account_id.as_str(),
        access_key_id.as_str(),
        secret_access_key.as_str(),
        session_token.as_str(),
        expiration.parse::<i64>().unwrap(),
        &credentials_path,
    )?;
    Ok(())
}

pub fn load_cache_file() -> String {
    let base_dirs = BaseDirs::new().unwrap();
    let home_dir = base_dirs.home_dir().to_string_lossy().to_string();
    let default_sso_cache_path = format!("{}/{}", home_dir, defaults::AWS_SSO_PATH);
    std::env::var("SSO_CACHE_PATH").unwrap_or(default_sso_cache_path)
}

pub fn session_name(start_url: &str) -> String {
    let start_url_without_schema = start_url.replace("https://", "");
    let (subdomain, _) = start_url_without_schema.split_once(".").unwrap();
    format!("sso-{}", &subdomain)
}

#[allow(clippy::too_many_arguments)]
pub fn write_to_credentials_file(
    profile: &str,
    region: &str,
    account_id: &str,
    access_key_id: &str,
    secret_access_key: &str,
    session_token: &str,
    expiration: i64,
    path: &str,
) -> Result<String> {
    let mut credentials = Ini::new();
    credentials.set(profile, "aws_account_id", Some(String::from(account_id)));
    credentials.set(profile, "region", Some(String::from(region)));
    credentials.set(
        profile,
        "aws_access_key_id",
        Some(String::from(access_key_id)),
    );
    credentials.set(
        profile,
        "aws_secret_access_key",
        Some(String::from(secret_access_key)),
    );
    credentials.set(
        profile,
        "aws_session_token",
        Some(String::from(session_token)),
    );
    credentials.set(profile, "expiration", Some(expiration.to_string()));

    let _ = credentials.write(path);

    Ok(String::from("ok"))
}
