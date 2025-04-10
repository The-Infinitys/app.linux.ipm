use flate2::read::GzDecoder;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use tar::Archive;
pub mod detail;
mod install;
// mod depend;
pub mod list;
mod uninstall;
use crate::core::system;
use serde;
use serde::{Deserialize, Serialize};
// IPM write system
// use crate::utils::shell::question;
use crate::write_error;
use crate::write_info;
use crate::write_log;
use crate::write_warn;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    pub name: String,
    pub id: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DependInfo {
    pub depend_type: String, // `pub`を追加
    pub name: String,        // `pub`を追加
    pub version: String,     // `pub`を追加
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMapping {
    from: String,
    to: Vec<String>,
    file_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Files {
    global: Vec<FileMapping>,
    local: Vec<FileMapping>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct About {
    pub name: String,
    pub id: String,
    pub version: String,
    pub author: Author,
    pub description: String, // `pub`を追加
    pub license: String,     // `pub`を追加
    pub dependencies: Vec<DependInfo>,
    pub architecture: Vec<String>, // `pub`を追加
    pub size: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageInfo {
    pub about: About,
    files: Files,
}

pub fn install_packages(args: Vec<String>) {
    // Function to install a package
    write_info!("Installing package...");
    if args.is_empty() {
        write_error!("No package name or file path provided.");
        return;
    }
    system::configure::cleanup_tmp();
    write_info!("Downloading {} packages...", args.len());
    for arg in &args {
        let path = system::current_path().join(arg);
        if path.exists() {
            write_info!("File found: {}. Importing as local package...", arg);
            import_package_from_local(path.to_str().unwrap());
        } else {
            write_info!("Installing package: {}", arg);
        }
    }
    // ここでパッケージのインストール処理を行う
    install::install_packages();
}

pub fn import_package_from_local(file_path: &str) {
    // 1. ファイル、フォルダのデータを先に取得する
    let path = &fs::canonicalize(file_path).unwrap_or_else(|_| {
        write_error!("Failed to resolve absolute path for: {}", file_path);
        Path::new(file_path).to_path_buf()
    });
    let file_path = path.to_str().unwrap_or(file_path);
    if !path.exists() {
        write_error!("File or directory does not exist: {}", file_path);
        return;
    }

    // データの取得
    let is_directory = path.is_dir();

    // 2. カレントディレクトリを ipm のワークディレクトリに変更する
    let current_dir = env::current_dir().unwrap();
    env::set_current_dir(&system::work_dir()).expect("Failed to set current directory");

    // 3. 予め取得したデータを ./tmp にコピーする
    let cache_dir = &system::tmp_path();
    if !cache_dir.exists() {
        fs::create_dir_all(cache_dir).expect("Failed to create cache directory");
    }

    if is_directory {
        // ディレクトリの場合
        write_log!("Detected directory. Copying {} to ./tmp...", file_path);
        if let Err(e) = copy_directory(path, &cache_dir.join(path.file_name().unwrap())) {
            write_error!("Failed to cache directory: {}", e);
        } else {
            write_log!("Successfully cache directory");
        }
    } else if let Some(extension) = path.extension() {
        // ファイルの場合
        match extension.to_str() {
            Some("gz") => {
                if let Some(parent_extension) =
                    path.file_stem().and_then(|s| Path::new(s).extension())
                {
                    if parent_extension == "tar" {
                        write_log!("Detected .tar.gz file. Extracting...");
                        if let Err(e) = extract_tar_gz_to(file_path, cache_dir) {
                            write_error!("Failed to extract .tar.gz file: {}", e);
                        }
                    } else {
                        write_error!("Unsupported .gz file format: {}", file_path);
                    }
                }
            }
            Some("tar") => {
                write_log!("Detected .tar file. Extracting...");
                if let Err(e) = extract_tar_to(file_path, cache_dir) {
                    write_error!("Failed to extract .tar file: {}", e);
                }
            }
            Some("zip") => {
                write_log!("Detected .zip file. Extracting...");
                if let Err(e) = extract_zip_to(file_path, cache_dir) {
                    write_error!("Failed to extract .zip file: {}", e);
                }
            }
            _ => {
                write_info!("Copying file to ./tmp...");
                if let Err(e) = fs::copy(path, cache_dir.join(path.file_name().unwrap())) {
                    write_error!("Failed to copy file: {}", e);
                } else {
                    write_log!("Successfully copied file");
                }
            }
        }
    } else {
        write_warn!("File has no extension: {}", file_path);
    }

    // 元のカレントディレクトリに戻す
    env::set_current_dir(current_dir).expect("Failed to set current directory");
}

fn copy_directory(src: &Path, dest: &Path) -> io::Result<()> {
    if !src.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Source directory does not exist",
        ));
    }
    if !src.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source is not a directory",
        ));
    }
    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }
    if !dest.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Destination is not a directory",
        ));
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if entry_path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_directory(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }
    Ok(())
}

