use aws_sdk_rds::{operation::describe_db_instances::DescribeDbInstancesOutput, Client, Error};
use aws_types::SdkConfig;

use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(Tabled, Debug)]
struct DBInstance {
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

pub async fn list_instances(config: SdkConfig) {
    let client = set_client(config).await.unwrap();
    let instances_list = get_instances(&client).await;
    match instances_list {
        Ok(instances) => {
            let mut db_instances: Vec<DBInstance> = Vec::new();
            // println!("id,class,engine,status,endpoint");
            for instance in instances.db_instances() {
                let db_id = instance.db_instance_identifier().unwrap().to_string();
                let db_class = instance.db_instance_class().unwrap().to_string();
                let db_engine = instance.engine().unwrap().to_string();
                let db_status = instance.db_instance_status().unwrap().to_string();
                let db_endpoint = instance.endpoint().unwrap().address.clone().unwrap();
                // println!("{},{},{},{},{}", id, class, engine, status, endpoint);
                db_instances.push(DBInstance {
                    id: db_id,
                    class: db_class,
                    engine: db_engine,
                    status: db_status,
                    endpoint: db_endpoint,
                });
            }
            let mut table = Table::new(&db_instances);
            table.with(Style::psql());
            println!("{}", table);
        }
        Err(e) => println!("{:?}", e),
    }
}
