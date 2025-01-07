use aws_sdk_dynamodb::{Client, Error};
use aws_types::SdkConfig;
use serde::Serialize;
use tabled::Tabled;

#[derive(Tabled, Debug, Serialize)]
pub struct DynamoDBTable {
    table_name: String,
}

async fn set_client(config: SdkConfig) -> Result<Client, Error> {
    let client = Client::new(&config);
    Ok(client)
}

pub async fn list_tables(config: SdkConfig) -> Result<Vec<DynamoDBTable>, Error> {
    // Get list of tables in DynamoDB
    let client = set_client(config).await.unwrap();
    let mut ddb_tables: Vec<DynamoDBTable> = Vec::new();
    let mut list_tables = client.list_tables().into_paginator().send();
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

// pub async fn list_tables(config: SdkConfig) {
//     let client = set_client(config).await.unwrap();
//     let tables = get_tables(&client).await;
//     match tables {
//         Err(e) => println!("{:?}", e),
//         _ => {
//             for table in tables.into_iter() {
//                 println!("{:?}", table)
//             }
//         }
//     }
// }
