use aws_sdk_dynamodb::{Client, Error};
use aws_types::SdkConfig;
use serde::Serialize;
use tabled::Tabled;

pub struct DynamoDB {
    pub client: Client,
}

impl DynamoDB {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn list_tables(&self) -> Result<Vec<DynamoDBTable>, Error> {
        // Get list of tables in DynamoDB
        let mut ddb_tables: Vec<DynamoDBTable> = Vec::new();
        let mut list_tables = self.client.list_tables().into_paginator().send();
        while let Some(list_tables_output) = list_tables.next().await {
            match list_tables_output {
                Ok(list_tables) => {
                    let tables = list_tables.table_names();
                    for table in tables {
                        ddb_tables.push(DynamoDBTable {
                            table_name: table.to_string(),
                        });
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }
        Ok(ddb_tables)
    }
}

#[derive(Tabled, Debug, Serialize)]
pub struct DynamoDBTable {
    table_name: String,
}
