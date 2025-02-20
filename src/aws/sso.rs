use crate::aws::config::AwsCliConfig;
use crate::aws::credentials::AWSCredentials;
use crate::aws::region;
use crate::aws::sso_token::AccessToken;
use crate::utils::common::get_home_dir;
use anyhow::Result;
use aws_config::SdkConfig;
use aws_sdk_sso::operation::get_role_credentials::GetRoleCredentialsOutput;
use aws_sdk_sso::Client;
use inquire::{Select, Text};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use super::sso_token::SsoAccessTokenProvider;

// Constants
const PARENT_CONFIG_PATH: &str = ".aws_sso";
const CONFIG_PATH: &str = "config";
const CREDENTIALS_PATH: &str = "credentials";

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

    pub async fn get_role_credentials(
        &self,
        role: &str,
        account_info: &AccountInfo,
        access_token: &AccessToken,
    ) -> Result<GetRoleCredentialsOutput> {
        let res = self
            .client
            .get_role_credentials()
            .role_name(role)
            .account_id(account_info.account_id.as_str())
            .access_token(access_token.access_token.as_str())
            .send()
            .await?;
        Ok(res)
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
        let config_dir = home_dir.join(PARENT_CONFIG_PATH);
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }
        Ok(config_dir)
    }

    fn get_session_name(&self, start_url: &str) -> String {
        let start_url_without_schema = start_url.replace("https://", "");
        let (subdomain, _) = start_url_without_schema.split_once(".").unwrap();
        format!("sso-{}", &subdomain)
    }

    pub async fn get_role_credentials(
        &self,
        provider: AccountInfoProvider,
        account: &AccountInfo,
        role: &str,
        token: &AccessToken,
    ) -> Result<GetRoleCredentialsOutput> {
        // Get role credentials
        let role_credentials = provider.get_role_credentials(role, account, token).await?;
        Ok(role_credentials)
    }

    pub async fn get_session_token(
        &self,
        url: &str,
        session: &str,
        conf: &SdkConfig,
        conf_dir: &Path,
    ) -> Result<AccessToken> {
        // Session token retrieval phase
        println!("Retrieving Access Token...");
        let token_provider = SsoAccessTokenProvider::new(conf, session, conf_dir)?;

        // Register client device and retrieve the access token
        token_provider.get_access_token(url).await
    }

    pub async fn register_sso(&self, config: SdkConfig) -> Result<()> {
        // Prepare the configuration
        let home_dir = get_home_dir();
        let aws_config_dir = self.get_config_dir(&home_dir).unwrap();
        let aws_config_file = aws_config_dir.join(CONFIG_PATH);
        let aws_credentials_file = aws_config_dir.join(CREDENTIALS_PATH);

        let start_url = Text::new("SSO start-url:").prompt()?;
        let regions: Vec<String> = region::REGIONS.iter().map(|x| x.to_string()).collect();
        let sso_region = Select::new("SSO region:", regions).prompt()?;

        // Set the account info provider
        let account_info_provider = AccountInfoProvider::new(&config);

        // Get session name
        let session_name = self.get_session_name(start_url.as_str());

        // Register client device and get the access token
        let access_token = self
            .get_session_token(&start_url, &session_name, &config, &aws_config_dir)
            .await?;

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

        // Get the role credentials
        let _role_credentials = self
            .get_role_credentials(
                account_info_provider,
                &selected_account,
                &selected_role,
                &access_token,
            )
            .await?;

        // Create or update AWS credentials file
        // TODO!
        let account_id = selected_account.account_id.clone();
        print!(
            "Please type the profile to use for the AWS credentials file (default = {}): ",
            account_id
        );
        let profile = String::new();
        let _aws_credentials_service = AWSCredentials::new(&aws_credentials_file, profile.as_str());
        println!("{}", account_id);
        println!(
            "Writing AWS credentials using profile: {} to {}",
            profile,
            aws_credentials_file.display()
        );

        // Create or update AWS config file
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

pub fn configure_sso() -> Result<SsoConfig> {
    let start_url = Text::new("SSO start-url:").prompt()?;
    let regions: Vec<String> = region::REGIONS.iter().map(|x| x.to_string()).collect();
    let sso_region = Select::new("SSO region:", regions).prompt()?;
    Ok(SsoConfig {
        start_url,
        region: sso_region,
    })
}

pub fn session_name(start_url: &str) -> String {
    let start_url_without_schema = start_url.replace("https://", "");
    let (subdomain, _) = start_url_without_schema.split_once(".").unwrap();
    format!("sso-{}", &subdomain)
}
