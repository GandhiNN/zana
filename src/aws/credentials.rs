use crate::aws::sso::{session_name, AccountInfoProvider, Sso};
use crate::aws::sso_token::SsoAccessTokenProvider;
use crate::utils::fileutil::FileAge;
use anyhow::Result;
use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::default_provider::region::DefaultRegionChain;
use aws_config::timeout::TimeoutConfig;
use aws_config::BehaviorVersion;
use aws_sdk_ssooidc::config::StalledStreamProtectionConfig;
use aws_types::region::Region;
use configparser::ini::Ini;
use directories::BaseDirs;
use inquire::{Select, Text};
use std::env::set_var;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time;
use tracing::info;

const DEFAULT_AWS_SSO_PATH: &str = ".aws_sso";
const DEFAULT_CREDENTIALS_PATH: &str = ".aws_sso/credentials";
const DEFAULT_CONFIG_PATH: &str = ".aws_sso/config";
const CREDENTIALS_PLACEHOLDER: &str = "[dummy_profile]
aws_account_id=11111111111
region=eu-west-1
aws_secret_access_key=dummy_secret_access_key
aws_access_key_id=dummy_access_key_id
aws_session_token=dummy_session_token
";

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct AWSCredentials {
    aws_config_dir: String,
    pub credentials_file: String,
    profile: String,
    region: String,
    access_key_id: String,
    secret_access_key: String,
    session_token: String,
}

#[allow(dead_code)]
#[non_exhaustive]
struct Prompt {}

#[allow(dead_code)]
impl Prompt {
    pub const CONFIGURE_CREDENTIALS: &'static str = "Would you like to configure the credentials?";
    pub const REFRESH_CREDENTIALS: &'static str = "Would you like to refresh the credentials?";
    pub const CONFIGURE_PROFILE: &'static str = "Would you like to configure the default profile?";
    pub const CONFIGURE_REGION: &'static str = "Would you like to configure the default region?";
    pub const SELECT_ACCOUNT: &'static str = "Select the account you would like to use:";
    pub const SELECT_PROFILE: &'static str = "Select the profile you would like to use:";
    pub const INPUT_PROFILE: &'static str = "Enter the profile you would like to use:";
    pub const SELECT_ROLE: &'static str = "Select the role you would like to assume:";
    pub const CONTINUE_PROGRAM: &'static str = "Would you like to continue?";
}

#[allow(dead_code)]
impl AWSCredentials {
    pub fn new(profile: &str) -> Self {
        // Load credentials file
        let base_dirs = BaseDirs::new().unwrap();
        let home_dir = base_dirs.home_dir().to_string_lossy().to_string();
        let default_credentials_path = format!("{}/{}", home_dir, DEFAULT_CREDENTIALS_PATH);
        let default_config_dir = format!("{}/{}", home_dir, DEFAULT_CONFIG_PATH);
        Self {
            aws_config_dir: default_config_dir,
            credentials_file: default_credentials_path,
            profile: profile.to_string(),
            region: "".to_string(),
            access_key_id: "".to_string(),
            secret_access_key: "".to_string(),
            session_token: "".to_string(),
        }
    }

    pub async fn load(&mut self, path: &PathBuf, profile: &str) {
        let mut config_reader = Ini::new();
        let config_map = config_reader.load::<PathBuf>(path.into()).unwrap();
        println!("{:#?}", config_map);
        self.region = config_map
            .get(profile)
            .unwrap()
            .get("region")
            .unwrap()
            .clone()
            .unwrap();
        self.access_key_id = config_map
            .get(profile)
            .unwrap()
            .get("aws_access_key_id")
            .unwrap()
            .clone()
            .unwrap();
        self.secret_access_key = config_map
            .get(profile)
            .unwrap()
            .get("aws_secret_access_key")
            .unwrap()
            .clone()
            .unwrap();
        self.session_token = config_map
            .get(profile)
            .unwrap()
            .get("aws_session_token")
            .unwrap()
            .clone()
            .unwrap();
    }

