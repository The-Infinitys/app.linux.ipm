pub mod configure;
use std::env;
use std::path::PathBuf;

// Path Configure
pub fn current_path() -> PathBuf {
    let _dir = env::var("IPM_CURRENT_DIR").expect("Failed to get $IPM_CURRENT_DIR");
    return std::fs::canonicalize(_dir).expect("Failed to convert to absolute path");
}
pub fn current_dir() -> String {
    return current_path().to_str().unwrap().to_string();
}
pub fn work_path() -> PathBuf {
    let _dir = env::var("IPM_WORK_DIR").expect("Failed to get $IPM_WORK_DIR");
    return current_path().join(_dir);
}

pub fn work_dir() -> String {
    return work_path().to_str().unwrap().to_string();
}

pub fn tmp_path() -> PathBuf {
    return work_path().join("tmp");
}
pub fn tmp_dir() -> String {
    return tmp_path().to_str().unwrap().to_string();
}
pub fn package_path() -> PathBuf {
    return work_path().join("package");
}
pub fn package_dir() -> String {
    return package_path().to_str().unwrap().to_string();
}

pub fn system_info_path() -> PathBuf {
    return work_path().join("bin/ipm-info.json");
}

pub fn logfile_path() -> PathBuf {
    return work_path().join("log/log.txt");
}

pub fn www_list_path() -> PathBuf {
    return work_path().join("www/list.json");
}
pub fn www_list_dir() -> String {
    return www_list_path().to_str().unwrap().to_string();
}

pub fn www_packages_path() -> PathBuf {
    return work_path().join("www/packages.json");
}
pub fn www_packages_dir() -> String {
    return www_packages_path().to_str().unwrap().to_string();
}
