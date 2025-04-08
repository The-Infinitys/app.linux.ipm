use crate::core::system;
use std::fs;
use std::path::Path;
use crate::utils::shell::question;
use crate::write_info;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct IPMserverInfo {
    server: ServerInfo,
    author: AuthorInfo,
    version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServerInfo {
    name: String,
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthorInfo {
    name: String,
    id: String,
    email: String,
}

pub fn init_server() {
    // Initialize the server
    println!("IPM Server initialized");
    let current_path = system::current_path();

    // Configure the server
    let server_id = question("Enter server id (example: ipm-custom-server)", "kebab");
    let server_name = question("Enter server name (example: IPM Custom Server)", "string");
    let server_path = Path::new(&current_path).join(&server_id);
    // Collect user information
    let user_name = question("Enter your name (example: The Infinity's)", "string");
    let user_id = question("Enter your id (example: the-infinitys)", "kebab");
    let user_email = question("Enter your email address", "email");

    // Display collected information
    println!("Server name: {}", server_name);
    println!("Server id: {}", server_id);
    println!("Server path: {}", server_path.display());
    println!("User name: {}", user_name);
    println!("User email: {}", user_email);
    println!("User id: {}", user_id);

    // Confirm the information
    let check = question("Is that correct? (yes/no)", "yesno");
    if check == "yes" {
        write_info!("Start Server Initialization!");
    } else {
        write_info!("Server initialization canceled");
        return;
    }

    // Create the server directory if it doesn't exist
    if !server_path.exists() {
        fs::create_dir(&server_path).expect("Failed to create server directory");
        println!("Server directory '{}' created successfully", server_id);
        let ipm_info = IPMserverInfo {
            server: ServerInfo {
                name: server_name.clone(),
                id: server_id.clone(),
            },
            author: AuthorInfo {
                name: user_name.clone(),
                id: user_id.clone(),
                email: user_email.clone(),
            },
            version: "0.1.0".to_string(),
        };
        let ipm_info_path = server_path.join("ipm-info.json");
        let ipm_info_json = serde_json::to_string_pretty(&ipm_info).expect("Failed to serialize IPM server info");
        fs::write(&ipm_info_path, ipm_info_json).expect("Failed to write IPM server info to file");
        // Create a README.md file in the server directory
        let readme_path = server_path.join("README.md");
        let readme_content = format!(
            "# {}\n\n\
            **Server ID:** {}\n\n\
            **Author:** {}\n\n\
            **Email:** {}\n\n\
            **Version:** {}\n\n\
            This is the README file for the IPM server '{}'.",
            &server_name, &server_id, &user_name, &user_email, &ipm_info.version, &server_name
        );
        fs::write(&readme_path, readme_content).expect("Failed to write README.md file");
        let readme_path = server_path.join("package/README.md");
        let readme_content = include_str!("./server/default/package/README.md").to_string();
        fs::create_dir(&server_path.join("package")).expect("Failed to create package directory");
        fs::write(&readme_path, readme_content).expect("Failed to write README.md file");
        println!("README.md created at '{}'", readme_path.display());
        println!("IPM server info saved to '{}'", ipm_info_path.display());
    } else {
        println!("Server directory '{}' already exists", server_id);
        return;
    }
}
