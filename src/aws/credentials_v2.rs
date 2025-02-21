use crate::aws::sso::{configure_sso, session_name, AccountInfoProvider};
use crate::aws::sso_token::SsoAccessTokenProvider;
use crate::utils::fileutil::get_file_age;
use anyhow::Result;
use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::default_provider::region::DefaultRegionChain;
use aws_config::timeout::TimeoutConfig;
use aws_config::BehaviorVersion;
use aws_types::region::Region;
use configparser::ini::Ini;
use directories::BaseDirs;
use inquire::{Select, Text};
use std::env::set_var;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;
use std::string::String;
use std::time;
use tracing::info;

const DEFAULT_CREDENTIALS_PATH: &str = ".aws_sso/credentials";

#[derive(Default)]
pub struct FileAge {
    pub age_duration: time::Duration,
    pub seconds: u64,
    pub hours: u64,
    pub minutes: u64,
}

impl fmt::Display for FileAge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}h {}m {}s", self.hours, self.minutes, self.seconds)
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct AWSCredentials {
    pub credential_file: PathBuf,
    pub age: FileAge,
    profile: String,
    region: String,
    access_key_id: String,
    secret_access_key: String,
    session_token: String,
}

impl AWSCredentials {
    pub fn new(path: &PathBuf, profile: &str) -> Self {
        let mut config_reader = Ini::new();

        let config_map = config_reader.load::<PathBuf>(path.into()).unwrap();
        let region = config_map
            .get(profile)
            .unwrap()
            .get("region")
            .unwrap()
            .clone()
            .unwrap();
        let access_key_id = config_map
            .get(profile)
            .unwrap()
            .get("aws_access_key_id")
            .unwrap()
            .clone()
            .unwrap();
        let secret_access_key = config_map
            .get(profile)
            .unwrap()
            .get("aws_secret_access_key")
            .unwrap()
            .clone()
            .unwrap();
        let session_token = config_map
            .get(profile)
            .unwrap()
            .get("aws_session_token")
            .unwrap()
            .clone()
            .unwrap();
        Self {
            credential_file: path.into(),
            age: FileAge::default(),
            profile: profile.to_owned(),
            region,
            access_key_id,
            secret_access_key,
            session_token,
        }
    }

    pub fn get_credentials_file_age(&mut self) -> FileAge {
        let metadata = std::fs::metadata(&self.credential_file).unwrap();
        let t = metadata.modified().unwrap().elapsed().unwrap();
        let age = t.as_secs();
        let hours = age / 3600;
        let minutes = age % 3600 / 60;
        let seconds = age % 3600 % 60;
        FileAge {
            age_duration: t,
            seconds,
            hours,
            minutes,
        }
    }

