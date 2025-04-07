#[macro_export]
macro_rules! export_to_logfile {
    ($txt:expr) => {{
        use std::io::Write;
        use $crate::core::system;
        let logfile_path = system::logfile_path();
        // ログファイルに書き込む処理
        match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&logfile_path)
        {
            Ok(mut file) => {
                writeln!(file, "{}", $txt)
                    .unwrap_or_else(|e| eprintln!("Failed to write to log file: {}", e));
            }
            Err(e) => {
                eprintln!(
                    "Failed to open log file for writing: {}. Error: {}",
                    logfile_path.display(),
                    e
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! write_log {
    ($($arg:tt)*) => {{
        use $crate::export_to_logfile;
        use $crate::utils::shell::color_txt;
        use chrono::Local;

        // フォーマットされたログメッセージを作成
        let log_txt = format!($($arg)*);
        let colored_log = format!("[{}] {}", color_txt(" LOG ", 0, 255, 255), log_txt);

        // 標準出力にログを表示
        println!("{}", colored_log);

        // タイムスタンプ付きのログを作成
        let timestamped_log = format!("[{}][ LOG ] {}", Local::now().to_rfc3339(), log_txt);

        // ログをファイルに書き込む
        export_to_logfile!(timestamped_log);
    }};
}

#[macro_export]
macro_rules! write_info {
    ($($arg:tt)*) => {{
        use $crate::export_to_logfile;
        use $crate::utils::shell::color_txt;
        use chrono::Local;

        // フォーマットされたログメッセージを作成
        let log_txt = format!($($arg)*);
        let colored_log = format!("[{}] {}", color_txt("INFO", 0, 255, 0), log_txt);

        // 標準出力にログを表示
        println!("{}", colored_log);

        // タイムスタンプ付きのログを作成
        let timestamped_log = format!("[{}][INFO ] {}", Local::now().to_rfc3339(), log_txt);

        // ログをファイルに書き込む
        export_to_logfile!(timestamped_log);
    }};
}

#[macro_export]
macro_rules! write_warn {
    ($($arg:tt)*) => {{
        use $crate::export_to_logfile;
        use $crate::utils::shell::color_txt;
        use chrono::Local;

        // フォーマットされたログメッセージを作成
        let log_txt = format!($($arg)*);
        let colored_log = format!("[{}] {}", color_txt("WARN ", 255, 255, 0), log_txt);

        // 標準出力にログを表示
        println!("{}", colored_log);

        // タイムスタンプ付きのログを作成
        let timestamped_log = format!("[{}][WARN ] {}", Local::now().to_rfc3339(), log_txt);

        // ログをファイルに書き込む
        export_to_logfile!(timestamped_log);
    }};
}

#[macro_export]
macro_rules! write_error {
    ($($arg:tt)*) => {{
        use $crate::export_to_logfile;
        use $crate::utils::shell::color_txt;
        use chrono::Local;

        // フォーマットされたログメッセージを作成
        let log_txt = format!($($arg)*);
        let colored_log = format!("[{}] {}", color_txt("ERROR", 255, 0, 0), log_txt);

        // 標準出力にログを表示
        println!("{}", colored_log);

        // タイムスタンプ付きのログを作成
        let timestamped_log = format!("[{}][ERROR] {}", Local::now().to_rfc3339(), log_txt);

        // ログをファイルに書き込む
        export_to_logfile!(timestamped_log);
    }};
}

#[macro_export]
macro_rules! write_simple {
    ($($arg:tt)*) => {{
        use $crate::export_to_logfile;
        use $crate::utils::shell::color_txt;
        use chrono::Local;
        // フォーマットされたログメッセージを作成
        let log_txt = format!($($arg)*);
        // 標準出力にログを表示
        println!("{}", log_txt);
        // タイムスタンプ付きのログを作成
        let timestamped_log = format!("[{}][     ] {}", Local::now().to_rfc3339(), log_txt);
        // ログをファイルに書き込む
        export_to_logfile!(timestamped_log);
    }};
}
