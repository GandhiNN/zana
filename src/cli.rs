use clap::{Arg, ArgAction, Command};

pub fn cmd() -> clap::Command {
    Command::new("zana")
        .version("0.1.0")
        .about("A CLI tool to interact with AWS environment")
        .author("Ngakan Gandhi <ngandhi.pmintl.net>")
        .bin_name("zana")
        .arg(
            Arg::new("profile")
                .short('p')
                .long("profile")
                .default_value("default")
                .value_parser(clap::value_parser!(String))
                .help("AWS profile name to be used"),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .action(ArgAction::SetTrue)
                .value_parser(clap::value_parser!(bool))
                .help("Print the currently configured AWS profile"),
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .default_value("10")
                .value_parser(clap::value_parser!(u64))
                .help("Session timeout duration in seconds"),
        )
        .arg(
            Arg::new("service")
                .short('s')
                .long("service")
                .value_parser(clap::value_parser!(String))
                .help("AWS service name to use"),
        )
}
