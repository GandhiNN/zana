use aws_sdk_glue::{Client, Error};
use aws_types::SdkConfig;

async fn set_client(config: SdkConfig) -> Result<Client, Error> {
    let client = Client::new(&config);
    Ok(client)
}

async fn get_tables(client: &Client) -> Result<Vec<String>, Error> {
    // Get list of tables in DynamoDB
    let mut table_names: Vec<String> = Vec::new();
    let response = client.list_tables().send().await?;
    let names = response.table_names();
    for name in names {
        table_names.push(name.to_string());
    }
    Ok(table_names)
}

pub async fn list_tables(config: SdkConfig) {
    let client = set_client(config).await.unwrap();
    let tables = get_tables(&client).await;
    match tables {
        Err(e) => println!("{:?}", e),
        _ => {
            for table in tables.into_iter() {
                println!("{:?}", table)
            }
        }
    }
}
