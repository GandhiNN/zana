use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use zana::aws::{config, dynamodb, glue, rds, s3};
use zana::util::{pretty_print, write_csv};

#[tokio::main]
async fn main() {
    // Setup logger
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] = {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    // Read from CLI arguments
    let matches = zana::cli::cmd().get_matches();

    // Parse global arguments to propagate to subcommands
    let aws_profile = matches.get_one::<String>("profile").unwrap();
    let timeout: &u64 = matches.get_one::<u64>("timeout").unwrap();
    log::info!("Using shared config with profile name: {}", aws_profile);
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
            _ => log::error!("No task provided!"),
        }
    } else if let Some(matches) = matches.subcommand_matches("dynamodb") {
        // AWS DynamoDB logic
        match matches.get_one::<String>("task").unwrap().as_str() {
            "list-tables" => dynamodb::list_tables(shared_config).await,
            _ => log::error!("No task provided!"),
        }
    } else if let Some(matches) = matches.subcommand_matches("s3") {
        // AWS S3 logic
        if let Some(matches) = matches.subcommand_matches("bucket") {
            if matches.get_flag("list") {
                let res = s3::list_buckets(shared_config).await;
                if matches.get_flag("pretty") {
                    pretty_print(res.unwrap());
                } else {
                    let _ = write_csv(res.unwrap());
                }
            }
        } else if let Some(matches) = matches.subcommand_matches("objects") {
            let bucket = matches.get_one::<String>("bucket").unwrap();
            if matches.get_flag("list") {
                let res = s3::list_objects(shared_config, bucket.to_string()).await;
                if matches.get_flag("pretty") {
                    pretty_print(res.unwrap());
                } else {
                    let _ = write_csv(res.unwrap());
                }
            } else if matches.get_flag("versions") {
                let res = s3::list_objects_versions(shared_config, bucket.to_string()).await;
                if matches.get_flag("pretty") {
                    pretty_print(res.unwrap());
                } else {
                    let _ = write_csv(res.unwrap());
                }
            }
        }
    } else {
        log::error!("Faulty input is provided!")
    }
}
