use anyhow::Result;
use aws_sdk_sts::Client;
use aws_types::SdkConfig;

pub struct STS {
    pub client: Client,
}

impl STS {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn get_account_id(&self) -> Result<()> {
        let sts_output = self.client.get_caller_identity().send().await;
        println!("{:#?}", sts_output);
        Ok(())
    }
}
