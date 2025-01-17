use aws_sdk_redshift::{Client, Error};
use aws_types::SdkConfig;

pub struct Redshift {
    pub client: Client,
}

impl Redshift {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn describe_clusters(&self, cluster_id: String) -> Result<(), Error> {
        let mut resp = self
            .client
            .describe_clusters()
            .cluster_identifier(cluster_id)
            .send()
            .await?;
        println!("{:?}", resp);
        Ok(())
    }
}
