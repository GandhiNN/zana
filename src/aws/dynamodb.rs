use aws_sdk_dynamodb::types::KeyType;
use aws_sdk_dynamodb::{Client, Error};
use aws_types::SdkConfig;
use serde::Serialize;
use std::fmt;
use tabled::Tabled;

pub struct DynamoDB {
    pub client: Client,
}

#[derive(Debug)]
pub struct TableDescription {
    name: String,
    hash_key: String,
    range_key: String,
    status: String,
    creation_date: String,
    read_capacity: i64,
    write_capacity: i64,
    size: i64,
    item_count: i64,
}

impl fmt::Display for TableDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "-------------------\nTable Description\n-------------------\n 
            Table Name: {}\nHash Key: {}\nRange Key: {}\nStatus: 
            {}\nCreation Date: {}\nRead Capacity: {}\nWrite Capacity: 
            {}\nSize: {}\nItem Count: {}",
            self.name,
            self.hash_key,
            self.range_key,
            self.status,
            self.creation_date,
            self.read_capacity,
            self.write_capacity,
            self.size,
            self.item_count
        )
    }
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

    pub async fn describe_table(&self, table_name: String) -> Result<TableDescription, Error> {
        let mut table_description = TableDescription {
            name: "".to_string(),
            hash_key: "".to_string(),
            range_key: "".to_string(),
            status: "".to_string(),
            creation_date: "".to_string(),
            read_capacity: 0,
            write_capacity: 0,
            size: 0,
            item_count: 0,
        };
        let mut resp = self
            .client
            .describe_table()
            .table_name(table_name)
            .send()
            .await?;
        table_description.name = resp
            .table
            .as_mut()
            .unwrap()
            .table_name
            .as_ref()
            .unwrap()
            .to_string();
        table_description.hash_key = resp
            .table
            .as_mut()
            .unwrap()
            .key_schema
            .as_mut()
            .unwrap()
            .iter()
            .find(|&x| x.key_type == KeyType::Hash)
            .unwrap()
            .attribute_name
            .to_string();
        table_description.range_key = resp
            .table
            .as_mut()
            .unwrap()
            .key_schema
            .as_mut()
            .unwrap()
            .iter()
            .find(|&x| x.key_type == KeyType::Range)
            .unwrap()
            .attribute_name
            .to_string();
        table_description.status = resp
            .table
            .as_mut()
            .unwrap()
            .table_status
            .as_mut()
            .unwrap()
            .to_string();
        table_description.creation_date = resp
            .table
            .as_mut()
            .unwrap()
            .creation_date_time
            .as_ref()
            .unwrap()
            .to_string();
        table_description.read_capacity = resp
            .table
            .as_mut()
            .unwrap()
            .provisioned_throughput
            .as_mut()
            .unwrap()
            .read_capacity_units
            .unwrap();
        table_description.write_capacity = resp
            .table
            .as_mut()
            .unwrap()
            .provisioned_throughput
            .as_mut()
            .unwrap()
            .write_capacity_units
            .unwrap();
        table_description.size = resp.table.as_mut().unwrap().table_size_bytes.unwrap();
        table_description.item_count = resp.table.as_mut().unwrap().item_count.unwrap();

        Ok(table_description)
    }
}

#[derive(Tabled, Debug, Serialize)]
pub struct DynamoDBTable {
    table_name: String,
}
