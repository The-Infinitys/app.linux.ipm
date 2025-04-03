use std::env;

pub fn show_welcome_msg() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const COMMAND_NAME: &'static str = env!("CARGO_PKG_NAME");
    const WELCOME_TEXT: &'static str = include_str!("welcome/welcome.txt");
    let text = WELCOME_TEXT
        .replace("{version}", VERSION)
        .replace("{command_name}", COMMAND_NAME);
    println!("{}", text);
}

pub fn show_version() {
    println!("IPM version: {}", env!("CARGO_PKG_VERSION"));
}
