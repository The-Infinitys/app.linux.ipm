use crate::core::system;
use crate::utils::shell::color_txt;
use std::fs;
use std::io::Write;
use std::path::Path;
pub fn init_server() {
    // Initialize the server
    println!("IPM Server initialized");
    let current_path = system::current_path();
    println!("Current path: {}", current_path.display());

    // Delete all files and folders in the current_path
    if let Err(e) = delete_all_in_directory(&current_path) {
        eprintln!(
            "Failed to delete files in {}: {}",
            current_path.display(),
            e
        );
    }
    // Configure the server
    let server_name = question("Enter server name", "int");
    println!("Server name: {}", server_name);
}

fn delete_all_in_directory(path: &Path) -> std::io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                fs::remove_dir_all(&entry_path)?; // Recursively remove directories
            } else {
                fs::remove_file(&entry_path)?; // Remove files
            }
        }
    }
    Ok(())
}
fn question(prompt: &str, answer_type: &str) -> String {
    let mut _error_msg = String::new();
    let prompt = color_txt(prompt, 0, 255, 0);
    let mut is_retry = false; // フラグを追加して再試行かどうかを判定
    loop {
        if is_retry {
            print!("\r\x1B[2K"); // 行をクリアしてカーソルを行頭に戻す
        }
        let full_prompt = if is_retry {
            format!("{}{}: ", _error_msg, prompt)
        } else {
            format!("{}: ", prompt)
        };

        // 質問を表示
        print!("{}", full_prompt);
        std::io::stdout().flush().expect("Failed to flush stdout");

        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        let answer = answer.trim();

        let result = match answer_type {
            "string" => Ok(answer.to_string()),
            "int" => answer
                .parse::<i32>()
                .map(|v| v.to_string())
                .map_err(|_| "invalid integer input".to_string()),
            "float" => answer
                .parse::<f64>()
                .map(|v| v.to_string())
                .map_err(|_| "invalid float input".to_string()),
            "yesno" => {
                let lower = answer.to_lowercase();
                if lower == "yes" || lower == "y" {
                    Ok("yes".to_string())
                } else if lower == "no" || lower == "n" {
                    Ok("no".to_string())
                } else {
                    Err("invalid yes/no input".to_string())
                }
            }
            _ => Err("Unsupported answer type".to_string()),
        };

        match result {
            Ok(valid_answer) => return valid_answer,
            Err(err) => {
                _error_msg = color_txt(
                    &format!("(Error:\"{}\" is {}. Please try again) ", answer, err),
                    255,
                    0,
                    0,
                );
                is_retry = true; // 再試行フラグを有効化
                continue; // 正しい入力が得られるまでループ
            }
        }
    }
}
