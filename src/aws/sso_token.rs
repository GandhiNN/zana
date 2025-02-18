use crate::utils::json;
use crate::utils::serde::json_date_format;
use anyhow::{anyhow, Result};
use aws_config::SdkConfig;
use aws_sdk_ssooidc::Client;
use chrono::{DateTime, Duration, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, io::Write};

// Define constants
const CLIENT_NAME: &str = "zana-rs";
const DEVICE_GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:device_code";
const REFRESH_GRANT_TYPE: &str = "refresh_token";
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccessToken {
    pub start_url: String,
    pub region: String,
    pub access_token: String,
    #[serde(with = "json_date_format")]
    pub expires_at: DateTime<Utc>,
    #[serde(flatten)]
    pub device_client: DeviceClient,
    pub refresh_token: String,
}

impl AccessToken {
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
}

pub struct AccessTokenCache {
    cache_dir: PathBuf,
    sso_session_name: String,
}

impl AccessTokenCache {
    pub fn new(sso_session_name: &str, cache_dir: &Path) -> Self {
        Self {
            cache_dir: cache_dir.to_path_buf(),
            sso_session_name: String::from(sso_session_name),
        }
    }
    pub fn get_cached_token(&self) -> Result<AccessToken> {
        let cache_file_path = self.cache_dir.join(format!("{}.json", self.hash_key()));
        json::read_from_file(cache_file_path.as_path())
    }
    pub fn cache_token(&self, access_token: AccessToken) -> Result<AccessToken> {
        let cache_file_path = self.cache_dir.join(format!("{}.json", self.hash_key()));
        json::write_to_file(cache_file_path.as_path(), &access_token)?;
        Ok(access_token)
    }
    fn hash_key(&self) -> String {
        use sha1::{Digest, Sha1};
        let mut hasher = Sha1::new();
        hasher.update(self.sso_session_name.as_str());
        format!("{:02x}", hasher.finalize())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClient {
    pub client_id: String,
    pub client_secret: String,
    #[serde(with = "json_date_format")]
    pub registration_expires_at: DateTime<Utc>,
}

impl DeviceClient {
    pub fn is_expired(&self) -> bool {
        self.registration_expires_at < Utc::now()
    }
}

pub struct SsoAccessTokenProvider {
    sso_session_name: String,
    client: Client,
    cache: AccessTokenCache,
}

impl SsoAccessTokenProvider {
    pub fn new(config: &SdkConfig, sso_session_name: &str, config_dir: &Path) -> Result<Self> {
        let sso_cache_dir = config_dir.join("sso").join("cache");
        if !sso_cache_dir.exists() {
            fs::create_dir_all(&sso_cache_dir)?;
        }
        Ok(Self {
            sso_session_name: String::from(sso_session_name),
            client: Client::new(config),
            cache: AccessTokenCache::new(sso_session_name, sso_cache_dir.as_path()),
        })
    }

    pub async fn get_access_token(&self, start_url: &str) -> Result<AccessToken> {
        let cached_token_option = self.cache.get_cached_token();

        match cached_token_option {
            Ok(cached_token) => {
                if cached_token.is_expired() {
                    self.get_new_token(start_url).await
                } else {
                    self.refresh_token(cached_token).await
                }
            }
            Err(_) => self.get_new_token(start_url).await,
        }
    }

    async fn get_new_token(&self, start_url: &str) -> Result<AccessToken> {
        let device_client = self.register_device_client().await?;
        self.authenticate(start_url, device_client).await
    }

    async fn register_device_client(&self) -> Result<DeviceClient, anyhow::Error> {
        let response = self
            .client
            .register_client()
            .client_name(format!("{}-{}", CLIENT_NAME, self.sso_session_name))
            .client_type("public")
            .scopes("sso:account:access")
            .send()
            .await?;
        let client_id = response.client_id().unwrap();
        let client_secret = response.client_secret().unwrap();
        let registration_expires_at = Utc
            .timestamp_opt(response.client_secret_expires_at(), 0)
            .unwrap();
        let device_client = DeviceClient {
            client_id: String::from(client_id),
            client_secret: String::from(client_secret),
            registration_expires_at,
        };
        Ok(device_client)
    }

    async fn authenticate(
        &self,
        start_url: &str,
        device_client: DeviceClient,
    ) -> Result<AccessToken> {
        let auth_response = self
            .client
            .start_device_authorization()
            .client_id(device_client.client_id.as_str())
            .client_secret(device_client.client_secret.as_str())
            .start_url(start_url)
            .send()
            .await?;

        // Prompt user to open browser
        open::that(auth_response.verification_uri_complete().unwrap())?;

        // TODO: Implement timeout for browser inactivity termination
        // i.e. if user is not responding to browser prompt, terminate after n seconds
        println!(
            "\nVerify authorization code: \x1B[36;1m{}\x1B[0m",
            &auth_response.user_code().unwrap()
        );
        let interval = auth_response.interval();
        loop {
            let token_response = self
                .client
                .create_token()
                .client_id(device_client.client_id.as_str())
                .client_secret(device_client.client_secret.as_str())
                .grant_type(DEVICE_GRANT_TYPE)
                .device_code(auth_response.device_code().unwrap())
                .send()
                .await;

            match token_response {
                Ok(out) => {
                    let access_token = out.access_token().unwrap();
                    let refresh_token = out.refresh_token().unwrap();
                    let expires_at = Utc::now() + Duration::seconds(out.expires_in() as i64);

                    let access_token = AccessToken {
                        region: self.client.config().region().unwrap().to_string(),
                        start_url: String::from(start_url),
                        access_token: String::from(access_token),
                        expires_at,
                        device_client,
                        refresh_token: String::from(refresh_token),
                    };

                    print!("\x1B[1A");
                    print!("\x1B[2K");
                    std::io::stdout().flush().unwrap();

                    break Ok(self.cache.cache_token(access_token)?);
                }
                Err(err) => {
                    let service_error = err.into_service_error();
                    if service_error.is_access_denied_exception() {
                        break Err(anyhow!("Access request rejected"));
                    }
                    let millis = Duration::seconds(interval as i64);
                    std::thread::sleep(millis.to_std()?);
                }
            }
        }
    }

    async fn refresh_token(&self, cached_token: AccessToken) -> Result<AccessToken> {
        let device_client = &cached_token.device_client;
        let response = self
            .client
            .create_token()
            .client_id(device_client.client_id.as_str())
            .client_secret(device_client.client_secret.as_str())
            .grant_type(REFRESH_GRANT_TYPE)
            .refresh_token(cached_token.refresh_token.as_str())
            .send()
            .await?;

        let access_token = response.access_token().unwrap();
        let refresh_token = response.refresh_token().unwrap();
        let expires_at = Utc::now() + Duration::seconds(response.expires_in() as i64);

        let new_access_token = AccessToken {
            region: self.client.config().region().unwrap().to_string(),
            start_url: cached_token.start_url.clone(),
            access_token: String::from(access_token),
            expires_at,
            device_client: cached_token.device_client,
            refresh_token: String::from(refresh_token),
        };

        self.cache.cache_token(new_access_token)
    }
}
