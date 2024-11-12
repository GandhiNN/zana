use aws_sdk_dynamodb::{Client, Error};
use aws_types::SdkConfig;

async fn set_client(config: SdkConfig) -> Result<Client, Error> {
    let client = Client::new(&config);
    Ok(client)
}

async fn list_tables(client: &Client) -> Result<Vec<String>, Error> {
    // Get list of tables in DynamoDB
    let mut table_names: Vec<String> = Vec::new();
    let response = client.list_tables().send().await?;
    let names = response.table_names();
    for name in names {
        table_names.push(name.to_string());
    }
    Ok(table_names)
}

pub async fn list_tables_v2(config: SdkConfig) {
    let client = set_client(config).await.unwrap();
    let tables = list_tables(&client).await;
    match tables {
        Err(e) => println!("{:?}", e),
        _ => {
            for table in tables.into_iter() {
                println!("{:?}", table)
            }
        }
    }
}
