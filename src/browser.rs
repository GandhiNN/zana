#![allow(dead_code)]
use crate::utils;
use ini::ini;
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::process::Command;
use tracing::info;

const OSRELEASE: &str = "/proc/sys/kernel/osrelease";
const EDGE_PATH_WIN: &str = "\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe";
const EDGE_PATH_WSL: &str = "/mnt/c/Program Files (x86)/Microsoft/Edge/Application/msedge.exe";

/// Example browser invocation in WSL:
/// `$ ./msedge.exe www.google.com -inprivate`

#[derive(Debug)]
pub struct RuntimeOS {
    family: String,
}

impl RuntimeOS {
    pub fn get_family(&self) -> Self {
        if self.is_wsl() {
            Self {
                family: String::from("wsl"),
            }
        } else {
            Self {
                family: String::from(std::env::consts::OS),
            }
        }
    }

    pub fn is_wsl(&self) -> bool {
        let mut file = fs::File::open(OSRELEASE).expect("File not found");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Error while reading file");
        data.to_lowercase().contains("wsl")
    }

    pub fn url_browser_menu(self, browser: Browser) {
        let mut url_map = browser.load_url_config();
        browser.print_url_map(&mut url_map);
        let opt = utils::common::read_from_stdin();
        let url = match opt {
            Ok(buf) => buf,
            Err(error) => panic!("{}", error),
        };
        if url.as_str().trim_end() == "exit" {
            info!("Exiting program!");
            std::process::exit(exitcode::OK)
        }
        let url_to_browse = url_map
            .get(url.trim_end())
            .unwrap()
            .as_ref()
            .unwrap()
            .as_str();
        info!("Opening {}", url_to_browse);
        self.browse(url_to_browse);
    }

    pub fn browse(self, url: &str) {
        let runtime_os = self.get_family();
        match runtime_os.family.as_str() {
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
            _ => std::process::exit(1),
        };
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Browser {}

impl Browser {
    pub fn load_url_config(self) -> HashMap<String, Option<String>> {
        let current_path = std::env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        let url_config_path = format!("{}/url.ini", current_path);
        let ini = ini!(url_config_path.as_str());
        ini["url"].clone()
    }

    pub fn browse_edge_inprivate(url: &str) {
        Command::new(EDGE_PATH_WSL)
            .args([url, "-inprivate"])
            .status()
            .expect("process failed to execute");
    }

    pub fn browse_edge(url: &str) {
        Command::new(EDGE_PATH_WSL)
            .args([url])
            .status()
            .expect("process failed to execute");
    }

    pub fn print_url_map(self, url_map: &mut HashMap<String, Option<String>>) {
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
}
