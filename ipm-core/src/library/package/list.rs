use crate::library::package::PackageInfo;
use crate::library::system;
use crate::utils::shell::color_txt;
use std::env;
use std::path::Path;

pub fn installed_packages() {
    // Function to list installed packages
    let ipm_work_dir =
        env::var("IPM_WORK_DIR").expect("環境変数 IPM_WORK_DIR が設定されていません");
    let installed_dir = Path::new(&ipm_work_dir).join("package");
    for entry in installed_dir.read_dir().unwrap() {
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
