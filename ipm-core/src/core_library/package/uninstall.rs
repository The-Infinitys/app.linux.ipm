use crate::core_library::package::PackageInfo;
use crate::core_library::package::detail;
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
        }
        Err(e) => {
            eprintln!("Failed to load information: {}", e);
        }
    }
}
