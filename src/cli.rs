use clap::ArgAction;
#[allow(dead_code)]
use clap::{arg, command, value_parser, Command};

pub fn cmd() -> Command {
    command!()
        .version("0.1.0")
        .about("A CLI tool to interact with AWS environment")
        .author("Ngakan Gandhi <ngandhi.pmintl.net>")
        .bin_name("zana")
        .arg(
            arg!(--profile <VALUE>)
                .value_parser(value_parser!(String))
                .global(true),
        )
        .arg(
            arg!(--timeout <VALUE>)
                .value_parser(value_parser!(u64))
                .global(true),
        )
        .subcommand(
            Command::new("glue")
                .about("Glue API")
                .subcommand(
                    Command::new("jobs")
                        .about("Glue jobs API")
                        .arg(arg!(-l --list "lists glue jobs").action(ArgAction::SetTrue)),
                )
                .subcommand(
                    Command::new("databases")
                        .about("Glue databases API")
                        .arg(arg!(-l --list "lists glue databases").action(ArgAction::SetTrue)),
                )
                .subcommand(
                    Command::new("table")
                        .about("Glue table API")
                        .arg(arg!(-d --database <VALUE> "input database").required(true))
                        .arg(arg!(-l --list "lists glue tables").action(ArgAction::SetTrue)),
                ),
        )
        .subcommand(
            Command::new("s3").about("S3 API").subcommand(
                Command::new("bucket")
                    .about("S3 bucket API")
                    .arg(arg!(-l --list "lists s3 buckets")),
            ),
        )
        .subcommand(
            Command::new("rds").about("RDS API").arg(
                arg!(--task <VALUE>)
                    .required(true)
                    .value_parser(value_parser!(String)),
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
