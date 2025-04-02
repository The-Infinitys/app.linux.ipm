use std::process::Command;

pub fn color_txt(txt: &str, r: u8, g: u8, b: u8) -> String {
  format!("\x1b[38;2;{};{};{}m{}\x1b[m", r, g, b, txt)
}

#[allow(dead_code)] // ここだけ
pub struct ShellResult{
  pub status:u8,
  pub output:String
}
pub fn cmd(cmd:&str) -> ShellResult {
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