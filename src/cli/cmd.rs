use clap::{arg, command, value_parser, ArgAction, Command};

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
            Command::new("redshift")
                .about("Redshift API")
                .subcommand(
                    Command::new("cluster")
                        .about("Redshift cluster API")
                        .subcommand(
                            Command::new("describe")
                                .arg(arg!(--cluster_id <VALUE> "Redshift Cluster ID"))
                                .arg_required_else_help(true),
                        ),
                )
                .subcommand(
                    Command::new("events")
                        .about("Redshift events API")
                        .subcommand(Command::new("describe")),
                ),
        )
        .subcommand(
            Command::new("resource-explorer")
                .about("Resource Explorer API")
                .subcommand(
                    Command::new("search")
                        .about("Search resources")
                        .arg(arg!(--query <VALUE> "Query string").required(true))
                        .arg(arg!(-p --pretty "Pretty print output").action(ArgAction::SetTrue))
                        .arg_required_else_help(true),
                ),
        )
        .subcommand(
            Command::new("cost-explorer")
                .about("Cost Explorer API")
                .subcommand(
                    Command::new("get-cost-and-usage")
                        .about("Get cost and usage")
                        .arg(arg!(--start <VALUE> "Start date. Format: YYYY-MM-DD").required(true))
                        .arg(arg!(--end <VALUE> "End date. Format: YYYY-MM-DD").required(true))
                        .arg(arg!(--granularity <VALUE> "Report Granularity. Possible values are:\n[daily, hourly, monthly]").default_value("hourly"))
                        .arg(
                            arg!(--"cost-aggregation-metrics" <VALUE> "Cost Aggregation Metrics. Possible values are:\n[AmortizedCost, BlendedCost, NetAmortizedCost, NetUnblendedCost, NormalizedUsageAmount, UnblendedCost, UsageQuantity]")
                                .value_delimiter(',').default_value("UnblendedCost,UsageQuantity") // accept multiple values e.g. "value1,value2"
                        )
                        .arg(arg!(--"group-type" <VALUE> "Group by type. Possible values are:\n[dimension, tag, costcategory]").default_value("dimension"))
                        .arg(arg!(--"group-key" <VALUE> "Group by key. Possible values are:\n[az, instance_type, legal, entity_name, invoicing_entity, linked_account, operation, platform, purchase_type, service, tenancy, record_type, usage_type]").default_value("service"))
                        .arg_required_else_help(true),
                ),
        )
        .subcommand(
            Command::new("docdb")
                .about("DocumentDB API")
                .subcommand(
                    Command::new("describe-clusters")
                        .about("Describe DocumentDB Clusters")
                )
                .subcommand(
                    Command::new("describe-cluster")
                        .about("Describe DocumentDB Cluster")
                        .arg(arg!(--id <VALUE> "DB Cluster ID").required(true))
                        .arg_required_else_help(true),
                )
        )
}
