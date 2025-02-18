use crate::aws::config::AwsCliConfig;
use crate::aws::region;
use crate::aws::sso_token::AccessToken;
use crate::utils::common::get_home_dir;
use anyhow::Result;
use aws_config::SdkConfig;
use aws_sdk_sso::Client;
use inquire::{Select, Text};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use super::sso_token::SsoAccessTokenProvider;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SsoConfig {
    pub start_url: String,
    pub region: String,
}

impl SsoConfig {
    pub fn new(start_url: &str, sso_region: &str) -> Self {
        Self {
            start_url: String::from(start_url),
            region: String::from(sso_region),
        }
    }
}

impl fmt::Display for SsoConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SSO Start URL: {}\nSSO Region: {}",
            self.start_url, self.region
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountInfo {
    pub account_name: String,
    pub account_id: String,
}

impl fmt::Display for AccountInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.account_name, self.account_id)
    }
}

pub struct AccountInfoProvider {
    client: Client,
}

impl AccountInfoProvider {
    pub fn new(sdk_config: &SdkConfig) -> Self {
        AccountInfoProvider {
            client: Client::new(sdk_config),
        }
    }

    pub async fn get_account_list(&self, access_token: &AccessToken) -> Result<Vec<AccountInfo>> {
        let list_accounts = self
            .client
            .list_accounts()
            .access_token(access_token.access_token.as_str())
            .max_results(300)
            .send()
            .await?;

        let account_infos = list_accounts
            .account_list()
            .iter()
            .map(|account| AccountInfo {
                account_id: String::from(account.account_id().unwrap()),
                account_name: String::from(account.account_name().unwrap_or("unknown")),
            })
            .collect::<Vec<_>>();

        Ok(account_infos)
    }

    pub async fn get_roles_for_account(
        &self,
        access_token: &AccessToken,
        account_info: &AccountInfo,
    ) -> Result<Vec<String>> {
        let account_roles = self
            .client
            .list_account_roles()
            .access_token(access_token.access_token.as_str())
            .account_id(account_info.account_id.as_str())
            .max_results(10)
            .send()
            .await?;

        Ok(account_roles
            .role_list()
            .iter()
            .map(|r| r.role_name().unwrap())
            .map(String::from)
            .collect::<Vec<_>>())
    }
}

#[allow(dead_code)]
pub struct Sso {
    client: Client,
}

impl Sso {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    fn get_config_dir(&self, home_dir: &Path) -> Result<PathBuf> {
        let config_dir = home_dir.join(".aws_sso");
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }
        Ok(config_dir)
    }

    fn session_name(&self, start_url: &str) -> String {
        let start_url_without_schema = start_url.replace("https://", "");
        let (subdomain, _) = start_url_without_schema.split_once(".").unwrap();
        format!("sso-{}", &subdomain)
    }

    pub async fn configure_sso(&self, config: SdkConfig) -> Result<()> {
        let home_dir = get_home_dir();
        let aws_config_dir = self.get_config_dir(&home_dir).unwrap();
        let aws_config_file = aws_config_dir.join("config");

        let start_url = Text::new("SSO start-url:").prompt()?;
        let regions: Vec<String> = region::REGIONS.iter().map(|x| x.to_string()).collect();
        let sso_region = Select::new("SSO region:", regions).prompt()?;

        // Session token retrieval phase
        println!("Retrieving Access Token...");
        let session_name = self.session_name(start_url.as_str());
        let token_provider =
            SsoAccessTokenProvider::new(&config, session_name.as_str(), &aws_config_dir)?;
        let account_info_provider = AccountInfoProvider::new(&config);

        // Register client device and retrieve the access token
        let access_token = token_provider.get_access_token(&start_url).await?;

        // Get the available SSO accounts from SSO start page
        let mut sso_accounts = account_info_provider
            .get_account_list(&access_token)
            .await?;
        sso_accounts.sort();
        let selected_account = Select::new("Select account:", sso_accounts).prompt()?;

        // Get the available roles from the selected SSO account
        let mut roles = account_info_provider
            .get_roles_for_account(&access_token, &selected_account)
            .await?;
        roles.sort();
        let selected_role = Select::new("Select role:", roles).prompt()?;

        // Create AWS profile
        let aws_config_service = AwsCliConfig::new(&aws_config_file);
        let profile_name = aws_config_service.create_or_update_profile(
            &selected_account.account_id,
            &selected_account.account_name,
            &selected_role,
            &start_url,
            &session_name,
            sso_region.as_str(),
        )?;
        println!(
            "Writing AWS profile {} to {}",
            profile_name,
            aws_config_file.display()
        );

        Ok(())
    }
}
