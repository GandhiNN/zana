use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::default_provider::region::DefaultRegionChain;
use aws_config::timeout::TimeoutConfig;
use configparser::ini::Ini;
use std::env::set_var;
use std::error::Error;
use std::string::String;
use std::time;

const PATH: &str = "config.ini";

pub fn load_credentials_config(
    profile_name: &str,
) -> Result<(String, String, String, String), Box<dyn Error>> {
    let mut config = Ini::new();
    let cfg_map = config.load(PATH)?;
    let region = cfg_map
        .get(profile_name)
        .unwrap()
        .get("region")
        .unwrap()
        .clone()
        .unwrap();
    let access_key_id = cfg_map
        .get(profile_name)
        .unwrap()
        .get("aws_access_key_id")
        .unwrap()
        .clone()
        .unwrap();
    let secret_access_key = cfg_map
        .get(profile_name)
        .unwrap()
        .get("aws_secret_access_key")
        .unwrap()
        .clone()
        .unwrap();
    let session_token = cfg_map
        .get(profile_name)
        .unwrap()
        .get("aws_session_token")
        .unwrap()
        .clone()
        .unwrap();

    Ok((region, access_key_id, secret_access_key, session_token))
}

pub async fn set_config(aws_profile: &str, timeout: u64) -> aws_types::SdkConfig {
    // Set the AWS Region
    let region = DefaultRegionChain::builder()
        .profile_name(aws_profile)
        .build()
        .region()
        .await;

    // Load the credentials to be used
    let credentials = DefaultCredentialsChain::builder()
        .profile_name(aws_profile)
        .region(region.clone())
        .build()
        .await;

    // Set timeout config
    let timeout_config = TimeoutConfig::builder()
        .connect_timeout(time::Duration::from_secs(timeout))
        .operation_timeout(time::Duration::from_secs(timeout * 3))
        .operation_attempt_timeout(time::Duration::from_secs(timeout * 3 * 3))
        .build();

    // Configure our AWS credentials as process-scoped env var
    // config.ini must not use double-quoting
    match load_credentials_config(aws_profile) {
        Ok(res) => {
            set_var("AWS_REGION", res.0);
            set_var("AWS_ACCESS_KEY_ID", res.1);
            set_var("AWS_SECRET_ACCESS_KEY", res.2);
            set_var("AWS_SESSION_TOKEN", res.3);
        }
        Err(e) => println!("{:?}", e),
    }

    // set AWS config
    aws_config::from_env()
        .credentials_provider(credentials)
        .profile_name(aws_profile)
        .region("eu-west-1")
        .timeout_config(timeout_config)
        .load()
        .await
}
