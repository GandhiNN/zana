use anyhow::Result;
use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::default_provider::region::DefaultRegionChain;
use aws_config::timeout::TimeoutConfig;
use aws_types::region::Region;
use configparser::ini::Ini;
use std::env::set_var;
use std::fmt;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::string::String;
use std::time;

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

#[derive(Default)]
pub struct AWSCredentialsFile {
    file_path: String,
    pub age: FileAge,
}

impl AWSCredentialsFile {
    pub fn new(path: String) -> Self {
        Self {
            file_path: path,
            age: FileAge::default(),
        }
    }
    pub fn get_credentials_file_age(&mut self) {
        let metadata = std::fs::metadata(&self.file_path).unwrap();
        let t = metadata.modified().unwrap().elapsed().unwrap();
        let age = t.as_secs();
        let hours = age / 3600;
        let minutes = age % 3600 / 60;
        let seconds = age % 3600 % 60;
        self.age = FileAge {
            age_duration: t,
            seconds,
            hours,
            minutes,
        };
    }
}

pub struct AWSCredentials {
    profile: String,
    region: String,
    access_key_id: String,
    secret_access_key: String,
    session_token: String,
}

impl AWSCredentials {
    pub fn new(file: AWSCredentialsFile, profile: &str) -> Self {
        let mut config_reader = Ini::new();
        let config_map = config_reader.load(file.file_path).unwrap();
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
            profile: profile.to_owned(),
            region,
            access_key_id,
            secret_access_key,
            session_token,
        }
    }
}

pub async fn set_config(credentials: AWSCredentials, timeout: u64) -> aws_types::SdkConfig {
    // Load AWS credentials chain
    let region = DefaultRegionChain::builder()
        .profile_name(credentials.profile.as_str())
        .build()
        .region()
        .await;
    let credentials_chain = DefaultCredentialsChain::builder()
        .profile_name(credentials.profile.as_str())
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
    set_var("AWS_REGION", credentials.region.as_str());
    set_var("AWS_ACCESS_KEY_ID", credentials.access_key_id.as_str());
    set_var(
        "AWS_SECRET_ACCESS_KEY",
        credentials.secret_access_key.as_str(),
    );
    set_var("AWS_SESSION_TOKEN", credentials.session_token.as_str());

    // set AWS config
    aws_config::from_env()
        .credentials_provider(credentials_chain)
        .profile_name(credentials.profile)
        .region(Region::new(credentials.region))
        .timeout_config(timeout_config)
        .load()
        .await
}

pub struct AwsCliConfig {
    config_file: PathBuf,
}

impl AwsCliConfig {
    pub fn new(config_file: &Path) -> Self {
        Self {
            config_file: config_file.to_path_buf(),
        }
    }
    pub fn create_or_update_profile(
        &self,
        account_id: &str,
        account_name: &str,
        role_name: &str,
        start_url: &str,
        session_name: &str,
        sso_region: &str,
    ) -> Result<String> {
        let session_section_name = format!("sso-session {}", &session_name);
        let profile_name = format!("{session_name}_{role_name}_{account_name}");
        let profile_section_name = format!("profile {}", &profile_name);

        if !&self.config_file.try_exists()? {
            File::create(&self.config_file)?;
        }

        let mut config = Ini::new();
        config.set(
            &session_section_name,
            "sso_region",
            Some(String::from(sso_region)),
        );
        config.set(
            &session_section_name,
            "sso_start_url",
            Some(String::from(start_url)),
        );
        config.set(
            &session_section_name,
            "sso_start_url",
            Some(String::from(start_url)),
        );
        config.set(
            &session_section_name,
            "sso_registration_scopes",
            Some(String::from("sso:account:access")),
        );
        config.set(
            &profile_section_name,
            "sso_session",
            Some(String::from(session_name)),
        );
        config.set(
            &profile_section_name,
            "sso_account_id",
            Some(String::from(account_id)),
        );
        config.set(
            &profile_section_name,
            "sso_role_name",
            Some(String::from(role_name)),
        );
        config.set(
            &profile_section_name,
            "region",
            Some(String::from(sso_region)),
        );
        config.set(&profile_section_name, "output", Some(String::from("json")));

        let _ = config.write(&self.config_file);

        Ok(profile_name)
    }
}
