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

    pub async fn describe_cluster(&self, cluster_id: &str) -> Result<ClusterDescription, Error> {
        let mut resp = self
            .client
            .describe_clusters()
            .cluster_identifier(cluster_id)
            .into_paginator()
            .send();
        let mut cluster_desc = ClusterDescription::default();
        while let Some(output) = resp.next().await {
            match output {
                Ok(res) => {
                    // println!("{:#?}", res.clusters.unwrap());
                    let _ = res
                        .clusters
                        .unwrap()
                        .into_iter()
                        .map(|r| {
                            cluster_desc.name = r.cluster_identifier.clone().unwrap();
                            cluster_desc.node_type = r.node_type.clone().unwrap();
                            cluster_desc.status = r.cluster_status.clone().unwrap();
                            cluster_desc.master_username = r.master_username.clone().unwrap();
                            cluster_desc.db_name = r.db_name.clone().unwrap();
                            let addr = r.endpoint.clone().unwrap().address.unwrap();
                            let port = r.endpoint.clone().unwrap().port.unwrap();
                            cluster_desc.endpoint = format!("{}:{}", addr, port);
                            cluster_desc.create_time = r.cluster_create_time.unwrap().to_string();
                            cluster_desc.encrypted = r.encrypted.unwrap().to_string();
                        })
                        .collect::<Vec<()>>(); // Collect to execute the map but ignore the result
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