fn extract_tar_gz_to(file_path: &str, dest: &Path) -> io::Result<()> {
    let tar_gz = File::open(file_path)?;
    let decompressed = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(decompressed);
    archive.unpack(dest)?; // 指定されたディレクトリに展開
    write_log!("Successfully extracted .tar.gz file to {:?}", dest);
    Ok(())
}

fn extract_tar_to(file_path: &str, dest: &Path) -> io::Result<()> {
    let tar = File::open(file_path)?;
    let mut archive = Archive::new(tar);
    archive.unpack(dest)?; // 指定されたディレクトリに展開
    write_log!("Successfully extracted .tar file to {:?}", dest);
    Ok(())
}

fn extract_zip_to(file_path: &str, dest: &Path) -> io::Result<()> {
    let file = File::open(file_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    archive.extract(dest)?;
    write_log!("Successfully extracted .zip file to {:?}", dest);
    Ok(())
}

pub fn uninstall_packages(args: Vec<String>) {
    // Function to uninstall packages
    write_info!("Uninstalling packages...");
    if args.is_empty() {
        write_warn!("No package name provided...");
        return;
    }
    // ここでパッケージのアンインストール処理を行う
    let package_list = list::data();
    let current_dir = env::current_dir().expect("Failed to get current dir.");
    for package_id in args {
        let mut is_exist = false;
        for package in &package_list {
            if package.about.id == package_id {
                is_exist = true;
                break;
            }
        }
        if is_exist {
            env::set_current_dir(system::package_path().join(&package_id))
                .expect("Failed to move current dir.");
            uninstall::uninstall();
        } else {
            write_warn!("Package not found: {}", package_id);
        }
    }

    env::set_current_dir(current_dir).expect("failed to set dir");
}

// TODO: Create Here
// pub fn init_package() {
//     // Initialize the package
//     println!("IPM Package initialized");
//     let current_path = system::current_path();

//     // Configure the package
//     let package_id = question("Enter package id (example: ipm-default-package)", "kebab");
//     let package_name = question(
//         "Enter package name (example: IPM Default Package)",
//         "string",
//     );
//     let package_path = Path::new(&current_path).join(&package_id);
//     // Collect user information
//     let user_name = question("Enter your name (example: The Infinity's)", "string");
//     let user_id = question("Enter your id (example: the-infinitys)", "kebab");
//     let user_email = question("Enter your email address", "email");

//     // Display collected information
//     println!("Package name: {}", package_name);
//     println!("Package id: {}", package_id);
//     println!("Package path: {}", package_path.display());
//     println!("User name: {}", user_name);
//     println!("User email: {}", user_email);
//     println!("User id: {}", user_id);

//     // Confirm the information
//     let check = question("Is that correct? (yes/no)", "yesno");
//     if check == "yes" {
//         write_info!("Start Package Initialization!");
//     } else {
//         write_info!("Package initialization canceled");
//         return;
//     }

//     // Create the package directory if it doesn't exist
//     if !package_path.exists() {
//         fs::create_dir(&package_path).expect("Failed to create package directory");
//         println!("package directory '{}' created successfully", package_id);
//         let ipm_info = PackageInfo {};
//         let ipm_info_path = package_path.join("ipm-info.json");
//         let ipm_info_json =
//             serde_json::to_string_pretty(&ipm_info).expect("Failed to serialize IPM package info");
//         fs::write(&ipm_info_path, ipm_info_json).expect("Failed to write IPM package info to file");
//         // Create a README.md file in the package directory
//         let readme_path = package_path.join("README.md");
//         let readme_content = format!(
//             "# {}\n\n\
//         **package ID:** {}\n\n\
//         **Author:** {}\n\n\
//         **Email:** {}\n\n\
//         **Version:** {}\n\n\
//         This is the README file for the IPM package '{}'.",
//             &package_name, &package_id, &user_name, &user_email, &ipm_info.version, &package_name
//         );
//         fs::write(&readme_path, readme_content).expect("Failed to write README.md file");
//         let readme_path = package_path.join("package/README.md");
//         let readme_content = include_str!("./package/default/package/README.md").to_string();
//         fs::create_dir(&package_path.join("package")).expect("Failed to create package directory");
//         fs::write(&readme_path, readme_content).expect("Failed to write README.md file");
//         println!("README.md created at '{}'", readme_path.display());
//         println!("IPM package info saved to '{}'", ipm_info_path.display());
//     } else {
//         println!("package directory '{}' already exists", package_id);
//         return;
//     }
// }
