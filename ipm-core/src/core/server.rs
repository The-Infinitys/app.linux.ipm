use super::package;
use super::package::Author;
use crate::core::package::PackageInfo;
use crate::core::system;
use crate::utils::shell::question;
use crate::write_info;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IPMserverInfo {
    pub server: ServerInfo,
    pub author: Author,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerInfo {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IPMserverIndex {
    pub info: IPMserverInfo,
    pub packages: Vec<package::About>,
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
            author: Author {
                name: user_name.clone(),
                id: user_id.clone(),
                email: user_email.clone(),
            },
            version: "0.1.0".to_string(),
        };
        let ipm_info_path = server_path.join("ipm-info.json");
        let ipm_info_json =
            serde_json::to_string_pretty(&ipm_info).expect("Failed to serialize IPM server info");
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

pub fn build_server() {
    // Build the server
    println!("Building IPM Server");
    let current_path = system::current_path();
    let server_path = current_path;
    let ipm_info_path = server_path.join("ipm-info.json");
    if !ipm_info_path.exists() {
        write_info!("ipm-info.json not found");
        return;
    }
    let ipm_info_json = fs::read_to_string(&ipm_info_path).expect("Failed to read IPM server info");
    let ipm_info: IPMserverInfo =
        serde_json::from_str(&ipm_info_json).expect("Failed to parse IPM server info");
    let package_path = server_path.join("package");
    if !package_path.exists() {
        write_info!("package directory not found");
        return;
    }
    let package_list = fs::read_dir(&package_path).expect("Failed to read package directory");
    let package_count = package_list.count();
    if package_count == 0 {
        write_info!("No packages found in the package directory");
        return;
    }
    let mut ipm_server_index = IPMserverIndex {
        info: ipm_info.clone(),
        packages: Vec::with_capacity(package_count),
    };

    fs::create_dir_all("out/packages").expect("Failed to create output directories");
    let export_path = server_path.join("out");
    let export_packages_path = export_path.join("packages");

    for package in fs::read_dir(&package_path).expect("Failed to read package directory") {
        let package = package.expect("Failed to read package");
        let package_path = package.path();
        if package_path.is_dir() {
            // Gather package information
            let information_file_path = package_path.join("information.json");
            if !information_file_path.exists() {
                write_info!("information.json not found in {}", package_path.display());
                continue;
            }
            let information_file_json = fs::read_to_string(&information_file_path)
                .expect("Failed to read information.json");
            let package_info: PackageInfo = serde_json::from_str(&information_file_json)
                .expect("Failed to parse information.json");
            ipm_server_index.packages.push(package_info.about.clone());

            // Compress the package directory
            let zip_file_path = export_packages_path.join(format!(
                "{}.ipm",
                package_path.file_name().unwrap().to_str().unwrap()
            ));
            let file = fs::File::create(&zip_file_path).expect("Failed to create ZIP file");
            let mut zip = zip::ZipWriter::new(file);

            let options: zip::write::FileOptions<()> = zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755);

            for entry in fs::read_dir(&package_path).expect("Failed to read package directory") {
                let entry = entry.expect("Failed to read directory entry");
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    zip.start_file(file_name, options)
                        .expect("Failed to start file in ZIP");
                    let mut f = fs::File::open(&path).expect("Failed to open file for zipping");
                    std::io::copy(&mut f, &mut zip).expect("Failed to write file to ZIP");
                }
            }

            zip.finish().expect("Failed to finalize ZIP file");
            println!(
                "Compressed package directory '{}' into '{}'",
                package_path.display(),
                zip_file_path.display()
            );
        }
    }

    let ipm_server_index_path = export_path.join("ipm-server-index.json");
    let ipm_server_index_json = serde_json::to_string_pretty(&ipm_server_index)
        .expect("Failed to serialize IPM server index");
    fs::write(&ipm_server_index_path, ipm_server_index_json)
        .expect("Failed to write IPM server index to file");
}
