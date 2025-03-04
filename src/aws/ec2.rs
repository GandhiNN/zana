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

    pub async fn describe_images_owned_by_account(&self) -> Result<()> {
        let mut desc_images = self
            .client
            .describe_images()
            .owners("self")
            .into_paginator()
            .send();
        while let Some(desc_images_output) = desc_images.next().await {
            match desc_images_output {
                Ok(out) => {
                    println!("{:#?}", out)
                }
                Err(e) => println!("{:#?}", e),
            }
        }
        Ok(())
    }

    pub async fn describe_instances_owned_by_account(&self) -> Result<()> {
        let mut desc_instances = self.client.describe_instances().into_paginator().send();
        while let Some(desc_instances_output) = desc_instances.next().await {
            match desc_instances_output {
                Ok(out) => {
                    println!("{:#?}", out)
                }
                Err(e) => println!("{:#?}", e),
            }
        }
        Ok(())
    }
}
