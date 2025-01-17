use crate::aws::config::{AWSConfigFile, AWSCredentialsConfig};
use crate::aws::dynamodb::DynamoDB;
use crate::aws::rds::RDS;
use crate::aws::redshift::Redshift;
use crate::aws::{config, glue, s3};
use crate::cli;
use crate::util::{pretty_print, write_csv};
use tracing::{error, info};

#[allow(dead_code)]
use clap::{arg, command, value_parser, Command};

pub fn cmd() -> Command {
    command!()
        .version("0.1.0")
        .about("A CLI tool to interact with AWS environment")
        .author("Ngakan Gandhi <ngandhi.pmintl.net>")
        .bin_name("zana")
        .arg(
            arg!(--profile <VALUE> "AWS profile name to use")
                .value_parser(value_parser!(String))
                .default_value("default")
                .global(true),
        )
        .arg(
            arg!(--timeout <VALUE> "AWS SDK timeout in milliseconds")
                .value_parser(value_parser!(u64))
                .default_value("10000")
                .global(true),
        )
        .subcommand(
            Command::new("glue")
                .about("Glue API")
                .subcommand(
                    Command::new("job")
                        .about("Glue job API")
                        .subcommand(
                            Command::new("runs").about("Glue job runs API").subcommand(
                                Command::new("list")
                                    .arg(arg!(-j --jobname <VALUE> "Glue job name").required(true))
                                    .arg(arg!(-p --pretty "Pretty print output"))
                                    .arg(arg!(-c --csv "Print output as CSV"))
                                    .arg_required_else_help(true),
                            ),
                        )
                        .subcommand(
                            Command::new("list")
                                .about("List available Glue jobs")
                                .arg(arg!(-p --pretty "Pretty print output"))
                                .arg(arg!(-c --csv "Print output as CSV"))
                                .arg_required_else_help(true),
                        ),
                )
                .subcommand(
                    Command::new("databases")
                        .about("Glue databases API")
                        .subcommand(
                            Command::new("list")
                                .arg(arg!(-p --pretty "Pretty print output"))
                                .arg(arg!(-c --csv "Print output as CSV"))
                                .arg_required_else_help(true),
                        ),
                )
                .subcommand(
                    Command::new("table").about("Glue table API").subcommand(
                        Command::new("list")
                            .arg(arg!(-d --database <VALUE> "Database name").required(true))
                            .arg(arg!(-p --pretty "Pretty print output"))
                            .arg(arg!(-c --csv "Print output as CSV"))
                            .arg_required_else_help(true),
                    ),
                ),
        )
        .subcommand(
            Command::new("s3")
                .about("S3 API")
                .subcommand(
                    Command::new("bucket").about("S3 bucket API").subcommand(
                        Command::new("list")
                            .arg(arg!(-p --pretty "Pretty print output"))
                            .arg(arg!(-c --csv "Print output as CSV"))
                            .arg_required_else_help(true),
                    ),
                )
                .subcommand(
                    Command::new("objects").about("S3 objects API").subcommand(
                        Command::new("list")
                            .arg(arg!(-b --bucket <VALUE> "input S3 bucket").required(true))
                            .arg(arg!(-v --versions "List objects versions in an S3 bucket"))
                            .arg(arg!(-p --pretty "Pretty print output"))
                            .arg(arg!(-c --csv "Print output as CSV"))
                            .arg_required_else_help(true),
                    ),
                ),
        )
        .subcommand(
            Command::new("rds").about("RDS API").subcommand(
                Command::new("instances")
                    .about("RDS Instances API")
                    .subcommand(
                        Command::new("list")
                            .arg(arg!(-p --pretty "Pretty print output"))
                            .arg(arg!(-c --csv "Print output as CSV"))
                            .arg_required_else_help(true),
                    ),
            ),
        )
        .subcommand(
            Command::new("dynamodb")
                .about("DynamoDB API")
                .subcommand(
                    Command::new("tables")
                        .about("DynamoDB Tables API")
                        .subcommand(
                            Command::new("list")
                                .arg(arg!(-p --pretty "Pretty print output"))
                                .arg(arg!(-c --csv "Print output as CSV"))
                                .arg_required_else_help(true),
                        ),
                )
                .subcommand(
                    Command::new("table")
                        .about("DynamoDB Table API")
                        .subcommand(
                            Command::new("describe")
                                .arg(arg!(--table_name <VALUE> "DynamoDB table name"))
                                .arg_required_else_help(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("redshift").about("Redshift API").subcommand(
                Command::new("cluster")
                    .about("Redshift cluster API")
                    .subcommand(
                        Command::new("describe")
                            .arg(arg!(--cluster_id <VALUE> "Redshift Cluster ID"))
                            .arg_required_else_help(true),
                    ),
            ),
        )
}

pub async fn run(conf: AWSConfigFile) {
    // Read from CLI arguments
    let matches = cli::cmd().get_matches();

    // Parse global arguments to propagate to subcommands
    let default_profile = "default".to_owned();
    let profile = matches
        .get_one::<String>("profile")
        .unwrap_or(&default_profile);
    let timeout: &u64 = matches.get_one::<u64>("timeout").unwrap_or(&(5000_u64));

    // Load AWS Credentials Configuration
    info!("Using shared config with profile name: {}", profile);
    let aws_credentials_config = AWSCredentialsConfig::new(conf, profile);
    let shared_config: aws_types::SdkConfig =
        config::set_config(aws_credentials_config, *timeout).await;

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
                                        error!("Unknown input")
                                    }
                                }
                                _ => error!("Unknown input"),
                            }
                        }
                        ("list", flags) => {
                            let res = glue::list_jobs(shared_config).await;
                            if flags.get_flag("pretty") {
                                pretty_print(res.unwrap());
                            } else if flags.get_flag("csv") {
                                let _ = write_csv(res.unwrap());
                            } else {
                                error!("Unknown input")
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
                                error!("Unknown input")
                            }
                        }
                        _ => error!("Unknown input"),
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
                                error!("Unknown input")
                            }
                        }
                        _ => error!("Unknown input"),
                    }
                }
                _ => unreachable!(),
            }
        }
        Some(("rds", sub_matches)) => {
            let rds: RDS = RDS::new(shared_config); // Initialize DynamoDB client object
            let rds_command = sub_matches.subcommand().unwrap();
            match rds_command {
                ("instances", flags) => {
                    let res = rds.list_instances().await;
                    if flags.get_flag("pretty") {
                        pretty_print(res.unwrap());
                    } else if flags.get_flag("csv") {
                        let _ = write_csv(res.unwrap());
                    } else {
                        error!("Unknown input")
                    }
                }
                _ => error!("Unknown input"),
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
                                error!("Unknown input")
                            }
                        }
                        _ => error!("Unknown input"),
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
                                error!("Unknown input")
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
                                error!("Unknown input")
                            }
                        }
                        _ => error!("Unknown input"),
                    }
                }
                _ => error!("Unknown input"),
            }
        }
        Some(("dynamodb", sub_matches)) => {
            let ddb: DynamoDB = DynamoDB::new(shared_config); // Initialize DynamoDB client object
            let ddb_command = sub_matches.subcommand().unwrap();
            match ddb_command {
                ("tables", sub_matches) => {
                    let tables_subcommands = sub_matches.subcommand().unwrap();
                    match tables_subcommands {
                        ("list", flags) => {
                            let res = ddb.list_tables().await;
                            if flags.get_flag("pretty") {
                                pretty_print(res.unwrap());
                            } else if flags.get_flag("csv") {
                                let _ = write_csv(res.unwrap());
                            } else {
                                error!("Unknown input")
                            }
                        }
                        _ => error!("Unknown input"),
                    }
                }
                ("table", sub_matches) => {
                    let table_subcommands = sub_matches.subcommand().unwrap();
                    match table_subcommands {
                        ("describe", flags) => {
                            let table_name = flags.get_one::<String>("table_name").unwrap();
                            let res = ddb.describe_table(String::from(table_name)).await;
                            println!("{}", res.unwrap());
                        }
                        _ => error!("Unknown input"),
                    }
                }
                _ => error!("Unknown input"),
            }
        }
        Some(("redshift", sub_matches)) => {
            let redshift = Redshift::new(shared_config); // Initialize DynamoDB client object
            let command = sub_matches.subcommand().unwrap();
            match command {
                ("cluster", sub_matches) => {
                    let subcommands = sub_matches.subcommand().unwrap();
                    match subcommands {
                        ("describe", flags) => {
                            let cluster_id = flags.get_one::<String>("cluster_id").unwrap();
                            let _res = redshift.describe_clusters(String::from(cluster_id)).await;
                        }
                        _ => error!("Unknown input"),
                    }
                }
                _ => error!("Unknown input"),
            }
        }
        _ => error!("Faulty input is provided!"),
    }
}
