use anyhow::Result;
use aws_sdk_secretsmanager::Client;
use aws_types::SdkConfig;

pub struct SecretsManager {
    pub client: Client,
}

impl SecretsManager {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn list_secrets(&self) -> Result<()> {
        let mut secrets = self.client.list_secrets().into_paginator().send();
        while let Some(output) = secrets.next().await {
            match output {
                Ok(secret) => {
                    println!("{:#?}", secret);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Ok(())
    }
}
