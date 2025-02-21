use anyhow::Result;
use aws_sdk_secretsmanager::{Client, Error};
use aws_types::SdkConfig;

pub struct SecretsManager {
    pub client: Client,
}

impl SecretsManager {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }
}
