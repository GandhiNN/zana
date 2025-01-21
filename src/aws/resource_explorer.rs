use aws_sdk_resourceexplorer2::{Client, Error};
use aws_types::SdkConfig;

pub struct ResourceExplorer {
    pub client: Client,
}

impl ResourceExplorer {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn search(&self, query_string: String) -> Result<(), Error> {
        let mut resp = self
            .client
            .search()
            .query_string(query_string)
            .into_paginator()
            .send();
        while let Some(output) = resp.next().await {
            match output {
                Ok(res) => {
                    println!("{:#?}", res);
                }
                Err(e) => println!("{:#?}", e),
            }
        }
        Ok(())
    }
}
