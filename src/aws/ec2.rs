use anyhow::Result;
use aws_sdk_ec2::Client;
use aws_types::SdkConfig;

pub struct Ec2 {
    pub client: Client,
}

impl Ec2 {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn describe_volumes(&self) -> Result<()> {
        let mut desc_volumes = self.client.describe_volumes().into_paginator().send();
        while let Some(desc_volumes_output) = desc_volumes.next().await {
            match desc_volumes_output {
                Ok(out) => {
                    println!("{:#?}", out)
                }
                Err(e) => println!("{:#?}", e),
            }
        }
        Ok(())
    }
}
