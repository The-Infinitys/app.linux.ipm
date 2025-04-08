use std::io::Write;
use std::process::Command;

pub fn color_txt(txt: &str, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m{}\x1b[m", r, g, b, txt)
}

#[allow(dead_code)]
pub struct ShellResult {
    pub status: u8,
    pub output: String,
}

#[allow(dead_code)]
pub fn cmd(cmd: &str) -> ShellResult {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Failed to execute command");

    ShellResult {
        status: output.status.code().unwrap_or(1) as u8,
        output: String::from_utf8_lossy(&output.stdout).to_string(),
    }
}

pub fn question(prompt: &str, answer_type: &str) -> String {
    let mut _error_msg = String::new();
    let pascal_regex = regex::Regex::new(r"^[A-Z][a-zA-Z0-9]*$").unwrap();
    let camel_regex = regex::Regex::new(r"^[a-z][a-zA-Z0-9]*$").unwrap();
    let snake_regex = regex::Regex::new(r"^[a-z0-9_]+$").unwrap();
    let constant_regex = regex::Regex::new(r"^[A-Z0-9_]+$").unwrap();
    let kebab_regex = regex::Regex::new(r"^[a-z0-9\-]+$").unwrap();
    let email_regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
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
            "camel" => {
                if camel_regex.is_match(answer) {
                    Ok(answer.to_string())
                } else {
                    Err("invalid camel case input".to_string())
                }
            }
            "pascal" => {
                if pascal_regex.is_match(answer) {
                    Ok(answer.to_string())
                } else {
                    Err("invalid pascal case input".to_string())
                }
            }
            "snake" => {
                if snake_regex.is_match(answer) {
                    Ok(answer.to_string())
                } else {
                    Err("invalid snake case input".to_string())
                }
            }
            "constant" => {
                if constant_regex.is_match(answer) {
                    Ok(answer.to_string())
                } else {
                    Err("invalid constant case input".to_string())
                }
            }
            "kebab" => {
                if kebab_regex.is_match(answer) {
                    Ok(answer.to_string())
                } else {
                    Err("invalid kebab case input".to_string())
                }
            }
            "email" => {
                if email_regex.is_match(answer) {
                    Ok(answer.to_string())
                } else {
                    Err("invalid email input".to_string())
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
