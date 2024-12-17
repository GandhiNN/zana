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

    // Match commands and subcommands input
    match matches.subcommand() {
        Some(("glue", sub_matches)) => {
            let glue_command = sub_matches.subcommand().unwrap();
            match glue_command {
                ("job", sub_matches) => {
                    let job_subcommands = sub_matches.subcommand().unwrap();
                    match job_subcommands {
                        ("runs", sub_matches) => {
                            let runs_subcommands = sub_matches.subcommand().unwrap();
                            match runs_subcommands {
                                ("list", flags) => {
                                    let glue_job_name =
                                        flags.get_one::<String>("jobname").unwrap().to_string();
                                    let res =
                                        glue::get_job_runs(shared_config, glue_job_name).await;
                                    if flags.get_flag("pretty") {
                                        pretty_print(res.unwrap());
                                    } else if flags.get_flag("csv") {
                                        let _ = write_csv(res.unwrap());
                                    } else {
                                        log::error!("Unknown input")
                                    }
                                }
                                _ => log::error!("Unknown input"),
                            }
                        }
                        (name, _) => {
                            unreachable!("Unknown subcommand `{name}`")
                        }
                    }
                }
                ("databases", sub_matches) => {
                    let databases_subcommands = sub_matches.subcommand().unwrap();
                    match databases_subcommands {
                        ("list", flags) => {
                            let res = glue::list_databases(shared_config).await;
                            if flags.get_flag("pretty") {
                                pretty_print(res.unwrap());
                            } else if flags.get_flag("csv") {
                                let _ = write_csv(res.unwrap());
                            } else {
                                log::error!("Unknown input")
                            }
                        }
                        _ => log::error!("Unknown input"),
                    }
                }
                ("table", sub_matches) => {
                    let table_subcommands = sub_matches.subcommand().unwrap();
                    match table_subcommands {
                        ("list", flags) => {
                            let db = flags.get_one::<String>("database").unwrap();
                            let res = glue::list_tables(shared_config, db.to_string()).await;
                            if flags.get_flag("pretty") {
                                pretty_print(res.unwrap());
                            } else if flags.get_flag("csv") {
                                let _ = write_csv(res.unwrap());
                            } else {
                                log::error!("Unknown input")
                            }
                        }
                        _ => log::error!("Unknown input"),
                    }
                }
                _ => unreachable!(),
            }
        }
        Some(("rds", sub_matches)) => {
            let rds_command = sub_matches.subcommand().unwrap();
            match rds_command {
                ("instances", flags) => {
                    let res = rds::list_instances(shared_config).await;
                    if flags.get_flag("pretty") {
                        pretty_print(res.unwrap());
                    } else if flags.get_flag("csv") {
                        let _ = write_csv(res.unwrap());
                    } else {
                        log::error!("Unknown input")
                    }
                }
                _ => log::error!("Unknown input"),
            }
        }
        Some(("s3", sub_matches)) => {
            let s3_command = sub_matches.subcommand().unwrap();
            match s3_command {
                ("bucket", sub_matches) => {
                    let bucket_subcommands = sub_matches.subcommand().unwrap();
                    match bucket_subcommands {
                        ("list", flags) => {
                            let res = s3::list_buckets(shared_config).await;
                            if flags.get_flag("pretty") {
                                pretty_print(res.unwrap());
                            } else if flags.get_flag("csv") {
                                let _ = write_csv(res.unwrap());
                            } else {
                                log::error!("Unknown input")
                            }
                        }
                        _ => log::error!("Unknown input"),
                    }
                }
                ("objects", sub_matches) => {
                    let objects_subcommands = sub_matches.subcommand().unwrap();
                    match objects_subcommands {
                        ("list", flags) => {
                            let bucket = flags.get_one::<String>("bucket").unwrap();
                            let res = s3::list_objects(shared_config, bucket.to_string()).await;
                            if flags.get_flag("pretty") {
                                pretty_print(res.unwrap());
                            } else if flags.get_flag("csv") {
                                let _ = write_csv(res.unwrap());
                            } else {
                                log::error!("Unknown input")
                            }
                        }
                        ("versions", flags) => {
                            let bucket = flags.get_one::<String>("bucket").unwrap();
                            let res =
                                s3::list_objects_versions(shared_config, bucket.to_string()).await;
                            if flags.get_flag("pretty") {
                                pretty_print(res.unwrap());
                            } else if flags.get_flag("csv") {
                                let _ = write_csv(res.unwrap());
                            } else {
                                log::error!("Unknown input")
                            }
                        }
                        _ => log::error!("Unknown input"),
                    }
                }
                _ => log::error!("Unknown input"),
            }
        }
        Some(("dynamodb", flags)) => match flags.get_one::<String>("task").unwrap().as_str() {
            "list-tables" => dynamodb::list_tables(shared_config).await,
            _ => log::error!("No task provided!"),
        },
        _ => log::error!("Faulty input is provided!"),
    }
}
