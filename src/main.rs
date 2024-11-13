#[allow(dead_code)]
use zana::aws::{config, dynamodb, glue, rds};

#[tokio::main]
async fn main() {
    // Read from CLI arguments
    let matches = zana::cli::cmd().get_matches();

    // Match subcommands input
    if let Some(matches) = matches.subcommand_matches("glue") {
        // AWS Glue tasks
        // Parse arguments
        let aws_profile = matches.get_one::<String>("profile").unwrap();
        let timeout: &u64 = matches.get_one::<u64>("timeout").unwrap();
        // Build aws config
        println!("Using shared config with profile name: {}", aws_profile);
        let shared_config: aws_types::SdkConfig = config::set_config(aws_profile, *timeout).await;
        match matches.get_one::<String>("task").unwrap().as_str() {
            "list-tables" => glue::list_tables(shared_config, "lesmes-ro01".to_string()).await,
            _ => println!("No task provided!"),
        }
    } else if let Some(matches) = matches.subcommand_matches("rds") {
        // AWS RDS tasks
        // Parse arguments
        let aws_profile = matches.get_one::<String>("profile").unwrap();
        let timeout: &u64 = matches.get_one::<u64>("timeout").unwrap();
        // Build aws config
        println!("Using shared config with profile name: {}", aws_profile);
        let shared_config: aws_types::SdkConfig = config::set_config(aws_profile, *timeout).await;
        match matches.get_one::<String>("task").unwrap().as_str() {
            "list-instances" => rds::list_instances(shared_config).await,
            _ => println!("No task provided!"),
        }
    } else if let Some(matches) = matches.subcommand_matches("dynamodb") {
        // AWS DynamoDB tasks
        // Parse arguments
        let aws_profile = matches.get_one::<String>("profile").unwrap();
        let timeout: &u64 = matches.get_one::<u64>("timeout").unwrap();
        // Build aws config
        println!("Using shared config with profile name: {}", aws_profile);
        let shared_config: aws_types::SdkConfig = config::set_config(aws_profile, *timeout).await;
        match matches.get_one::<String>("task").unwrap().as_str() {
            "list-tables" => dynamodb::list_tables(shared_config).await,
            _ => println!("No task provided!"),
        }
    } else {
        eprintln!("Faulty input is provided!")
    }
}
