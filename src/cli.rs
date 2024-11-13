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
                .arg(
                    arg!(--database <VALUE>)
                        .required(false)
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(--table <VALUE>)
                        .required(false)
                        .value_parser(value_parser!(String)),
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
