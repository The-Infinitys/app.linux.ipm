use std::env;
mod core_library;
use core_library::welcome::show_welcome_msg;

fn main() {
    // Prints each argument on a separate line
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        show_welcome_msg();
    } else {
        sub_cmd(args[1].clone());
    }
}

fn sub_cmd(cmd_name: String) -> u8 {
    match &*cmd_name {
        "update" => println!("Run update!"),
        "search" => println!("Run search!"),
        "detail" => println!("Run detail!"),
        _ => println!("Tried to run {}.\nHowever, not found.", cmd_name),
    }
    return 0;
}
