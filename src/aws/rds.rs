use aws_sdk_rds::{operation::describe_db_instances::DescribeDbInstancesOutput, Client, Error};
use aws_types::SdkConfig;

async fn set_client(config: SdkConfig) -> Result<Client, Error> {
    let client = Client::new(&config);
    Ok(client)
}

async fn get_instances(client: &Client) -> Result<DescribeDbInstancesOutput, Error> {
    Ok(client.describe_db_instances().send().await?)
}

pub async fn list_instances(config: SdkConfig) {
    let client = set_client(config).await.unwrap();
    let instances_list = get_instances(&client).await;
    match instances_list {
        Ok(instances) => {
            println!("id,class,engine,status,endpoint",);
            for instance in instances.db_instances() {
                let id = instance.db_instance_identifier().unwrap();
                let class = instance.db_instance_class().unwrap();
                let engine = instance.engine().unwrap();
                let status = instance.db_instance_status().unwrap();
                let endpoint = instance.endpoint().unwrap().address.clone().unwrap();
                println!("{},{},{},{},{}", id, class, engine, status, endpoint);
            }
        }
        Err(e) => println!("{:?}", e),
    }
}
