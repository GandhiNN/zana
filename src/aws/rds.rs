use aws_sdk_rds::{Client, Error};
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

pub struct RDS {
    pub client: Client,
}

impl RDS {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn list_instances(&self) -> Result<Vec<DBInstance>, Error> {
        let mut db_instances: Vec<DBInstance> = Vec::new();
        let instances_list = self.client.describe_db_instances().send().await;
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
}
