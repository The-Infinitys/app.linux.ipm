use crate::core::system;
// use nix;
use std::env;
use std::fs;
use std::path::Path;

// IPM write system
use crate::write_info;

pub fn configure() {
    // Configure Environment Information
    const DEBUG: bool = cfg!(debug_assertions);
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let mut ipm_work_dir = env::current_exe().expect("Failed to get current exe.");
    if ipm_work_dir.is_symlink() {
        ipm_work_dir = fs::read_link(ipm_work_dir).expect("Failed to read link.");
    }
    ipm_work_dir = ipm_work_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    ipm_work_dir = std::fs::canonicalize(ipm_work_dir).expect("Failed to canonicalize");
    println!("{:?}", ipm_work_dir);
    let ipm_work_dir = ipm_work_dir.to_str().unwrap().to_string();
    const IPM_VERSION: &str = env!("CARGO_PKG_VERSION");
    unsafe {
        env::set_var("IPM_EXEC_MODE", if DEBUG { "debug" } else { "release" });
        env::set_var("IPM_WORK_DIR", &ipm_work_dir);
        env::set_var("IPM_CURRENT_DIR", &current_dir);
        env::set_var("IPM_VERSION", &IPM_VERSION);
    }
    if DEBUG {
        write_info!("Debug mode is enabled.");
        write_info!("Current directory: {:?}", &system::current_dir());
        write_info!("IPM Working directory: {:?}", &system::work_dir());
        write_info!("IPM Temporary directory: {:?}", &system::tmp_dir());
        write_info!("IPM Package directory: {:?}", &system::package_dir());
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
    //     write_error!("Error: This program must be run as root.");
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
    create_file_if_not_exists("package/list.json", Some("{}"));
    // wwwリポジトリデータの保存場所
    create_dir_if_not_exists("www");
    {
        let new_list = super::super::www::WwwList {
            list: Vec::new(),
            last_update: chrono::Local::now().to_rfc3339(),
        };
        let new_list = serde_json::to_string(&new_list).expect("Failed to serialize");
        create_file_if_not_exists("www/list.json", Some(&new_list));
    } // バイナリの保存場所
    create_dir_if_not_exists("bin");
    create_file_if_not_exists("bin/ipm-info.json", Some("{}"));
    // 設定ファイルの保存場所
    create_dir_if_not_exists("setting");
    create_file_if_not_exists("setting/setting.json", Some("{}"));
    // ログの保存場所
    create_dir_if_not_exists("log");
    create_file_if_not_exists("log/log.txt", None);
    // 一時ディレクトリの保存場所
    create_dir_if_not_exists("tmp");
    create_file_if_not_exists("tmp/tmp", None);
    // 本来の作業ディレクトリに移動
    env::set_current_dir(&_current_dir).expect("作業ディレクトリに移動できません");
}

fn create_dir_if_not_exists(dir: &str) {
    let path = Path::new(dir);
    if !path.exists() {
        fs::create_dir(path).expect(&format!("ディレクトリ {:?} の作成に失敗しました", path));
    }
}
fn create_file_if_not_exists(file: &str, data: Option<&str>) {
    let path = Path::new(file);
    if !path.exists() {
        let mut file =
            fs::File::create(path).expect(&format!("ファイル {:?} の作成に失敗しました", path));
        if let Some(content) = data {
            use std::io::Write;
            file.write_all(content.as_bytes()).expect(&format!(
                "ファイル {:?} にデータを書き込むのに失敗しました",
                path
            ));
        }
    }
}

pub fn cleanup_tmp() {
    fs::remove_dir_all(system::tmp_dir()).expect("Failed to clean up tmp.");
    fs::create_dir(system::tmp_dir()).expect("Failed to recreate tmp.");
}
