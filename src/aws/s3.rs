use aws_sdk_s3::{Client, Error};
use aws_types::SdkConfig;
use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(Tabled, Debug)]
struct S3Bucket {
    name: String,
    created_at: String,
    region: String,
}

async fn set_client(config: SdkConfig) -> Result<Client, Error> {
    let client = Client::new(&config);
    Ok(client)
}

pub async fn list_buckets(config: SdkConfig) {
    let client = set_client(config).await.unwrap();
    let mut list_buckets = client.list_buckets().into_paginator().send();
    while let Some(list_buckets_output) = list_buckets.next().await {
        match list_buckets_output {
            Ok(list_buckets) => {
                let mut s3_bucket: Vec<S3Bucket> = Vec::new();
                let buckets = list_buckets.buckets();
                for bucket in buckets {
                    let bucket_name = bucket.name().unwrap().to_string();
                    let creation_date = bucket.creation_date().unwrap().to_string();
                    let bucket_region = bucket.bucket_region().unwrap_or("None").to_string();
                    s3_bucket.push(S3Bucket {
                        name: bucket_name,
                        created_at: creation_date,
                        region: bucket_region,
                    });
                }
                let mut table = Table::new(&s3_bucket);
                table.with(Style::psql());
                println!("{}", table);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
