use aws_sdk_s3::{Client, Error};
use aws_types::SdkConfig;
use serde::Serialize;
use tabled::Tabled;

#[derive(Tabled, Debug, Serialize)]
pub struct S3Bucket {
    name: String,
    created_at: String,
    region: String,
}
#[derive(Tabled, Debug, Serialize)]
pub struct S3Object {
    obj_key: String,
    obj_last_modified_at: String,
    obj_etag: String,
    obj_size: i64,
    obj_storage_class: String,
}

#[derive(Tabled, Debug, Serialize)]
pub struct S3ObjectVersion {
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

pub async fn list_buckets(config: SdkConfig) -> Result<Vec<S3Bucket>, Error> {
    let client = set_client(config).await?;
    let mut list_buckets = client.list_buckets().into_paginator().send();
    let mut s3_bucket: Vec<S3Bucket> = Vec::new();
    while let Some(list_buckets_output) = list_buckets.next().await {
        match list_buckets_output {
            Ok(list_buckets) => {
                let buckets = list_buckets.buckets();
                for bucket in buckets {
                    s3_bucket.push(S3Bucket {
                        name: bucket.name().unwrap().to_string(),
                        created_at: bucket.creation_date().unwrap().to_string(),
                        region: bucket.bucket_region().unwrap_or("None").to_string(),
                    });
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(s3_bucket)
}

pub async fn list_objects(config: SdkConfig, bucket: &str) -> Result<Vec<S3Object>, Error> {
    let client = set_client(config).await?;
    let mut list_objects = client
        .list_objects_v2()
        .bucket(bucket)
        .into_paginator()
        .send();
    let mut s3_objects: Vec<S3Object> = Vec::new();
    while let Some(list_objects_v2_output) = list_objects.next().await {
        match list_objects_v2_output {
            Ok(list_objects) => {
                let objects = list_objects.contents();
                for object in objects {
                    s3_objects.push(S3Object {
                        obj_key: object.key().unwrap().to_string(),
                        obj_last_modified_at: object.last_modified().unwrap().to_string(),
                        obj_etag: object.e_tag().unwrap().to_string(),
                        obj_size: object.size().unwrap(),
                        obj_storage_class: object.storage_class().unwrap().to_string(),
                    });
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(s3_objects)
}

pub async fn list_objects_versions(
    config: SdkConfig,
    bucket: &str,
) -> Result<Vec<S3ObjectVersion>, Error> {
    let client = set_client(config).await?;
    let list_objects_versions = client.list_object_versions().bucket(bucket).send().await?;
    let mut s3_object_version: Vec<S3ObjectVersion> = Vec::new();
    for version in list_objects_versions.versions() {
        s3_object_version.push(S3ObjectVersion {
            obj_key: version.key().unwrap_or_default().to_string(),
            obj_version_id: version.version_id().unwrap_or_default().to_string(),
            obj_is_latest: version.is_latest().unwrap(),
            obj_last_modified_date: version.last_modified().unwrap().to_string(),
            obj_size: version.size().unwrap_or_default(),
            obj_etag: version.e_tag().unwrap_or_default().to_string(),
            obj_owner: version
                .owner()
                .unwrap()
                .display_name()
                .unwrap_or_default()
                .to_string(),
        });
    }
    Ok(s3_object_version)
}
