use crate::aws::bedrock::Bedrock;
use crate::aws::bedrock_runtime::BedrockRuntime;
use crate::aws::config::{AWSConfigFile, AWSCredentialsConfig};
use crate::aws::cost_explorer::CostExplorer;
use crate::aws::docdb::DocDB;
use crate::aws::dynamodb::DynamoDB;
use crate::aws::glue::Glue;
use crate::aws::rds::RDS;
use crate::aws::redshift::Redshift;
use crate::aws::resource_explorer::ResourceExplorer;
use crate::aws::sso::Sso;
use crate::aws::{config, s3};
use crate::cli::cmd;
use crate::utils::common::{pretty_print, write_csv};
use tracing::{error, info};

pub async fn run(conf: AWSConfigFile) {
    // Read from CLI arguments
    let matches = cmd::cmd().get_matches();

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
            let glue = Glue::new(shared_config); // Initialize Glue client object
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
                                        flags.get_one::<String>("job-name").unwrap().to_string();
                                    let res = glue.get_job_runs(&glue_job_name).await;
                                    let pretty = flags.get_one::<bool>("pretty").unwrap_or(&false);
                                    if *pretty {
                                        pretty_print(res.unwrap());
                                    } else {
                                        let _ = write_csv(res.unwrap());
                                    }
                                }
                                _ => error!("Unknown input"),
                            }
                        }
                        ("list", flags) => {
                            let res = glue.list_jobs().await;
                            let pretty = flags.get_one::<bool>("pretty").unwrap_or(&false);
                            if *pretty {
                                pretty_print(res.unwrap());
                            } else {
                                let _ = write_csv(res.unwrap());
                            }
                        }
                        ("bookmark", flags) => {
                            let glue_job_name =
                                flags.get_one::<String>("job-name").unwrap().to_string();
                            let _ = glue.get_job_bookmark(&glue_job_name).await;
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
                            let res = glue.list_databases().await;
                            let pretty = flags.get_one::<bool>("pretty").unwrap_or(&false);
                            if *pretty {
                                pretty_print(res.unwrap());
                            } else {
                                let _ = write_csv(res.unwrap());
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
                            let res = glue.list_tables(db).await;
                            let pretty = flags.get_one::<bool>("pretty").unwrap_or(&false);
                            if *pretty {
                                pretty_print(res.unwrap());
                            } else {
                                let _ = write_csv(res.unwrap());
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
                    let pretty = flags.get_one::<bool>("pretty").unwrap_or(&false);
                    if *pretty {
                        pretty_print(res.unwrap());
                    } else {
                        let _ = write_csv(res.unwrap());
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
                            let pretty = flags.get_one::<bool>("pretty").unwrap_or(&false);
                            if *pretty {
                                pretty_print(res.unwrap());
                            } else {
                                let _ = write_csv(res.unwrap());
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
                            let prefix = flags.get_one::<String>("prefix").unwrap();
                            let last_mod_time =
                                flags.get_one::<String>("last-modified-time").unwrap();
                            let predicate = flags.get_one::<String>("predicate").unwrap();
                            let res = s3::list_objects(
                                shared_config,
                                bucket,
                                prefix,
                                last_mod_time,
                                predicate,
                            )
                            .await;
                            let pretty = flags.get_one::<bool>("pretty").unwrap_or(&false);
                            if *pretty {
                                pretty_print(res.unwrap());
                            } else {
                                let _ = write_csv(res.unwrap());
                            }
                        }
                        ("versions", flags) => {
                            let bucket = flags.get_one::<String>("bucket").unwrap();
                            let res = s3::list_objects_versions(shared_config, bucket).await;
                            let pretty = flags.get_one::<bool>("pretty").unwrap_or(&false);
                            if *pretty {
                                pretty_print(res.unwrap());
                            } else {
                                let _ = write_csv(res.unwrap());
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
                            let pretty = flags.get_one::<bool>("pretty").unwrap_or(&false);
                            if *pretty {
                                pretty_print(res.unwrap());
                            } else {
                                let _ = write_csv(res.unwrap());
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
                            let res = ddb.describe_table(table_name).await;
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
                            let res = redshift.describe_cluster(cluster_id).await;
                            println!("{}", res.unwrap());
                        }
                        _ => error!("Unknown input"),
                    }
                }
                ("events", sub_matches) => {
                    let subcommands = sub_matches.subcommand().unwrap();
                    match subcommands {
                        ("describe", _) => {
                            let _res = redshift.describe_events().await;
                        }
                        _ => error!("Unknown input"),
                    }
                }
                _ => error!("Unknown input"),
            }
        }
        Some(("resource-explorer", sub_matches)) => {
            let resource_explorer = ResourceExplorer::new(shared_config);
            let command = sub_matches.subcommand().unwrap();
            match command {
                ("search", flags) => {
                    let query_string = flags.get_one::<String>("query").unwrap();
                    let res = resource_explorer.search(query_string).await;
                    let pretty = flags.get_one::<bool>("pretty").unwrap_or(&false);
                    if *pretty {
                        pretty_print(res.unwrap());
                    } else {
                        let _ = write_csv(res.unwrap());
                    }
                }
                _ => error!("Unknown input"),
            }
        }
        Some(("cost-explorer", sub_matches)) => {
            let cost_explorer = CostExplorer::new(shared_config);
            let command = sub_matches.subcommand().unwrap();
            match command {
                ("get-cost-and-usage", flags) => {
                    let start = flags.get_one::<String>("start").unwrap();
                    let end = flags.get_one::<String>("end").unwrap();
                    let granularity = flags.get_one::<String>("granularity").unwrap();
                    let metrics = flags
                        .get_many::<String>("cost-aggregation-metrics")
                        .unwrap()
                        .map(|x| x.to_string())
                        .collect(); // Collect values as vector of owned strings
                    let group_by_type = flags.get_one::<String>("group-type").unwrap();
                    let group_by_key = flags.get_one::<String>("group-key").unwrap();
                    let _res = cost_explorer
                        .get_cost_and_usage(
                            start,
                            end,
                            granularity,
                            metrics,
                            group_by_type,
                            group_by_key,
                        )
                        .await;
                }
                _ => error!("Unknown input"),
            }
        }
        Some(("docdb", sub_matches)) => {
            let docdb = DocDB::new(shared_config);
            let command = sub_matches.subcommand().unwrap();
            match command {
                ("describe-clusters", _) => {
                    let _res = docdb.describe_clusters().await;
                }
                ("describe-cluster", flags) => {
                    let id = flags.get_one::<String>("id").unwrap();
                    let res = docdb.describe_cluster(id).await;
                    println!("{}", res.unwrap());
                }
                _ => error!("Unknown input"),
            }
        }
        Some(("bedrock", sub_matches)) => {
            let bedrock = Bedrock::new(shared_config.clone());
            let bedrock_runtime: BedrockRuntime = BedrockRuntime::new(shared_config.clone());
            let bedrock_command = sub_matches.subcommand().unwrap();
            match bedrock_command {
                ("mgmt", sub_matches) => {
                    let mgmt_subcommands = sub_matches.subcommand().unwrap();
                    match mgmt_subcommands {
                        ("list-foundational-models", flags) => {
                            let provider = flags.get_one::<String>("provider").unwrap();
                            let pretty = flags.get_one::<bool>("pretty").unwrap_or(&false);
                            let res = bedrock.list_foundational_models(provider).await;
                            if *pretty {
                                pretty_print(res.unwrap());
                            } else {
                                let _ = write_csv(res.unwrap());
                            }
                        }
                        _ => error!("Unknown input!"),
                    }
                }
                ("runtime", sub_matches) => {
                    let runtime_subcommands = sub_matches.subcommand().unwrap();
                    match runtime_subcommands {
                        ("invoke-prompt", flags) => {
                            let model = flags.get_one::<String>("model").unwrap();
                            let prompt = flags.get_one::<String>("prompt").unwrap();
                            let _res = bedrock_runtime.invoke_prompt(model, prompt).await;
                        }
                        _ => error!("Unknown input"),
                    }
                }
                _ => error!("Unknown input"),
            }
        }
        Some(("sso", sub_matches)) => {
            let sso = Sso::new(shared_config.clone());
            let sso_subcommands = sub_matches.subcommand().unwrap();
            match sso_subcommands {
                ("oidc", sub_matches) => {
                    let subcommands = sub_matches.subcommand().unwrap();
                    match subcommands {
                        ("configure", _) => {
                            let res = sso.configure_sso(shared_config).await;
                            println!("{}", res.unwrap());
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
