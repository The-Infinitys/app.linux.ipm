use std::env;
use std::fs;
use std::path::Path;
use nix;

pub fn configure() {
    // Configure Information
    const DEBUG: bool = cfg!(debug_assertions);
    if DEBUG {
        println!("Debug mode is enabled.");
        match env::current_dir() {
            Ok(current_dir) => println!("Current directory: {:?}", current_dir),
            Err(e) => eprintln!("Failed to get current directory: {}", e),
        }
    }
    const IPM_WORK_DIR: &str = if DEBUG { "./tmp" } else { "/opt/ipm/" };
    unsafe {
        env::set_var("IPM_EXEC_MODE", if DEBUG { "debug" } else { "release" });
        env::set_var("IPM_WORK_DIR", IPM_WORK_DIR);
    }
}

pub fn system_configure(){
    // Check for superuser privileges
    if !nix::unistd::Uid::effective().is_root() {
        eprintln!("Error: This program must be run as root.");
        std::process::exit(1);
    }
    // 環境変数 IPM_WORK_DIR を取得
    let work_dir = env::var("IPM_WORK_DIR").expect("環境変数 IPM_WORK_DIR が設定されていません");
    let _current_dir = env::current_dir().expect("現在のディレクトリを取得できません");
    // システムの作業ディレクトリに移動
    env::set_current_dir(&work_dir).expect("作業ディレクトリに移動できません");

    // 必要なディレクトリを作成
    // パッケージの保存場所
    create_dir_if_not_exists("package");
    create_dir_if_not_exists("package/www");
    create_file_if_not_exists("package/www/list.json");
    create_dir_if_not_exists("package/installed");
    create_file_if_not_exists("package/installed/list.json");
    // バイナリの保存場所
    create_dir_if_not_exists("bin");
    create_file_if_not_exists("bin/ipm-info.md");
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