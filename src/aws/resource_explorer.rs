use aws_sdk_resourceexplorer2::{Client, Error};
use aws_types::SdkConfig;
use serde::Serialize;
use tabled::Tabled;

pub struct ResourceExplorer {
    pub client: Client,
}

#[derive(Tabled, Default, Serialize, Debug)]
pub struct ResourceDescription {
    arn: String,
    owning_account_id: String,
    region: String,
    service: String,
    resource_type: String,
}

impl ResourceExplorer {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn search(&self, query_string: String) -> Result<Vec<ResourceDescription>, Error> {
        let mut resp = self
            .client
            .search()
            .query_string(query_string)
            .into_paginator()
            .send();
        let mut resource_descriptions: Vec<ResourceDescription> = Vec::new();
        while let Some(output) = resp.next().await {
            match output {
                Ok(res) => {
                    let resources = res.resources.unwrap();
                    for resource in resources {
                        resource_descriptions.push(ResourceDescription {
                            arn: resource.arn.clone().unwrap(),
                            owning_account_id: resource.owning_account_id.clone().unwrap(),
                            region: resource.region.clone().unwrap(),
                            service: resource.service.clone().unwrap(),
                            resource_type: resource.resource_type.clone().unwrap(),
                        });
                    }
                }
                Err(e) => println!("{:#?}", e),
            }
        }
        Ok(resource_descriptions)
    }
}
