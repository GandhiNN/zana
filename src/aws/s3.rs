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
#[derive(Tabled, Debug)]
struct S3Object {
    obj_key: String,
    obj_last_modified_at: String,
    obj_etag: String,
    obj_size: i64,
    obj_storage_class: String,
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

pub async fn list_objects(config: SdkConfig, bucket: String) {
    let client = set_client(config).await.unwrap();
    let mut list_objects = client
        .list_objects_v2()
        .bucket(bucket)
        .into_paginator()
        .send();
    while let Some(list_objects_v2_output) = list_objects.next().await {
        match list_objects_v2_output {
            Ok(list_objects) => {
                let objects = list_objects.contents();
                let mut s3_objects: Vec<S3Object> = Vec::new();
                for object in objects {
                    let key = object.key().unwrap().to_string();
                    let last_modified = object.last_modified().unwrap().to_string();
                    let etag = object.e_tag().unwrap().to_string();
                    let size = object.size().unwrap();
                    let storage_class = object.storage_class().unwrap().to_string();
                    s3_objects.push(S3Object {
                        obj_key: key,
                        obj_last_modified_at: last_modified,
                        obj_etag: etag,
                        obj_size: size,
                        obj_storage_class: storage_class,
                    });
                }
                let mut table = Table::new(&s3_objects);
                table.with(Style::psql());
                println!("{}", table);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

pub async fn list_objects_versions(config: SdkConfig, bucket: String) {
    let client = set_client(config).await.unwrap();
    let list_objects_versions = client.list_object_versions().bucket(bucket).send().await;
    println!("{:#?}", list_objects_versions)
}
