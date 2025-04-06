use crate::core::system;
// use nix;
use std::env;
use std::fs;
use std::path::Path;

pub fn configure() {
    // Configure Environment Information
    const DEBUG: bool = cfg!(debug_assertions);
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let ipm_work_dir = match env::current_exe()
        .expect("Failed to get current exe")
        .parent()
    {
        Some(path) => std::fs::canonicalize(path.join("../"))
            .expect("Failed to canonicalize")
            .to_path_buf(),
        None => panic!("Failed to get parent directory"),
    };
    println!("IPM Working directory: {:?}", &ipm_work_dir);
    const IPM_VERSION: &str = env!("CARGO_PKG_VERSION");
    unsafe {
        env::set_var("IPM_EXEC_MODE", if DEBUG { "debug" } else { "release" });
        env::set_var("IPM_WORK_DIR", &ipm_work_dir);
        env::set_var("IPM_CURRENT_DIR", &current_dir);
        env::set_var("IPM_VERSION", &IPM_VERSION);
    }
    if DEBUG {
        println!("Debug mode is enabled.");
        println!("Current directory: {:?}", &system::current_dir());
        println!("IPM Working directory: {:?}", &system::work_dir());
        println!("IPM Temporary directory: {:?}", &system::tmp_dir());
        println!("IPM Package directory: {:?}", &system::package_dir());
    }
    // Configure System Information
    std::fs::write(system::system_info_path(), &system_info())
        .expect("Failed to write system info");
}
fn system_info() -> String {
    #[derive(serde::Serialize)]
    struct SystemInfo {
        version: String,
        publish_date: String,
    }
    let system_info = SystemInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        publish_date: env!("CARGO_PKG_BUILD_TIMESTAMP").to_string(),
        // todo: Fix This
    };
    let system_info_json =
        serde_json::to_string_pretty(&system_info).expect("Failed to serialize system info");
    return system_info_json;
}

pub fn system_configure() {
    // Check for superuser privileges
    // if !nix::unistd::Uid::effective().is_root() && !cfg!(debug_assertions) {
    //     eprintln!("Error: This program must be run as root.");
    //     std::process::exit(1);
    // }
    // 環境変数 IPM_WORK_DIR を取得
    let work_dir = env::var("IPM_WORK_DIR").expect("環境変数 IPM_WORK_DIR が設定されていません");
    let _current_dir = env::current_dir().expect("現在のディレクトリを取得できません");
    // システムの作業ディレクトリに移動
    env::set_current_dir(&work_dir).expect("作業ディレクトリに移動できません");

    // 必要なディレクトリを作成
    // パッケージの保存場所
    create_dir_if_not_exists("package");
    create_file_if_not_exists("package/list.json");
    // wwwリポジトリデータの保存場所
    create_dir_if_not_exists("www");
    create_file_if_not_exists("www/list.json");
    // バイナリの保存場所
    create_dir_if_not_exists("bin");
    create_file_if_not_exists("bin/ipm-info.json");
    // 設定ファイルの保存場所
    create_dir_if_not_exists("setting");
    create_file_if_not_exists("setting/setting.json");
    // ログの保存場所
    create_dir_if_not_exists("log");
    create_file_if_not_exists("log/log.txt");
    // 一時ディレクトリの保存場所
    create_dir_if_not_exists("tmp");
    create_file_if_not_exists("tmp/tmp");
    // 本来の作業ディレクトリに移動
    env::set_current_dir(&_current_dir).expect("作業ディレクトリに移動できません");
}

fn create_dir_if_not_exists(dir: &str) {
    let path = Path::new(dir);
    if !path.exists() {
        fs::create_dir(path).expect(&format!("ディレクトリ {:?} の作成に失敗しました", path));
    }
}
fn create_file_if_not_exists(file: &str) {
    let path = Path::new(file);
    if !path.exists() {
        fs::File::create(path).expect(&format!("ファイル {:?} の作成に失敗しました", path));
    }
}

pub fn cleanup_tmp() {
    fs::remove_dir_all(system::tmp_dir()).expect("Failed to clean up tmp.");
    fs::create_dir(system::tmp_dir()).expect("Failed to recreate tmp.");
}
