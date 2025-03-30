// use std::env;
// use std::fs;
// use std::path::Path;
// use std::process::Command;
// use serde_json;

// pub fn install_package() {
//     let ipm_work_dir: String =
//         env::var("IPM_WORK_DIR").expect("IPM_WORK_DIR is not set. Please set it.");
//     let package_dir: String = Path::join(Path::new(ipm_work_dir.as_str()), "package/cache")
//         .to_str()
//         .unwrap()
//         .to_string();
//     env::set_current_dir(&package_dir).expect("Failed to change directory to package/cache");
//     let info_file_path = Path::join(Path::new(&package_dir), "information.json");
//     if !info_file_path.exists() {
//         println!("information.json not found.");
//         return;
//     }
//     let info_file_content = fs::read_to_string(&info_file_path).expect("Failed to read information.json");
//     let info: serde_json::Value = serde_json::from_str(&info_file_content).expect("Failed to parse information.json");
//     let package_name = info["package"]["name"].as_str().unwrap();
//     let package_version = info["package"]["version"].as_str().unwrap();
//     let package_file_name = format!("{}-{}.tar.gz", package_name, package_version);
//     let package_file_path = Path::join(Path::new(&package_dir), &package_file_name);
//     if !package_file_path.exists() {
//         println!("{} not found.", package_file_name);
//         return;
//     }
//     let installed_dir = Path::join(Path::new(ipm_work_dir.as_str()), "package/installed");
//     let installed_package_dir = Path::join(&installed_dir, package_name);
//     if installed_package_dir.exists() {
//         println!("{} is already installed.", package_name);
//         return;
//     }
//     fs::create_dir_all(&installed_package_dir).expect("Failed to create installed package directory");
//     let output = Command::new("tar")
//         .arg("-xzf")
//         .arg(&package_file_name)
//         .arg("-C")
//         .arg(&installed_package_dir)
//         .output()
//         .expect("Failed to execute tar command");
//     if !output.status.success() {
//         println!("Failed to extract package.");
//         println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
//         println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
//         return;
//     }
//     println!("{} installed successfully.", package_name);

// }
