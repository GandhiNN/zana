use log::{error, info};
use zana::aws::{config, dynamodb, glue, rds};

#[tokio::main]
async fn main() {
    // Read from CLI arguments
    let matches = zana::cli::cmd().get_matches();

    // Parse global arguments to propagate to subcommands
    let aws_profile = matches.get_one::<String>("profile").unwrap();
    let timeout: &u64 = matches.get_one::<u64>("timeout").unwrap();
    info!("Using shared config with profile name: {}", aws_profile);
    let shared_config: aws_types::SdkConfig = config::set_config(aws_profile, *timeout).await;

    // Match subcommands input
    if let Some(matches) = matches.subcommand_matches("glue") {
        // AWS Glue logic
        if let Some(matches) = matches.subcommand_matches("jobs") {
            if matches.get_flag("list") {
                glue::list_jobs(shared_config).await;
            }
        } else if let Some(matches) = matches.subcommand_matches("databases") {
            if matches.get_flag("list") {
                glue::list_databases(shared_config).await;
            }
        } else if let Some(matches) = matches.subcommand_matches("table") {
            let db = matches.get_one::<String>("database").unwrap();
            if matches.get_flag("list") {
                glue::list_tables(shared_config, db.to_string()).await;
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("rds") {
        // AWS RDS logic
        match matches.get_one::<String>("task").unwrap().as_str() {
            "list-instances" => rds::list_instances(shared_config).await,
            _ => error!("No task provided!"),
        }
    } else if let Some(matches) = matches.subcommand_matches("dynamodb") {
        // AWS DynamoDB logic
        match matches.get_one::<String>("task").unwrap().as_str() {
            "list-tables" => dynamodb::list_tables(shared_config).await,
            _ => error!("No task provided!"),
        }
    } else {
        error!("Faulty input is provided!")
    }
}
