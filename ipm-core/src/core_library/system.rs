pub mod init;
pub mod remove;
use std::env;

pub fn show_system_msg() {
    const COMMAND_NAME: &'static str = env!("CARGO_PKG_NAME");
    println!("System command");
    println!("Usage: ipm system <init|remove>");
    println!("Commands:");
    println!(
        "  init     Initialize the {command_name} system",
        command_name = COMMAND_NAME
    );
    println!(
        "  remove   Remove the {command_name} system",
        command_name = COMMAND_NAME
    );
}

pub fn run_system_cmd(cmd: &str,args: Vec<String>) {
    match cmd {
        "init" => init::init(args),
        "remove" => remove::remove(args),
        _ => println!("Unknown command: {}", cmd),
    }
}
