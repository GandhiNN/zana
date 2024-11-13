use aws_sdk_glue::{Client, Error};
use aws_types::SdkConfig;

async fn set_client(config: SdkConfig) -> Result<Client, Error> {
    let client = Client::new(&config);
    Ok(client)
}

pub async fn list_tables(config: SdkConfig, database: String) {
    let client = set_client(config).await.unwrap();
    let mut list_tables = client
        .get_tables()
        .database_name(database)
        .into_paginator()
        .send();
    while let Some(list_tables_output) = list_tables.next().await {
        match list_tables_output {
            Ok(list_tables) => {
                let tables = list_tables.table_list();
                println!("tableName,databaseName,description,owner,createTime,updateTime,lastAccessTime,averageRecordSizeInBytes,recordCount,sizeKeyInBytes");
                for table in tables {
                    let table_name = table.name();
                    let database_name = table.database_name().unwrap_or_default();
                    let description = table.description().unwrap_or_default();
                    let owner = table.owner().unwrap_or_default();
                    let create_time = table.create_time().unwrap();
                    let update_time = table.update_time().unwrap();
                    let last_access_time = table.last_access_time().unwrap();
                    let table_params = table.storage_descriptor().unwrap().parameters().unwrap();
                    let avg_record_size = table_params.get("averageRecordSize").unwrap();
                    let record_count = table_params.get("recordCount").unwrap();
                    let size_key = table_params.get("sizeKey").unwrap();
                    println!(
                        "{},{},{},{},{},{},{},{},{},{}",
                        table_name,
                        database_name,
                        description,
                        owner,
                        create_time,
                        update_time,
                        last_access_time,
                        record_count,
                        avg_record_size,
                        size_key
                    );
                }
                // println!("{:#?}", tables);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

pub async fn list_jobs(config: SdkConfig) {
    let client = set_client(config).await.unwrap();
    let mut list_jobs = client.list_jobs().into_paginator().send();
    while let Some(list_jobs_output) = list_jobs.next().await {
        match list_jobs_output {
            Ok(list_jobs) => {
                let names = list_jobs.job_names();
                for name in names {
                    println!("{}", name);
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

pub async fn list_databases(config: SdkConfig) {
    let client = set_client(config).await.unwrap();
    let mut list_databases = client.get_databases().into_paginator().send();
    while let Some(list_databases_output) = list_databases.next().await {
        match list_databases_output {
            Ok(list_databases) => {
                let databases = list_databases.database_list();
                println!("dbName,description,locationURI,createTime");
                for db in databases {
                    let name = db.name();
                    let description = db.description().unwrap_or_default();
                    let location_uri = db.location_uri().unwrap_or_default();
                    let create_time = db.create_time().unwrap();
                    println!("{},{},{},{}", name, description, location_uri, create_time);
                }
                // println!("{:#?}", databases)
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
