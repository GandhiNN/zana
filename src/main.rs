use zana::aws::{config, dynamodb};

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
    let shared_config: aws_types::SdkConfig = config::set_config(aws_profile, *timeout).await;

    if service == "dynamodb" {
        dynamodb::list_tables_v2(shared_config).await;
    } else {
        println!("No suitable services!")
    }
}
