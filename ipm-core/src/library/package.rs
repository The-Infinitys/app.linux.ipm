use flate2::read::GzDecoder;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use tar::Archive;
pub mod detail;
mod install;
pub mod list;
mod uninstall;
use crate::library::system;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    name: String,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DependInfo {
    depend_type: String,
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileMapping {
    from: String,
    to: Vec<String>,
    file_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Files {
    global: Vec<FileMapping>,
    local: Vec<FileMapping>,
}

#[derive(Serialize, Deserialize, Debug)]
struct About {
    name: String,
    id: String,
    version: String,
    author: Author,
    description: String,
    license: String,
    dependencies: Vec<DependInfo>,
    architecture: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageInfo {
    about: About,
    files: Files,
}

pub fn install_packages(args: Vec<String>) {
    // Function to install a package
    println!("Installing package...");
    if args.is_empty() {
        println!("No package name or file path provided.");
        return;
    }
    system::configure::cleanup_tmp();
    println!("Downloading {} packages...", args.len());
    for arg in &args {
        let path = system::current_path().join(arg);
        if path.exists() {
            println!("File found: {}. Importing as local package...", arg);
            import_package_from_local(path.to_str().unwrap());
        } else {
            println!("Installing package: {}", arg);
        }
    }
    // ここでパッケージのインストール処理を行う
    install::install_packages();
}

pub fn import_package_from_local(file_path: &str) {
    // 1. ファイル、フォルダのデータを先に取得する
    let path = &fs::canonicalize(file_path).unwrap_or_else(|_| {
        eprintln!("Failed to resolve absolute path for: {}", file_path);
        Path::new(file_path).to_path_buf()
    });
    let file_path = path.to_str().unwrap_or(file_path);
    if !path.exists() {
        eprintln!("File or directory does not exist: {}", file_path);
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
        println!("Detected directory. Copying {} to ./tmp...", file_path);
        if let Err(e) = copy_directory(path, &cache_dir.join(path.file_name().unwrap())) {
            eprintln!("Failed to cache directory: {}", e);
        } else {
            println!("Successfully cache directory");
        }
    } else if let Some(extension) = path.extension() {
        // ファイルの場合
        match extension.to_str() {
            Some("gz") => {
                if let Some(parent_extension) =
                    path.file_stem().and_then(|s| Path::new(s).extension())
                {
                    if parent_extension == "tar" {
                        println!("Detected .tar.gz file. Extracting...");
                        if let Err(e) = extract_tar_gz_to(file_path, cache_dir) {
                            eprintln!("Failed to extract .tar.gz file: {}", e);
                        }
                    } else {
                        eprintln!("Unsupported .gz file format: {}", file_path);
                    }
                }
            }
            Some("tar") => {
                println!("Detected .tar file. Extracting...");
                if let Err(e) = extract_tar_to(file_path, cache_dir) {
                    eprintln!("Failed to extract .tar file: {}", e);
                }
            }
            _ => {
                println!("Copying file to ./tmp...");
                if let Err(e) = fs::copy(path, cache_dir.join(path.file_name().unwrap())) {
                    eprintln!("Failed to copy file: {}", e);
                } else {
                    println!("Successfully copied file");
                }
            }
        }
    } else {
        eprintln!("File has no extension: {}", file_path);
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
    println!("Successfully extracted .tar.gz file to {:?}", dest);
    Ok(())
}

fn extract_tar_to(file_path: &str, dest: &Path) -> io::Result<()> {
    let tar = File::open(file_path)?;
    let mut archive = Archive::new(tar);
    archive.unpack(dest)?; // 指定されたディレクトリに展開
    println!("Successfully extracted .tar file to {:?}", dest);
    Ok(())
}

pub fn uninstall_packages(args: Vec<String>) {
    // Function to uninstall packages
    println!("Uninstalling packages...");
    if args.is_empty() {
        println!("No package name provided...");
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
            println!("Package not found: {}", package_id);
        }
    }

    env::set_current_dir(current_dir).expect("failed to set dir");
}
