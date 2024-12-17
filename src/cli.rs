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
                .default_value("dev")
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
                    Command::new("job").about("Glue job API").subcommand(
                        Command::new("runs").about("Glue job runs API").subcommand(
                            Command::new("list")
                                .arg(arg!(-j --jobname <VALUE> "Glue job name").required(true))
                                .arg(arg!(-p --pretty "Pretty print output"))
                                .arg(arg!(-c --csv "Print output as CSV"))
                                .arg_required_else_help(true),
                        ),
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
            Command::new("dynamodb").about("DynamoDB API").arg(
                arg!(--task <VALUE>)
                    .required(true)
                    .value_parser(value_parser!(String)),
            ),
        )
}
