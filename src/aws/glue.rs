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
                let names = list_tables.table_list();
                println!("{:#?}", names)
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
                println!("{:#?}", names)
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
