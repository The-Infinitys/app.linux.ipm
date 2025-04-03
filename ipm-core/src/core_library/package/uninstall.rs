use crate::core_library::package::PackageInfo;
use crate::core_library::package::detail;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
pub fn uninstall() {
    let mut package_info = String::new();
    let info_file_path = Path::new("information.json");
    if info_file_path.exists() {
        if let Ok(mut file) = File::open(&info_file_path) {
            if let Err(e) = file.read_to_string(&mut package_info) {
                eprintln!("Error: Failed to read 'information.json': {}", e);
            } else {
                println!("Successfully loaded package information.");
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
            detail::show_from_info(&info);
            println!("Start Uninstalling...");
            for global_file_set in &info.files.global {
                for remove_target in &global_file_set.to {
                    let absolute_path = Path::new("/").join(remove_target);
                    if let Err(e) = std::fs::remove_file(absolute_path) {
                        if e.kind() != std::io::ErrorKind::NotFound {
                            eprintln!("Failed to remove file '{}': {}", remove_target, e);
                        }
                    } else {
                        println!("Successfully removed file '{}'.", remove_target);
                    }
                }
            }
            if let Err(e) = fs::remove_dir_all(".") {
                eprintln!("Failed to remove current directory contents: {}", e);
            } else {
                println!("Successfully removed all contents of the current directory.");
            }
        }
        Err(e) => {
            eprintln!("Failed to load information: {}", e);
        }
    }
}
