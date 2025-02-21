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
        let resp = self.client.list_secrets().send().await?;
        println!("{:#?}", resp);
        Ok(())
    }
}
