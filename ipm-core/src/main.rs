use std::env;
mod core_library;
use core_library::help;
use core_library::package;
use core_library::system;
use core_library::welcome;
fn main() {
    system::configure::configure();
    println!("{}", env::current_dir().unwrap().display());
    // Prints each argument on a separate line
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        welcome::show_welcome_msg();
    } else if args.len() == 2 {
        sub_cmd(args[1].clone(), vec![]);
    } else {
        sub_cmd(args[1].clone(), args[2..].to_vec());
    }
}

fn sub_cmd(cmd_name: String, _args: Vec<String>) -> u8 {
    match &*cmd_name {
        "update" => println!("Run update!"),
        "search" => println!("Run search!"),
        "detail" => println!("Run detail!"),
        "help" => {
            if _args.len() > 0 {
                help::show_help_msg(&_args[0]);
            } else {
                help::show_help_msg("");
            }
        }
        "install" => {
            if _args.len() > 0 {
                package::install_package(_args);
            } else {
                help::show_help_msg("install");
            }
        }
        "version" => welcome::show_version(),
        _ => println!("Tried to run {}.\nHowever, not found.", cmd_name),
    }
    return 0;
}
