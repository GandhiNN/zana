use aws_sdk_glue::error::SdkError;
use aws_sdk_glue::operation::get_tables::GetTablesError;
use aws_sdk_glue::{operation::get_tables::GetTablesOutput, Client, Error};
use aws_types::SdkConfig;

async fn set_client(config: SdkConfig) -> Result<Client, Error> {
    let client = Client::new(&config);
    Ok(client)
}

async fn get_tables(
    client: &Client,
    database: String,
) -> Result<GetTablesOutput, SdkError<GetTablesError>> {
    client.get_tables().database_name(database).send().await
}

pub async fn list_tables(config: SdkConfig, database: String) {
    let client = set_client(config).await.unwrap();
    let tables = get_tables(&client, database).await;
    println!("{:#?}", tables);
}
