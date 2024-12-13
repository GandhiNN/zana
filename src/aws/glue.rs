use aws_sdk_glue::{Client, Error};
use aws_types::SdkConfig;
use serde::Serialize;
use tabled::Tabled;

#[derive(Tabled, Debug, Serialize)]
pub struct GlueTable {
    table_name: String,
    database_name: String,
    description: String,
    table_owner: String,
    table_create_time: String,
    table_update_time: String,
    table_last_access_time: String,
    table_avg_record_size: String,
    table_record_count: String,
    table_size_key: String,
}

#[derive(Tabled, Debug, Serialize)]
pub struct GlueDatabase {
    db_name: String,
    db_description: String,
    db_location_uri: String,
    db_create_time: String,
}

async fn set_client(config: SdkConfig) -> Result<Client, Error> {
    let client = Client::new(&config);
    Ok(client)
}

pub async fn list_tables(config: SdkConfig, database: String) -> Result<Vec<GlueTable>, Error> {
    let client = set_client(config).await.unwrap();
    let mut list_tables = client
        .get_tables()
        .database_name(database)
        .into_paginator()
        .send();
    let mut glue_tables: Vec<GlueTable> = Vec::new();
    while let Some(list_tables_output) = list_tables.next().await {
        match list_tables_output {
            Ok(list_tables) => {
                let tables = list_tables.table_list();
                println!("tableName,databaseName,description,owner,createTime,updateTime,lastAccessTime,averageRecordSizeInBytes,recordCount,sizeKeyInBytes");
                for table in tables {
                    let tbl_name = table.name().to_string();
                    let db_name = table.database_name().unwrap_or_default().to_string();
                    let desc = table.description().unwrap_or_default().to_string();
                    let owner = table.owner().unwrap_or_default().to_string();
                    let create_time = table.create_time().unwrap().to_string();
                    let update_time = table.update_time().unwrap().to_string();
                    let last_access_time = table.last_access_time().unwrap().to_string();
                    let table_params = table.storage_descriptor().unwrap().parameters().unwrap();
                    let avg_record_size =
                        table_params.get("averageRecordSize").unwrap().to_string();
                    let record_count = table_params.get("recordCount").unwrap().to_string();
                    let size_key = table_params.get("sizeKey").unwrap().to_string();
                    glue_tables.push(GlueTable {
                        table_name: tbl_name,
                        database_name: db_name,
                        description: desc,
                        table_owner: owner,
                        table_create_time: create_time,
                        table_update_time: update_time,
                        table_last_access_time: last_access_time,
                        table_record_count: record_count,
                        table_avg_record_size: avg_record_size,
                        table_size_key: size_key,
                    });
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(glue_tables)
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

pub async fn get_job_runs(config: SdkConfig, job_name: String) {
    let client = set_client(config).await.unwrap();
    let mut job_runs = client
        .get_job_runs()
        .job_name(job_name)
        .into_paginator()
        .send();
    while let Some(job_runs_output) = job_runs.next().await {
        match job_runs_output {
            Ok(job_runs) => {
                println!("{:#?}", job_runs)
            }
            Err(e) => println!("{:#?}", e),
        }
    }
}

pub async fn list_databases(config: SdkConfig) -> Result<Vec<GlueDatabase>, Error> {
    let client = set_client(config).await.unwrap();
    let mut list_databases = client.get_databases().into_paginator().send();
    let mut glue_databases: Vec<GlueDatabase> = Vec::new();
    while let Some(list_databases_output) = list_databases.next().await {
        match list_databases_output {
            Ok(list_databases) => {
                let databases = list_databases.database_list();
                for db in databases {
                    let name = db.name().to_string();
                    let description = db.description().unwrap_or_default().to_string();
                    let location_uri = db.location_uri().unwrap_or_default().to_string();
                    let create_time = db.create_time().unwrap().to_string();
                    glue_databases.push(GlueDatabase {
                        db_name: name,
                        db_description: description,
                        db_location_uri: location_uri,
                        db_create_time: create_time,
                    })
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(glue_databases)
}
