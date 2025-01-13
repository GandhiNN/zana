use aws_sdk_rds::{operation::describe_db_instances::DescribeDbInstancesOutput, Client, Error};
use aws_types::SdkConfig;

use serde::Serialize;
use tabled::Tabled;

#[derive(Tabled, Debug, Serialize)]
pub struct DBInstance {
    id: String,
    class: String,
    engine: String,
    status: String,
    endpoint: String,
}

async fn set_client(config: SdkConfig) -> Result<Client, Error> {
    let client = Client::new(&config);
    Ok(client)
}

async fn get_instances(client: &Client) -> Result<DescribeDbInstancesOutput, Error> {
    Ok(client.describe_db_instances().send().await?)
}

pub async fn list_instances(config: SdkConfig) -> Result<Vec<DBInstance>, Error> {
    let client = set_client(config).await?;
    let instances_list = get_instances(&client).await;
    let mut db_instances: Vec<DBInstance> = Vec::new();
    match instances_list {
        Ok(instances) => {
            for instance in instances.db_instances() {
                db_instances.push(DBInstance {
                    id: instance.db_instance_identifier().unwrap().to_string(),
                    class: instance.db_instance_class().unwrap().to_string(),
                    engine: instance.engine().unwrap().to_string(),
                    status: instance.db_instance_status().unwrap().to_string(),
                    endpoint: instance.endpoint().unwrap().address.clone().unwrap(),
                });
            }
        }
        Err(e) => println!("{:?}", e),
    }
    Ok(db_instances)
}
