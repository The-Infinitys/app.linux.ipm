use flate2::read::GzDecoder;
use lazy_static::lazy_static;
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
pub mod search;
mod uninstall;
use crate::core::system;
use serde;
use serde::{Deserialize, Serialize};
// IPM write system
// use crate::utils::shell::question;
use crate::core::www;
use crate::write_error;
use crate::write_info;
use crate::write_log;
use crate::write_warn;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    pub name: String,
    pub id: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DependencyType {
    Must,
    MustPre,
    Should,
    May,
    CannotBreak,
    CannotConflict,
}

impl fmt::Display for DependencyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DependencyType::Must => write!(f, "must"),
            DependencyType::MustPre => write!(f, "must.pre"),
            DependencyType::Should => write!(f, "should"),
            DependencyType::May => write!(f, "may"),
            DependencyType::CannotBreak => write!(f, "cannot.break"),
            DependencyType::CannotConflict => write!(f, "cannot.conflict"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DependInfo {
    pub depend_type: DependencyType,
    pub name: String,
    pub version: String,
    pub index: Option<u32>,
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

lazy_static! {
    static ref INSTALLED_PACKAGES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    static ref PENDING_PACKAGES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub fn install_packages(args: Vec<String>) {
    write_info!("Installing package...");
    if args.is_empty() {
        write_error!("No package name or file path provided.");
        return;
    }

    system::configure::cleanup_tmp();
    write_info!("Downloading {} packages...", args.len());

    let mut packages_to_install = Vec::new();
    let mut local_packages = Vec::new();

    // パッケージの分類と依存関係の収集
    for arg in &args {
        let path = system::current_path().join(arg);
        if path.exists() {
            write_info!("File found: {}. Importing as local package...", arg);
            local_packages.push(path);
        } else {
            packages_to_install.push(arg.clone());
        }
    }

    // ローカルパッケージの処理
    for package_path in local_packages {
        import_package_from_local(package_path.to_str().unwrap());
    }

    // 依存関係の解決とインストール
    let mut resolved_dependencies = HashMap::new();
    for package in &packages_to_install {
        resolve_dependencies(package, &mut resolved_dependencies);
    }

    // 依存関係に基づいてインストール順序を決定
    let install_order = determine_install_order(&resolved_dependencies);

    // パッケージのインストール
    for package_id in install_order {
        if !INSTALLED_PACKAGES.lock().unwrap().contains(&package_id) {
            write_info!("Installing package: {}", package_id);
            install_package(&package_id);
            INSTALLED_PACKAGES
                .lock()
                .unwrap()
                .insert(package_id.clone());
        }
    }
}

fn install_package(package_id: &str) {
    // TODO: 実際のパッケージインストール処理を実装
    write_info!("Installing package: {}", package_id);
    // ここにパッケージのインストール処理を実装
}

fn resolve_dependencies(package_id: &str, resolved: &mut HashMap<String, Vec<DependInfo>>) {
    // 循環依存のチェック
    if PENDING_PACKAGES.lock().unwrap().contains(package_id) {
        write_error!("Circular dependency detected for package: {}", package_id);
        return;
    }

    if INSTALLED_PACKAGES.lock().unwrap().contains(package_id) {
        return;
    }

    PENDING_PACKAGES
        .lock()
        .unwrap()
        .insert(package_id.to_string());

    // パッケージの依存関係を取得
    let dependencies = get_package_dependencies(package_id);
    resolved.insert(package_id.to_string(), dependencies.clone());

    // 依存パッケージの依存関係を再帰的に解決
    for dep in dependencies {
        resolve_dependencies(&dep.name, resolved);
    }

    PENDING_PACKAGES.lock().unwrap().remove(package_id);
}

fn get_package_dependencies(package_id: &str) -> Vec<DependInfo> {
    write_log!("Fetching dependencies for package: {}", package_id);

    let package_list = www::package_list();
    let package_info = match package_list.iter().find(|p| p.about.id == package_id) {
        Some(info) => info,
        None => {
            write_warn!("Package not found in www list: {}", package_id);
            return Vec::new();
        }
    };

    let dependencies = package_info.about.dependencies.clone();
    let mut valid_dependencies = Vec::new();

    for dep in dependencies {
        // 依存関係タイプの解析
        let (depend_type, index) = parse_dependency_type(&dep.depend_type.to_string());

        // バージョン要件の解析（deb形式）
        let version_req = parse_deb_version(&dep.version);

        valid_dependencies.push(DependInfo {
            depend_type,
            name: dep.name,
            version: version_req,
            index,
        });
    }

    write_log!(
        "Found {} dependencies for package: {}",
        valid_dependencies.len(),
        package_id
    );
    valid_dependencies
}

fn parse_dependency_type(type_str: &str) -> (DependencyType, Option<u32>) {
    let parts: Vec<&str> = type_str.split('.').collect();
    let base_type = parts[0];
    let index = parts.get(1).and_then(|&s| {
        if s == "index" {
            parts.get(2).and_then(|&num| num.parse::<u32>().ok())
        } else {
            None
        }
    });

    let depend_type = match base_type {
        "must" => DependencyType::Must,
        "must.pre" => DependencyType::MustPre,
        "should" => DependencyType::Should,
        "may" => DependencyType::May,
        "cannot.break" => DependencyType::CannotBreak,
        "cannot.conflict" => DependencyType::CannotConflict,
        _ => {
            write_warn!("Invalid dependency type: {}", type_str);
            DependencyType::Must
        }
    };

    (depend_type, index)
}

fn parse_deb_version(version_str: &str) -> String {
    // deb形式のバージョン要件を解析
    // 例: ">= 1.0.0", "= 2.3.4", "<< 3.0.0", ">> 1.5.0"
    version_str.to_string()
}

// 依存関係のグループ化を行う関数
fn group_dependencies_by_index(dependencies: Vec<DependInfo>) -> Vec<Vec<DependInfo>> {
    let mut groups: HashMap<Option<u32>, Vec<DependInfo>> = HashMap::new();

    for dep in dependencies {
        let group = groups.entry(dep.index).or_insert_with(Vec::new);
        group.push(dep);
    }

    // indexのない依存関係を個別のグループとして扱う
    let mut result: Vec<Vec<DependInfo>> = groups.into_iter().map(|(_, deps)| deps).collect();

    // indexの昇順でソート
    result.sort_by(|a, b| {
        let a_index = a.first().and_then(|d| d.index);
        let b_index = b.first().and_then(|d| d.index);
        a_index.cmp(&b_index)
    });

    result
}

fn determine_install_order(dependencies: &HashMap<String, Vec<DependInfo>>) -> Vec<String> {
    let mut install_order = Vec::new();
    let mut visited = HashSet::new();
    let mut temp = HashSet::new();

    for package in dependencies.keys() {
        if !visited.contains(package) {
            topological_sort(
                package,
                dependencies,
                &mut visited,
                &mut temp,
                &mut install_order,
            );
        }
    }

    install_order
}

fn topological_sort(
    package: &str,
    dependencies: &HashMap<String, Vec<DependInfo>>,
    visited: &mut HashSet<String>,
    temp: &mut HashSet<String>,
    result: &mut Vec<String>,
) {
    if temp.contains(package) {
        write_error!("Circular dependency detected for package: {}", package);
        return;
    }

    if visited.contains(package) {
        return;
    }

    temp.insert(package.to_string());

    if let Some(deps) = dependencies.get(package) {
        for dep in deps {
            topological_sort(&dep.name, dependencies, visited, temp, result);
        }
    }

    temp.remove(package);
    visited.insert(package.to_string());
    result.push(package.to_string());
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

pub fn search_packages(query: &str) {
    search::search_packages(query);
}
