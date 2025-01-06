use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::default_provider::region::DefaultRegionChain;
use aws_config::timeout::TimeoutConfig;
use aws_types::region::Region;
use configparser::ini::Ini;
use std::env::set_var;
use std::string::String;
use std::time;

pub struct AWSConfigFile {
    config_file_path: String,
}

impl AWSConfigFile {
    pub fn new(config_path: String) -> Self {
        Self {
            config_file_path: config_path,
        }
    }
}

pub struct AWSCredentialsConfig {
    profile: String,
    region: String,
    access_key_id: String,
    secret_access_key: String,
    session_token: String,
}

impl AWSCredentialsConfig {
    pub fn new(config: AWSConfigFile, profile: &str) -> Self {
        let mut config_reader = Ini::new();
        let config_map = config_reader.load(config.config_file_path).unwrap();
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

pub async fn set_config(
    credentials_config: AWSCredentialsConfig,
    timeout: u64,
) -> aws_types::SdkConfig {
    // Load AWS credentials chain
    let region = DefaultRegionChain::builder()
        .profile_name(credentials_config.profile.as_str())
        .build()
        .region()
        .await;
    let credentials = DefaultCredentialsChain::builder()
        .profile_name(credentials_config.profile.as_str())
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
    set_var("AWS_REGION", credentials_config.region.as_str());
    set_var(
        "AWS_ACCESS_KEY_ID",
        credentials_config.access_key_id.as_str(),
    );
    set_var(
        "AWS_SECRET_ACCESS_KEY",
        credentials_config.secret_access_key.as_str(),
    );
    set_var(
        "AWS_SESSION_TOKEN",
        credentials_config.session_token.as_str(),
    );

    // set AWS config
    aws_config::from_env()
        .credentials_provider(credentials)
        .profile_name(credentials_config.profile)
        .region(Region::new(credentials_config.region))
        .timeout_config(timeout_config)
        .load()
        .await
}
