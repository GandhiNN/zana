use anyhow::Result;
use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::default_provider::region::DefaultRegionChain;
use aws_config::timeout::TimeoutConfig;
use aws_types::region::Region;
use configparser::ini::Ini;
use std::env::set_var;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;
use std::string::String;
use std::time;

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
    expiration: String,
}

impl AWSCredentials {
    pub fn new(path: &str, profile: &str) -> Self {
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
        let expiration = config_map
            .get(profile)
            .unwrap()
            .get("expiration")
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
            expiration,
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

    // TODO: Complete the method
    pub fn create_or_update_credentials(&self) -> Result<()> {
        if !&self.credential_file.try_exists()? {
            File::create(&self.credential_file)?;
        }
        Ok(())
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
