use aws_sdk_docdb::{Client, Error};
use aws_types::SdkConfig;

pub struct DocDB {
    pub client: Client,
}

impl DocDB {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn describe_clusters(&self) -> Result<(), Error> {
        let desc_clusters = self.client.describe_db_clusters().into_paginator().send();
        println!("{:#?}", desc_clusters);
        Ok(())
    }
}
