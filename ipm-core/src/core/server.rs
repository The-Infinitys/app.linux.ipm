use crate::core::system;
use std::fs;
use std::path::Path;
use crate::utils::shell::question;
use std::env;
pub fn init_server() {
    // Initialize the server
    println!("IPM Server initialized");
    let current_path = system::current_path();
    // Configure the server
    let server_id = question("Enter server id(example: ipm_custom_server)", "kebab");
    let server_name = question("Enter server name(example: IPM Custom Server)", "string");
    let server_path = Path::new(&current_path).join(&server_id);
    if !server_path.exists() {
        fs::create_dir(&server_path).expect("Failed to create server directory");
        println!("Server directory '{}' created successfully", server_id);
    } else {
        println!("Server directory '{}' already exists", server_id);
    
    }
    // Change the current working directory to the server directory
    std::env::set_current_dir(&server_path).expect("Failed to change directory");
    // Delete all files and directories in the server directory
    let user_name = question("Enter your name(example: The Infinity's)", "string");
    let user_id = question("Enter your id(example: the-infinitys)", "kebab");
    let user_email = question("Enter your email address","email");
    println!("Server name: {}", server_name);
    println!("Server id: {}", server_id);
    println!("Server path: {}", server_path.display());
    println!("User name: {}", user_name);
    println!("User email: {}", user_email);
    println!("User id: {}", user_id);
    println!("Server directory: {}", server_path.display());
    let check=question("Is that correct? (yes/no)","yesno");
    if check=="yes"{
        println!("Server initialized successfully");
    } else{
        println!("Server initialization failed");
        env::set_current_dir("../").expect("Failed to change directory");
        fs::remove_dir_all(&server_path).expect("Failed to remove server directory");
        println!("Canceled. Server directory removed.");
    }
}
