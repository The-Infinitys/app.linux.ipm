use std::env;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use serde_json;
use serde;

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
                        eprintln!("Error: Failed to change directory to {}: {}", path.display(), e);
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
    #[derive(serde::Deserialize)]
    struct PackageInfo {
        name: String,
        version: String,
        description: Option<String>,
    }

    let package_info: Result<PackageInfo, _> = serde_json::from_str(&package_info);
    match package_info {
        Ok(info) => {
            println!("Package Name: {}", info.name);
            println!("Version: {}", info.version);
            if let Some(description) = info.description {
                println!("Description: {}", description);
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to parse package information: {}", e);
        }
    }
}