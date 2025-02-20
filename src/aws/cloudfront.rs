use anyhow::Result;
use aws_sdk_cloudfront::Client;
use aws_types::SdkConfig;

pub struct CloudFront {
    pub client: Client,
}

impl CloudFront {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn list_distributions(&self) -> Result<()> {
        let res = self.client.list_distributions().send().await?;
        println!("{:?}", res);
        Ok(())
    }
}
