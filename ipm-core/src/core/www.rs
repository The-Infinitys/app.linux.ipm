use std::collections::LinkedList;
use std::fs;

use super::package::About;
use super::server;
use super::system;
use crate::third::*;
use crate::write_error;
use crate::write_info;
use crate::write_log;
use chrono::Local;
use reqwest;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WwwList {
    pub list: Vec<WwwInfo>,
    pub last_update: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WwwInfo {
    pub name: String,
    pub url: String,
    pub www_type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WwwPackages {
    pub list: Vec<WwwPackageInfo>,
    pub last_update: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WwwPackageInfo {
    pub about: About,
    pub package_type: String,
    pub package_url: String,
}

pub fn add(args: Vec<String>) {
    if args.len() < 1 {
        write_error!("Please provide a website to add");
        return;
    }
    let www_type = args[0].clone();
    let www_url = args[1].clone();
    let www_list = system::www_list_path();
    if !www_list.exists() {
        write_info!("The WWW list does not exist. recreating it.");
        let new_list = WwwList {
            list: Vec::new(),
            last_update: Local::now().to_rfc3339(),
        };
        let new_list = serde_json::to_string(&new_list).expect("Failed to serialize");
        std::fs::write(&www_list, new_list).expect("Failed to write file");
    }
    let www_list = std::fs::read_to_string(&www_list).expect("Failed to read file");
    let mut www_list: WwwList = serde_json::from_str(&www_list).expect("Failed to parse JSON");
    match &*www_type {
        "ipm" => {
            write_log!("Adding IPM www: {}", www_url);
            let index_file_url = format!("{}/ipm-server-index.json", www_url);
            let response = reqwest::blocking::get(&index_file_url).expect("Failed to send request");
            let response = response.text().expect("Failed to read response");
            let www_index: server::IPMserverIndex =
                serde_json::from_str(&response).expect("Failed to parse JSON");
            let adding_www_info = WwwInfo {
                name: www_index.info.server.id.clone(),
                url: www_url.clone(),
                www_type: www_type.clone(),
            };
            www_list.list.push(adding_www_info);
            write_info!("{:?}", &www_index);
        }
        "apt" => {
            write_log!("Adding apt www: {}", www_url);
            let mut repo_info = apt::repository::AptRepositoryInfo {
                name: String::new(),
                url: www_url,
                suites: Vec::new(),
                components: Vec::new(),
                architectures: Vec::new(),
                options: Vec::new(),
            };
            let args = args[2..].to_vec();
            for arg in args {
                if let Some((key, value)) = arg.split_once('=') {
                    match key {
                        "--name" =>{
                            repo_info.name = value.trim().to_string();
                        }
                        "--suites" => {
                            repo_info.suites =
                                value.split(',').map(|s| s.trim().to_string()).collect();
                        }
                        "--components" => {
                            repo_info.components =
                                value.split(',').map(|c| c.trim().to_string()).collect();
                        }
                        "--architectures" => {
                            repo_info.architectures =
                                value.split(',').map(|a| a.trim().to_string()).collect();
                        }
                        "--options" => {
                            repo_info.options =
                                value.split(',').map(|o| o.trim().to_string()).collect();
                        }
                        _ => {
                            write_error!("Unknown argument: {}", key);
                        }
                    }
                } else {
                    write_error!("Invalid argument format: {}", arg);
                }
            }
            
            // Validate that all required fields in AptRepositoryInfo are set
            if repo_info.name.is_empty() {
                write_error!("The '--name' argument is required for an apt repository.");
                return;
            }
            if repo_info.suites.is_empty() {
                write_error!("The '--suites' argument is required for an apt repository.");
                return;
            }
            if repo_info.components.is_empty() {
                write_error!("The '--components' argument is required for an apt repository.");
                return;
            }
            if repo_info.architectures.is_empty() {
                write_error!("The '--architectures' argument is required for an apt repository.");
                return;
            }
            
            println!("{:?}",repo_info);
        }
        _ => {
            write_error!("Unknown wwww type: {}", www_type);
            write_info!("Please use 'ipm' or 'apt' as the website type");
            return;
        }
    }
    let www_list = serde_json::to_string_pretty(&www_list).expect("Failed to serialize");
    std::fs::write(&system::www_list_path(), &www_list).expect("Failed to write file");
}
pub fn rm(args: Vec<String>) {
    println!("Remove a website from the list{}", args[0]);
}

pub fn update() {
    let www_list = system::www_list_path();
    let www_list = std::fs::read_to_string(&www_list).expect("Failed to read file");
    let www_list: WwwList = serde_json::from_str(&www_list).expect("Failed to parse JSON");
    let mut www_packages: LinkedList<WwwPackageInfo> = LinkedList::new();
    for www_server in &www_list.list {
        match &*www_server.www_type {
            "ipm" => {
                let server_url = &www_server.url;
                let index_data = format!("{}/ipm-server-index.json", &www_server.url);
                let index_data = reqwest::blocking::get(index_data)
                    .expect("Failed to get Index Data.")
                    .text()
                    .expect("Failed to perse to text.");
                let index_data: server::IPMserverIndex =
                    serde_json::from_str(&index_data).expect("Failed to parse JSON");
                for www_package_info in &index_data.packages {
                    let package_url =
                        format!("{}/package/{}.ipm", &server_url, &www_package_info.id);
                    let adding_package = WwwPackageInfo {
                        about: www_package_info.clone(),
                        package_url: package_url,
                        package_type: "ipm".to_string(),
                    };
                    www_packages.push_back(adding_package);
                }
            }
            "apt" => {}
            _ => write_error!("Invalid www type!"),
        }
    }
    let www_packages = www_packages.into_iter().collect();
    let www_packages_list = system::www_packages_path();
    let www_packages_data = WwwPackages {
        list: www_packages,
        last_update: chrono::Local::now().to_rfc3339(),
    };
    let www_packages_data =
        serde_json::to_string_pretty(&www_packages_data).expect("Failed to parse.");
    fs::write(www_packages_list, www_packages_data).expect("Failed to write package's data");
}
pub fn list() {
    println!("List all websites");
}
