use crate::common::util;
use ini::ini;
use std::collections::HashMap;

extern crate std;

fn check_runtime_os() -> &'static str {
    if is_wsl() {
        return "wsl";
    } else {
        return std::env::consts::OS;
    }
}

fn is_wsl() -> bool {
    use std::io::prelude::*;
    let mut file = std::fs::File::open("/proc/sys/kernel/osrelease").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    if data.to_lowercase().contains("wsl") {
        return true;
    }
    return false;
}

fn load_url_config() -> HashMap<String, Option<String>> {
    let current_path = std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let url_config_path = format!("{}/url.ini", current_path);
    let ini = ini!(url_config_path.as_str());
    ini["url"].clone()
}

fn print_url_map(url_map: &mut HashMap<String, Option<String>>) {
    println!("####################");
    println!("## Available URLs ##");
    println!("####################");
    for (key, value) in url_map {
        let base_indent_level = 20;
        let key_length = key.len();
        println!(
            "{}{:indent$}=\t{}",
            key,
            "",
            value.clone().unwrap(),
            indent = base_indent_level - key_length
        );
    }
    println!();
}

fn open_url_in_browser(url: &str) {
    let runtime_os = check_runtime_os();
    match runtime_os {
        "linux" => std::process::Command::new("xdg-open")
            .arg(url)
            .status()
            .expect("process failed to execute"),
        "wsl" => std::process::Command::new("wslview")
            .arg(url)
            .status()
            .expect("process failed to execute"),
        "darwin" => std::process::Command::new("open")
            .arg(url)
            .status()
            .expect("process failed to execute"),
        _ => {
            println!("not implemented");
            return;
        }
    };
}

pub fn url_browser_menu() {
    let mut url_map = load_url_config();
    print_url_map(&mut url_map);
    let opt = util::read_from_stdin();
    let url = match opt {
        Ok(buf) => buf,
        Err(error) => panic!("{}", error),
    };
    match url.as_str().trim_end() {
        // trim_end() is used to remove '\n'
        "exit" => {
            println!("Exiting program!");
            std::process::exit(exitcode::OK);
        }
        _ => (),
    }
    let url_to_browse = url_map
        .get(url.trim_end())
        .unwrap()
        .as_ref()
        .unwrap()
        .as_str();
    println!("Opening {}", url_to_browse);
    open_url_in_browser(url_to_browse);
}
