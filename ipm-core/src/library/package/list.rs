use crate::library::package::PackageInfo;
use crate::library::system;
use crate::utils::shell::color_txt;
use chrono;
use serde;
use serde::Deserialize;
use serde::Serialize;
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
pub struct PackageList {
    pub packages: Vec<PackageInfo>,
    pub date: String,
    pub count: usize,
}

pub fn installed_packages() {
    // Function to list installed packages
    for entry in system::package_path().read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let info_file = path.join("information.json");
            if info_file.exists() {
                let package_info =
                    std::fs::read_to_string(info_file).expect("情報ファイルを読み取れませんでした");
                let package_info: Result<PackageInfo, _> = serde_json::from_str(&package_info);
                match package_info {
                    Ok(info) => {
                        // show package info
                        println!(
                            "{name}(version: {version}) by {author}",
                            name = color_txt(&info.about.id, 0, 255, 0),
                            version = info.about.version,
                            author = info.about.author.id
                        );
                    }
                    Err(e) => {
                        eprintln!("Error happend: {}", e)
                    }
                }
            }
        }
    }
}

pub fn data() -> Vec<PackageInfo> {
    let mut package_list =
        Vec::with_capacity(system::package_path().read_dir().into_iter().count());
    for entry in system::package_path().read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let info_file = path.join("information.json");
            if info_file.exists() {
                let package_info =
                    std::fs::read_to_string(info_file).expect("情報ファイルを読み取れませんでした");
                let package_info: Result<PackageInfo, _> = serde_json::from_str(&package_info);
                match package_info {
                    Ok(info) => {
                        // show package info
                        package_list.push(info);
                    }
                    Err(e) => {
                        eprintln!("Error happend: {}", e)
                    }
                }
            }
        }
    }
    return package_list;
}
pub fn update() {
    let package_list = data();
    let package_list_data = PackageList {
        packages: package_list,
        date: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        count: 0,
    };
    let package_list_path = system::package_path().join("list.json");
    if package_list_path.exists() {
        std::fs::remove_file(&package_list_path).expect("Failed to remove package list file");
    }
    let package_list_data =
        serde_json::to_string_pretty(&package_list_data).expect("Failed to convert to json");
    std::fs::write(&package_list_path, package_list_data)
        .expect("Failed to write package list file");
}
