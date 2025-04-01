use std::env;
use std::fs;
use std::path::Path;

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
    // 環境変数 IPM_WORK_DIR を取得
    let work_dir = env::var("IPM_WORK_DIR").expect("環境変数 IPM_WORK_DIR が設定されていません");
    let _current_dir = env::current_dir().expect("現在のディレクトリを取得できません");
    // システムの作業ディレクトリに移動
    env::set_current_dir(&work_dir).expect("作業ディレクトリに移動できません");

    // 必要なディレクトリを作成
    // パッケージの保存場所
    create_dir_if_not_exists("package");
    create_dir_if_not_exists("package/www");
    create_dir_if_not_exists("package/installed");
    // バイナリの保存場所
    create_dir_if_not_exists("bin");
    // 設定ファイルの保存場所
    create_dir_if_not_exists("setting");
    // ログの保存場所
    create_dir_if_not_exists("log");
    // 一時ディレクトリの保存場所
    create_dir_if_not_exists("tmp");
    // 本来の作業ディレクトリに移動
    env::set_current_dir(&_current_dir).expect("作業ディレクトリに移動できません");
}

fn create_dir_if_not_exists(dir: &str) {
    let path = Path::new(dir);
    if !path.exists() {
        fs::create_dir_all(path).expect(&format!("ディレクトリ {:?} の作成に失敗しました", path));
    }
}
