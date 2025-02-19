use anyhow::Result;
use configparser::ini::Ini;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::string::String;

pub struct AwsCredentialsConfig {
    credentials_file: PathBuf,
}

impl AwsCredentialsConfig {
    pub fn new(credentials_file: &Path) -> Self {
        Self {
            credentials_file: credentials_file.to_path_buf(),
        }
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
        if !&self.credentials_file.try_exists()? {
            File::create(&self.credentials_file)?;
        }

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
}
