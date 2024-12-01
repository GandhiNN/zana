use aws_sdk_s3::{Client, Error};
use aws_types::SdkConfig;

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
                let names = list_buckets.buckets();
                // for name in names {
                //     println!("{}", name);
                // }
                println!("{:?}", names);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
