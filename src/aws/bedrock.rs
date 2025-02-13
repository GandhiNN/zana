use anyhow::Result;
use aws_sdk_bedrock::{Client, Error};
use aws_types::SdkConfig;

pub struct Bedrock {
    pub client: Client,
}

impl Bedrock {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn list_foundational_models(&self) -> Result<(), Error> {
        let result = self.client.list_foundation_models().send().await?;
        println!("{:#?}", result.model_summaries);
        Ok(())
    }
}
