use crate::aws::region;
use inquire::{InquireError, Select, Text};
use serde::{Deserialize, Serialize};
use std::fmt;

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

pub fn configure_sso() -> Result<SsoConfig, InquireError> {
    let start_url = Text::new("SSO start-url:").prompt()?;
    let regions: Vec<String> = region::REGIONS.iter().map(|x| x.to_string()).collect();
    let sso_region = Select::new("SSO region:", regions).prompt()?;
    Ok(SsoConfig {
        start_url,
        region: sso_region,
    })
}
