use anyhow::Result;
use aws_sdk_glue::{Client, Error};
use aws_types::SdkConfig;
use serde::Serialize;
use std::fmt;
use tabled::Tabled;

#[derive(Tabled, Debug, Serialize)]
pub struct GlueJobNames {
    job_name: String,
}

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

#[derive(Tabled, Debug, Serialize)]
pub struct GlueJobRun {
    name: String,
    state: String,
    dpu_seconds: f64,
}

pub struct GlueJobBookmark {
    job_name: String,
    version: String,
    run: String,
    attempt: String,
    run_id: String,
    bookmark: String,
}

impl fmt::Display for GlueJobBookmark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "-------------------\nJob Bookmark Description\n-------------------\n 
Job Name: {}\nVersion: {}\nRun: {}\nAttempt: {}\nRun ID: {}\nDescription: {}\n",
            self.job_name, self.version, self.run, self.attempt, self.run_id, self.bookmark,
        )
    }
}

async fn set_client(config: SdkConfig) -> Result<Client, Error> {
    let client = Client::new(&config);
    Ok(client)
}

pub async fn list_tables(config: SdkConfig, database: &str) -> Result<Vec<GlueTable>, Error> {
    let client = set_client(config).await?;
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
                for table in tables {
                    let table_params = table.storage_descriptor().unwrap().parameters().unwrap();
                    glue_tables.push(GlueTable {
                        table_name: table.name().to_string(),
                        database_name: table.database_name().unwrap_or_default().to_string(),
                        description: table.description().unwrap_or_default().to_string(),
                        table_owner: table.owner().unwrap_or_default().to_string(),
                        table_create_time: table.create_time().unwrap().to_string(),
                        table_update_time: table.update_time().unwrap().to_string(),
                        table_last_access_time: table.last_access_time().unwrap().to_string(),
                        table_record_count: table_params
                            .get("averageRecordSize")
                            .unwrap()
                            .to_string(),
                        table_avg_record_size: table_params.get("recordCount").unwrap().to_string(),
                        table_size_key: table_params.get("sizeKey").unwrap().to_string(),
                    });
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(glue_tables)
}

pub async fn list_jobs(config: SdkConfig) -> Result<Vec<GlueJobNames>, Error> {
    let client = set_client(config).await?;
    let mut list_jobs = client.list_jobs().into_paginator().send();
    let mut glue_job_names: Vec<GlueJobNames> = Vec::new();
    while let Some(list_jobs_output) = list_jobs.next().await {
        match list_jobs_output {
            Ok(list_jobs) => {
                let names = list_jobs.job_names();

                for name in names {
                    glue_job_names.push(GlueJobNames {
                        job_name: name.to_owned(),
                    });
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(glue_job_names)
}

pub async fn get_job_runs(config: SdkConfig, job_name: &str) -> Result<Vec<GlueJobRun>, Error> {
    let client = set_client(config).await?;
    let mut job_runs = client
        .get_job_runs()
        .job_name(job_name)
        .into_paginator()
        .send();
    let mut glue_job_runs: Vec<GlueJobRun> = Vec::new();
    while let Some(job_runs_output) = job_runs.next().await {
        match job_runs_output {
            Ok(v) => {
                let runs = v.job_runs();
                for run in runs {
                    glue_job_runs.push(GlueJobRun {
                        name: run.job_name().unwrap().to_string(),
                        state: run.job_run_state().unwrap().to_string(),
                        dpu_seconds: run.dpu_seconds().unwrap_or_default(),
                    })
                }
            }
            Err(e) => println!("{:#?}", e),
        }
    }
    Ok(glue_job_runs)
}

pub async fn get_job_bookmark(config: SdkConfig, job_name: &str) -> Result<(), Error> {
    let client = set_client(config).await?;
    let job_bookmark = client.get_job_bookmark().job_name(job_name).send().await?;
    let bookmark_output = GlueJobBookmark {
        job_name: job_bookmark
            .clone()
            .job_bookmark_entry
            .unwrap()
            .job_name
            .unwrap()
            .to_string(),
        version: job_bookmark
            .clone()
            .job_bookmark_entry
            .unwrap()
            .version
            .to_string(),
        run: job_bookmark
            .clone()
            .job_bookmark_entry
            .unwrap()
            .run
            .to_string(),
        attempt: job_bookmark
            .clone()
            .job_bookmark_entry
            .unwrap()
            .attempt
            .to_string(),
        run_id: job_bookmark
            .clone()
            .job_bookmark_entry
            .unwrap()
            .run_id
            .unwrap()
            .to_string(),
        bookmark: job_bookmark
            .clone()
            .job_bookmark_entry
            .unwrap()
            .job_bookmark
            .unwrap()
            .to_string(),
    };
    println!("{}", bookmark_output);
    Ok(())
}

pub async fn list_databases(config: SdkConfig) -> Result<Vec<GlueDatabase>, Error> {
    let client = set_client(config).await?;
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
