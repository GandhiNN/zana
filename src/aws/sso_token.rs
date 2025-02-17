use crate::utils::json;
use crate::utils::serde::json_date_format;
use anyhow::Result;
use aws_sdk_ssooidc::Client;
use chrono::{DateTime, Duration, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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
