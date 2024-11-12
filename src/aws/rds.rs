use aws_sdk_rds::{Client, Error};

pub async fn show_instances(client: &Client) -> Result<(), Error> {
    let instances = client.describe_db_instances().send().await?;
    for instance in instances.db_instances() {
        println!(
            "DB instance identifier: {:?}",
            instance
                .db_instance_identifier()
                .expect("Instance should have identifiers")
        )
    }
    Ok(())
}
