use aws_sdk_redshift::{Client, Error};
use aws_types::SdkConfig;
use serde::Serialize;
use std::fmt;
use tabled::Tabled;

#[derive(Tabled, Default, Serialize, Debug)]
pub struct ClusterDescription {
    name: String,
    node_type: String,
    status: String,
    master_username: String,
    db_name: String,
    endpoint: String,
    create_time: String,
    encrypted: String,
}

impl fmt::Display for ClusterDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "-------------------\nCluster Description\n-------------------\n 
Cluster Name: {}\nNode Type: {}\nStatus: {}\nMaster Username: {}\nDatabase Name: {}\nEndpoint: {}\nCreate Time: {}\nIs Encrypted: {}\n",
            self.name,
            self.node_type,
            self.status,
            self.master_username,
            self.db_name,
            self.endpoint,
            self.create_time,
            self.encrypted,
        )
    }
}

pub struct Redshift {
    pub client: Client,
}

impl Redshift {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn describe_cluster(&self, cluster_id: String) -> Result<ClusterDescription, Error> {
        let mut resp = self
            .client
            .describe_clusters()
            .cluster_identifier(cluster_id)
            .into_paginator()
            .send();
        let mut cluster_desc = ClusterDescription::default();
        while let Some(output) = resp.next().await {
            match output {
                Ok(mut res) => {
                    cluster_desc.name = res.clusters.as_mut().unwrap()[0]
                        .cluster_identifier
                        .clone()
                        .unwrap();
                    cluster_desc.node_type =
                        res.clusters.as_mut().unwrap()[0].node_type.clone().unwrap();
                    cluster_desc.status = res.clusters.as_mut().unwrap()[0]
                        .cluster_status
                        .clone()
                        .unwrap();
                    cluster_desc.master_username = res.clusters.as_mut().unwrap()[0]
                        .master_username
                        .clone()
                        .unwrap();
                    cluster_desc.db_name =
                        res.clusters.as_mut().unwrap()[0].db_name.clone().unwrap();
                    let addr = res.clusters.as_mut().unwrap()[0]
                        .endpoint
                        .clone()
                        .unwrap()
                        .address
                        .unwrap();
                    let port = res.clusters.as_mut().unwrap()[0]
                        .endpoint
                        .clone()
                        .unwrap()
                        .port
                        .unwrap();
                    cluster_desc.endpoint = format!("{}:{}", addr, port);
                    cluster_desc.create_time = res.clusters.as_mut().unwrap()[0]
                        .cluster_create_time
                        .unwrap()
                        .to_string();
                    cluster_desc.encrypted = res.clusters.as_mut().unwrap()[0]
                        .encrypted
                        .unwrap()
                        .to_string();
                }
                Err(e) => println!("{:#?}", e),
            }
        }
        Ok(cluster_desc)
    }

    pub async fn describe_events(&self) -> Result<(), Error> {
        let mut resp = self.client.describe_events().into_paginator().send();
        while let Some(page) = resp.next().await {
            match page {
                Ok(res) => {
                    println!("{:#?}", res.events)
                }
                Err(e) => println!("{:#?}", e),
            }
        }
        println!("{:#?}", resp);
        Ok(())
    }
}