    pub fn check_credentials_file_existence(&self) -> Result<bool> {
        // Check if credentials file exists
        if !PathBuf::from(&self.credentials_file).exists() {
            return Ok(false);
        }
        Ok(true)
    }

    fn create_credentials_file(&self) -> Result<()> {
        let mut f = File::create(&self.credentials_file)?;
        f.write_all(CREDENTIALS_PLACEHOLDER.as_bytes())?;
        Ok(())
    }

    fn get_credentials_file_age(&self) -> Result<FileAge> {
        let metadata = std::fs::metadata(&self.credentials_file)?;
        let t = metadata.modified().unwrap().elapsed().unwrap();
        let age = t.as_secs();
        let hours = age / 3600;
        let minutes = age % 3600 / 60;
        let seconds = age % 3600 % 60;
        Ok(FileAge {
            age_duration: t,
            seconds,
            hours,
            minutes,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_or_update_credentials(
        &self,
        profile: &str,
        region: &str,
        account_id: &str,
        access_key_id: &str,
        secret_access_key: &str,
        session_token: &str,
        expiration: i64,
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

        let _ = credentials.write(&self.credentials_file);

        Ok(String::from("ok"))
    }

    pub async fn configure(&self) -> Result<bool> {
        // Check if credentials file exists
        let credentials_file_exists = self.check_credentials_file_existence().unwrap();
        if credentials_file_exists {
            let file_age = self.get_credentials_file_age().unwrap();
            info!(
                "Credentials file {} exists and is {} hours, {} minutes, and {} seconds old.",
                self.credentials_file, file_age.hours, file_age.minutes, file_age.seconds
            );
            if file_age.hours > 6 {
                info!(
                    "Credentials file {} is older than 6 hours.",
                    self.credentials_file
                );
                info!("The program is not guaranteed to run with an outdated credentials");
                let is_refresh =
                    Select::new(Prompt::REFRESH_CREDENTIALS, vec!["yes", "no"]).prompt()?;
                if is_refresh == "yes" {
                    info!("Refreshing credentials.");
                    let sso_default_config = Sso::configure_sso()?;
                    let config: aws_types::SdkConfig = aws_config::SdkConfig::builder()
                        .region(Region::new(sso_default_config.region.clone()))
                        .behavior_version(BehaviorVersion::latest())
                        .build();
                    let account_info_provider = AccountInfoProvider::new(&config);
                    let session_name = session_name(sso_default_config.start_url.as_str());
                    let token_provider = SsoAccessTokenProvider::new(
                        &config,
                        session_name.as_str(),
                        &PathBuf::from(&self.aws_config_dir),
                    )?;
                    let access_token = token_provider
                        .get_access_token(&sso_default_config.start_url)
                        .await?;
                    println!("{:?}", access_token);
                    let mut sso_accounts = account_info_provider
                        .get_account_list(&access_token)
                        .await?;
                    sso_accounts.sort();
                    let selected_account =
                        Select::new(Prompt::SELECT_ACCOUNT, sso_accounts).prompt()?;
                    let mut roles = account_info_provider
                        .get_roles_for_account(&access_token, &selected_account)
                        .await?;
                    roles.sort();
                    let selected_role = Select::new(Prompt::SELECT_ROLE, roles).prompt()?;

                    // Get role credentials
                    let role_credentials = account_info_provider
                        .get_role_credentials(&selected_role, &selected_account, &access_token)
                        .await?;
                    let (
                        mut access_key_id,
                        mut secret_access_key,
                        mut session_token,
                        mut expiration,
                    ) = (String::new(), String::new(), String::new(), String::new());
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
                    self.create_or_update_credentials(
                        new_profile.as_str(),
                        sso_default_config.region.as_str(),
                        selected_account.account_id.as_str(),
                        access_key_id.as_str(),
                        secret_access_key.as_str(),
                        session_token.as_str(),
                        expiration.parse::<i64>().unwrap(),
                    )?;
                } else {
                    info!("Continuing program with possibly outdated credentials.");
                    return Ok(true);
                }
            }
        } else {
            info!(
                "Credentials file {} does not exists. Creating the file",
                self.credentials_file
            );
            self.create_credentials_file()?;
            let is_configure =
                Select::new(Prompt::CONFIGURE_CREDENTIALS, vec!["yes", "no"]).prompt()?;
            info!("Configuring credentials...");
            let sso_default_config = Sso::configure_sso()?;
            info!("Building default config...");
            // Set default timeout config
            let default_timeout_config = TimeoutConfig::builder()
                .connect_timeout(time::Duration::from_secs(10000))
                .operation_timeout(time::Duration::from_secs(10000 * 3))
                .operation_attempt_timeout(time::Duration::from_secs(10000 * 3 * 3))
                .build();
            let config: aws_types::SdkConfig = aws_config::SdkConfig::builder()
                .region(Region::new(sso_default_config.region.clone()))
                .behavior_version(BehaviorVersion::latest())
                .stalled_stream_protection(StalledStreamProtectionConfig::disabled())
                .timeout_config(default_timeout_config)
                .build();
            info!("Retrieving account info...");
            let account_info_provider = AccountInfoProvider::new(&config);
            info!("Retrieving session name...");
            let session_name = session_name(sso_default_config.start_url.as_str());
            let token_provider = SsoAccessTokenProvider::new(
                &config,
                session_name.as_str(),
                &PathBuf::from(load_cache_file()),
            )?;
            info!("Retrieving access token...");
            let access_token = token_provider
                .get_access_token(&sso_default_config.start_url)
                .await?;
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
            self.create_or_update_credentials(
                new_profile.as_str(),
                sso_default_config.region.as_str(),
                selected_account.account_id.as_str(),
                access_key_id.as_str(),
                secret_access_key.as_str(),
                session_token.as_str(),
                expiration.parse::<i64>().unwrap(),
            )?;
            if is_configure == "no" {
                info!("Exiting program.");
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub async fn set_config(
        &mut self,
        path: &PathBuf,
        profile: &str,
        timeout: u64,
    ) -> aws_types::SdkConfig {
        // Load credentials file
        self.load(path, profile).await;

        // Load AWS credentials chain
        let region = DefaultRegionChain::builder()
            .profile_name(self.profile.as_str())
            .build()
            .region()
            .await;
        let credentials_chain = DefaultCredentialsChain::builder()
            .profile_name(self.profile.as_str())
            .region(region.clone())
            .build()
            .await;

        // Set timeout config
        let timeout_config = TimeoutConfig::builder()
            .connect_timeout(time::Duration::from_secs(timeout))
            .operation_timeout(time::Duration::from_secs(timeout * 3))
            .operation_attempt_timeout(time::Duration::from_secs(timeout * 3 * 3))
            .build();

        // Configure AWS credentials as process-scoped env var
        set_var("AWS_REGION", self.region.as_str());
        set_var("AWS_ACCESS_KEY_ID", self.access_key_id.as_str());
        set_var("AWS_SECRET_ACCESS_KEY", self.secret_access_key.as_str());
        set_var("AWS_SESSION_TOKEN", self.session_token.as_str());

        // set AWS config
        aws_config::from_env()
            .credentials_provider(credentials_chain)
            .profile_name(self.profile.clone())
            .region(Region::new(self.region.clone()))
            .timeout_config(timeout_config)
            .load()
            .await
    }
}

pub fn load_credentials_file() -> String {
    // Load credentials file
    let base_dirs = BaseDirs::new().unwrap();
    let home_dir = base_dirs.home_dir().to_string_lossy().to_string();
    let default_credentials_path = format!("{}/{}", home_dir, DEFAULT_CREDENTIALS_PATH);
    std::env::var("CREDENTIALS_PATH").unwrap_or(default_credentials_path)
}

pub fn load_cache_file() -> String {
    // Load credentials file
    let base_dirs = BaseDirs::new().unwrap();
    let home_dir = base_dirs.home_dir().to_string_lossy().to_string();
    let default_sso_cache_path = format!("{}/{}", home_dir, DEFAULT_AWS_SSO_PATH);
    std::env::var("SSO_CACHE_PATH").unwrap_or(default_sso_cache_path)
}
