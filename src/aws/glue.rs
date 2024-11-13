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
                println!("table,");
                for table in tables {
                    println!("{},", table.name())
                }
                // println!("{:#?}", tables)
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
