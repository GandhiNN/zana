use anyhow::Result;
use configparser::ini::Ini;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::string::String;

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

    // TODO: Complete the method
    pub fn create_or_update_credentials(
        &self,
        account_id: &str,
        account_name: &str,
        role_name: &str,
        access_key_id: &str,
        secret_access_key: &str,
        sso_region: &str,
        session_name: &str,
        token_expiration: &str,
        token_expiration_date: &str,
    ) -> Result<()> {
        if !&self.config_file.try_exists()? {
            File::create(&self.config_file)?;
        }
        Ok(())
    }
}
