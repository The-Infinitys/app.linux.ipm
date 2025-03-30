use serde;
use serde_json;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// Define PackageInfo Struct
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Author {
    name: String,
    id: String,
}

#[derive(Debug, Deserialize)]
struct DependencyPackage {
    name: String,
    package_type: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct Dependencies {
    command: Vec<String>,
    package: Vec<DependencyPackage>,
}

#[derive(Debug, Deserialize)]
struct FileMapping {
    from: String,
    to: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Files {
    global: Vec<FileMapping>,
    local: Vec<FileMapping>,
}

#[derive(Debug, Deserialize)]
struct About {
    name: String,
    id: String,
    version: String,
    author: Author,
    description: String,
    license: String,
    dependencies: Dependencies,
}

#[derive(Debug, Deserialize)]
struct PackageInfo {
    about: About,
    files: Files,
}
pub fn install_packages() {
    println!("Starting package installation...");
    if let Ok(work_dir) = env::var("IPM_WORK_DIR") {
        if let Err(e) = env::set_current_dir(&work_dir) {
            eprintln!("Error: Failed to change directory to {}: {}", work_dir, e);
        } else {
            println!("Successfully changed directory to {}", work_dir);
        }
    } else {
        eprintln!("Error: IPM_WORK_DIR environment variable is not set.");
    }
    if let Err(e) = env::set_current_dir("./tmp") {
        eprintln!("Error: Failed to change directory to ./tmp: {}", e);
    } else {
        println!("Successfully changed directory to ./tmp");
    }
    // ここでパッケージのインストール処理を行う
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    println!("Directory: {}", path.display());
                    if let Err(e) = env::set_current_dir(&path) {
                        eprintln!(
                            "Error: Failed to change directory to {}: {}",
                            path.display(),
                            e
                        );
                    } else {
                        println!("Successfully changed directory to {}", path.display());
                        // ここでパッケージのインストール処理を行う
                        install_process();
                    }
                    env::set_current_dir("..").unwrap_or_else(|e| {
                        eprintln!("Error: Failed to change directory to ..: {}", e);
                    });
                } else {
                    println!("File: {}", path.display());
                }
            }
        }
    } else {
        eprintln!("Error: Failed to read the directory.");
    }
}

fn install_process() {
    let mut package_info = String::new();
    let info_file_path = Path::new("information.json");
    if info_file_path.exists() {
        if let Ok(mut file) = File::open(&info_file_path) {
            if let Err(e) = file.read_to_string(&mut package_info) {
                eprintln!("Error: Failed to read 'information.json': {}", e);
            } else {
                println!("Successfully loaded package information: {}", package_info);
            }
        } else {
            eprintln!("Error: Failed to open 'information.json'.");
        }
    } else {
        eprintln!("Error: 'information.json' does not exist.");
    }
    let package_info: Result<PackageInfo, _> = serde_json::from_str(&package_info);
    match package_info {
        Ok(info) => {
            println!("Package Name: {}", info.about.name);
            println!("Package ID: {}", info.about.id);
            println!("Version: {}", info.about.version);
            println!("Description: {}", info.about.description);
            println!(
                "Author: {} (ID: {})",
                info.about.author.name, info.about.author.id
            );
            println!("License: {}", info.about.license);
            println!(
                "Dependencies (Commands): {:?}",
                info.about.dependencies.command
            );
            for package in &info.about.dependencies.package {
                println!(
                    "Dependency Package - Name: {}, Type: {}, Version: {}",
                    package.name, package.package_type, package.version
                );
            }
            for command in &info.about.dependencies.command {
                println!("Dependency Command: {}", command);
            }
            println!("Files (Global):");
            for file in &info.files.global {
                println!("  From: {}, To: {:?}", file.from, file.to);
            }
            println!("Files (Local):");
            for file in &info.files.local {
                println!("  From: {}, To: {:?}", file.from, file.to);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to parse package information: {}", e);
        }
    }
}
