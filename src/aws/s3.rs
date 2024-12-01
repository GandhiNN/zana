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

#[derive(Tabled, Debug)]
struct S3ObjectVersion {
    obj_key: String,
    obj_version_id: String,
    obj_is_latest: bool,
    obj_last_modified_date: String,
    obj_size: i64,
    obj_etag: String,
    obj_owner: String,
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

pub async fn list_objects_versions(config: SdkConfig, bucket: String) -> Result<(), Error> {
    let client = set_client(config).await.unwrap();
    let list_objects_versions = client.list_object_versions().bucket(bucket).send().await?;
    let mut s3_object_version: Vec<S3ObjectVersion> = Vec::new();
    for version in list_objects_versions.versions() {
        let etag = version.e_tag().unwrap_or_default().to_string();
        let size = version.size().unwrap_or_default();
        let key = version.key().unwrap_or_default().to_string();
        let version_id = version.version_id().unwrap_or_default().to_string();
        let is_latest = version.is_latest().unwrap();
        let last_modified = version.last_modified().unwrap().to_string();
        let owner = version
            .owner()
            .unwrap()
            .display_name()
            .unwrap_or_default()
            .to_string();

        s3_object_version.push(S3ObjectVersion {
            obj_key: key,
            obj_version_id: version_id,
            obj_is_latest: is_latest,
            obj_last_modified_date: last_modified,
            obj_size: size,
            obj_etag: etag,
            obj_owner: owner,
        });
        let mut table = Table::new(&s3_object_version);
        table.with(Style::psql());
        println!("{}", table);
    }
    Ok(())
}
