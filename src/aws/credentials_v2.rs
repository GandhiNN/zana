use anyhow::Result;
use configparser::ini::Ini;
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

pub struct AwsCredentialsConfig {
    credentials_file: PathBuf,
    pub age: FileAge,
}

impl AwsCredentialsConfig {
    pub fn new(credentials_file: &Path) -> Result<Self> {
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
}