    pub async fn set_config(&self, timeout: u64) -> aws_types::SdkConfig {
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

pub struct AwsCredentialsConfig {
    credentials_file: PathBuf,
    pub age: FileAge,
}

#[allow(clippy::too_many_arguments)]
impl AwsCredentialsConfig {
    pub fn new(credentials_file: &PathBuf) -> Result<Self> {
        if !credentials_file.exists() {
            File::create(credentials_file)?;
        }
        Ok(Self {
            credentials_file: credentials_file.to_path_buf(),
            age: FileAge::default(),
        })
    }

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

    pub fn get_credentials_file_age(&mut self) -> FileAge {
        let metadata = std::fs::metadata(&self.credentials_file).unwrap();
        let t = metadata.modified().unwrap().elapsed().unwrap();
        let age = t.as_secs();
        let hours = age / 3600;
        let minutes = age % 3600 / 60;
        let seconds = age % 3600 % 60;
        FileAge {
            age_duration: t,
            seconds,
            hours,
            minutes,
        }
    }

    pub fn load_credentials(&mut self) -> Result<(), Box<dyn Error>> {
        let mut config = Ini::new();
        let map = config.load(&self.credentials_file)?;
        println!("{:?}", map);
        Ok(())
    }
}

pub fn load_credentials_file() -> String {
    // Load credentials file
    let base_dirs = BaseDirs::new().unwrap();
    let home_dir = base_dirs.home_dir().to_string_lossy().to_string();
    let default_credentials_path = format!("{}/{}", home_dir, DEFAULT_CREDENTIALS_PATH);
    std::env::var("CREDENTIALS_PATH").unwrap_or(default_credentials_path)
}

pub async fn check_credentials_validity(path: &PathBuf, profile: &str) -> Result<bool> {
    // Load config file
    let base_dirs = BaseDirs::new().unwrap();
    let home_dir = base_dirs.home_dir().to_string_lossy().to_string();
    let aws_config_dir = format!("{}/{}", home_dir, ".aws");

    // Quit program if credentials file does not exist
    if !PathBuf::from(path).exists() {
        info!(
            "Credentials file {} does not exist",
            &path.to_string_lossy()
        );
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
            let new_profile = Text::new("Please input profile name:").prompt()?;
            // Write credentials to file
            let aws_credentials_config = AwsCredentialsConfig::new(&PathBuf::from(path))?;
            println!("Updating credentials file using profile: {}", new_profile);
            aws_credentials_config.create_or_update_credentials(
                new_profile.as_str(),
                sso_config.region.as_str(),
                selected_account.account_id.as_str(),
                access_key_id.as_str(),
                secret_access_key.as_str(),
                session_token.as_str(),
                expiration.parse::<i64>().unwrap(),
            )?;
        }
    } else {
        // credentials file exists
        // check for age of the credentials file
        let credentials_age = get_file_age(PathBuf::from(&path));
        info!("Credentials file age: {}", credentials_age);
        if credentials_age.age_duration > time::Duration::from_secs(6 * 3600) {
            info!("Credentials are older than 6 hours");
            info!("The program is not guaranteed to run with an outdated credentials");
            let is_continue =
                Select::new("Would you like to continue? [yes/no]", vec!["yes", "no"]).prompt()?;
            if is_continue == "no" {
                return Ok(false);
            } else {
                let is_configure = Select::new(
                    "Would you like to re-configure the profile? [yes/no]",
                    vec!["yes", "no"],
                )
                .prompt()?;
                if is_configure == "no" {
                    info!("Continuing with the same credentials");
                    return Ok(true);
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
                    let new_profile = Text::new("Please input profile name:").prompt()?;
                    // Write credentials to file
                    let aws_credentials_config = AwsCredentialsConfig::new(&PathBuf::from(path))?;
                    println!("Updating credentials file using profile: {}", new_profile);
                    aws_credentials_config.create_or_update_credentials(
                        new_profile.as_str(),
                        sso_config.region.as_str(),
                        selected_account.account_id.as_str(),
                        access_key_id.as_str(),
                        secret_access_key.as_str(),
                        session_token.as_str(),
                        expiration.parse::<i64>().unwrap(),
                    )?;
                }
            }
        } else {
            // credentials file is not older than 6 hours
            info!("Credentials are up-to-date");
            // check if `profile` exists in the credentials file
            let credentials_profile = Ini::new().load(path).unwrap();
            if !credentials_profile.contains_key(profile) {
                info!("Profile: {} does not exist!", profile);
                let is_continue = Select::new(
                    "Would you like to configure the profile? [yes/no]",
                    vec!["yes", "no"],
                )
                .prompt()?;
                if is_continue == "no" {
                    return Ok(false);
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
                    let new_profile = Text::new("Please input profile name:").prompt()?;
                    // Write credentials to file
                    let aws_credentials_config = AwsCredentialsConfig::new(&PathBuf::from(path))?;
                    aws_credentials_config.create_or_update_credentials(
                        new_profile.as_str(),
                        sso_config.region.as_str(),
                        selected_account.account_id.as_str(),
                        access_key_id.as_str(),
                        secret_access_key.as_str(),
                        session_token.as_str(),
                        expiration.parse::<i64>().unwrap(),
                    )?;
                }
            } else {
                info!("Profile: {} exist!", profile);
            }
        }
    }
    Ok(true)
}
