use super::server;
use super::system;
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
pub fn update(args: Vec<String>) {
    println!("Update the website list{}", args[0]);
}
pub fn list() {
    println!("List all websites");
}
