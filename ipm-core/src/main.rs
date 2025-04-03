use std::env;
mod library;
mod utils;
use library::help;
use library::package;
use library::system;
use library::welcome;
fn main() {
    system::configure::configure();
    // Prints each argument on a separate line
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        welcome::show_welcome_msg();
    } else if args.len() == 2 {
        sub_cmd(args[1].clone(), vec![]);
    } else {
        sub_cmd(args[1].clone(), args[2..].to_vec());
    }
    env::set_current_dir(system::current_path()).expect("Failed to move current dir");
}

fn sub_cmd(cmd_name: String, _args: Vec<String>) -> u8 {
    match &*cmd_name {
        "system" => {
            if _args.len() > 0 {
                if _args[0] == "configure" {
                    system::configure::system_configure();
                }
            } else {
                help::show_help_msg("system");
            }
        }
        "list" => package::list::installed_packages(),
        "uninstall" => {
            if _args.len() > 0 {
                package::uninstall_packages(_args);
            } else {
                help::show_help_msg("uninstall");
            }
        }
        "update" => package::list::update(),
        "search" => println!("Run search!"),
        "detail" => {
            if _args.len() > 0 {
                for package_name in &_args {
                    package::detail::show(&package_name);
                }
            }
        }
        "help" => {
            if _args.len() > 0 {
                help::show_help_msg(&_args[0]);
            } else {
                help::show_help_msg("");
            }
        }
        "install" => {
            if _args.len() > 0 {
                package::install_packages(_args);
            } else {
                help::show_help_msg("install");
            }
        }
        "version" => welcome::show_version(),
        _ => println!("Tried to run {}.\nHowever, not found.", cmd_name),
    }
    return 0;
}
