use aws_sdk_docdb::{Client, Error};
use aws_types::SdkConfig;
use std::fmt;

#[derive(Default, Debug)]
pub struct ClusterProperty {
    id: String,
    arn: String,
    status: String,
    engine: String,
    engine_version: String,
    endpoint: String,
    port: String,
    master_username: String,
    storage_encrypted: String,
}

impl fmt::Display for ClusterProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "-------------------\nCluster Property\n-------------------\n 
Cluster ID: {}\nARN: {}\nStatus: {}\nEngine: {}\nVersion: {}\nEndpoint: {}\nPort: {}\nMaster Username: {}\nIs Storage Encrypted: {}\n",
            self.id,
            self.arn,
            self.status,
            self.engine,
            self.engine_version,
            self.endpoint,
            self.port,
            self.master_username,
            self.storage_encrypted
        )
    }
}

pub struct DocDB {
    pub client: Client,
}

impl DocDB {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn describe_clusters(&self) -> Result<(), Error> {
        let mut desc_clusters = self.client.describe_db_clusters().into_paginator().send();
        while let Some(output) = desc_clusters.next().await {
            println!("{:#?}", output);
        }
        Ok(())
    }

    pub async fn describe_cluster(&self, id: &str) -> Result<ClusterProperty, Error> {
        let mut desc_cluster = self
            .client
            .describe_db_clusters()
            .db_cluster_identifier(id)
            .into_paginator()
            .send();
        let mut cluster_property = ClusterProperty::default();
        while let Some(output) = desc_cluster.next().await {
            match output {
                Ok(res) => {
                    let cluster_info_cloned = res.db_clusters.as_ref().unwrap()[0].clone();
                    cluster_property.id = cluster_info_cloned
                        .db_cluster_identifier
                        .unwrap_or_default();
                    cluster_property.arn = cluster_info_cloned.db_cluster_arn.unwrap_or_default();
                    cluster_property.status = cluster_info_cloned.status.unwrap_or_default();
                    cluster_property.engine = cluster_info_cloned.engine.unwrap_or_default();
                    cluster_property.engine_version =
                        cluster_info_cloned.engine_version.unwrap_or_default();
                    cluster_property.endpoint = cluster_info_cloned.endpoint.unwrap_or_default();
                    cluster_property.port =
                        cluster_info_cloned.port.unwrap_or_default().to_string();
                    cluster_property.master_username =
                        cluster_info_cloned.master_username.unwrap_or_default();
                    cluster_property.storage_encrypted = cluster_info_cloned
                        .storage_encrypted
                        .unwrap_or_default()
                        .to_string();
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(cluster_property)
    }
}
