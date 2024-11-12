use zana::aws::{config, dynamodb, glue, rds};

#[tokio::main]
async fn main() {
    // Read from CLI arguments
    let matches = zana::cli::cmd().get_matches();

    // Parse arguments
    let aws_profile = matches.get_one::<String>("profile").unwrap();
    let timeout: &u64 = matches.get_one::<u64>("timeout").unwrap();
    let service = matches.get_one::<String>("service").unwrap();

    // Build aws config
    println!("Using shared config with profile name: {}", aws_profile);
    println!();
    let shared_config: aws_types::SdkConfig = config::set_config(aws_profile, *timeout).await;

    // Match service name input
    // Convert &String to &str for match case statement
    match service.as_str() {
        "dynamodb" => dynamodb::list_tables(shared_config).await,
        "rds" => rds::list_instances(shared_config).await,
        "glue" => glue::list_tables(shared_config, "imel-compacted".to_owned()).await,
        _ => println!("No suitable services!"),
    }
}
