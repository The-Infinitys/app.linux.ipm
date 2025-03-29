// ∆use std::io;
use std::io::{self, Write};

pub fn init(args: Vec<String>) {
    println!("Init IPM Core System");
    match args.len() {
        0 => {
            print!("Do you want to proceed? (y/n): ");
            io::stdout().flush().unwrap();
            loop {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                match input.trim() {
                    "y" | "Y" | "yes" | "Yes" | "YES" => {
                        // Execute the init script
                        const INIT_SCRIPT: &'static str = include_str!("init.sh");
                        match std::env::current_dir() {
                            Ok(x) => {
                                let output = Command::new(INIT_SCRIPT)
                                    .current_dir(x)
                                    .output()
                                    .expect("failed to execute process");
                            }
                            Err(err) => {
                                eprintln!("{:?}", err);
                            }
                        }
                        break;
                    }
                    "n" | "N" | "no" | "No" | "NO" => {
                        println!("Operation canceled.");
                        break;
                    }
                    _ => {
                        print!("Invalid input. Please enter 'y' or 'n': ");
                        io::stdout().flush().unwrap();
                    }
                }
            }
        }
        1 => {
            if args[0] == "local" {
                // Execute the init script
                const INIT_SCRIPT: &'static str = include_str!("init.local.sh");
                match std::env::current_dir() {
                    Ok(x) => {
                        let output = Command::new(INIT_SCRIPT)
                            .current_dir(x)
                            .output()
                            .expect("failed to execute process");
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            } else if args[0] == "global" {
                // Execute the init script
                const INIT_SCRIPT: &'static str = include_str!("init.global.sh");
                match std::env::current_dir() {
                    Ok(x) => {
                        let output = Command::new(INIT_SCRIPT)
                            .current_dir(x)
                            .output()
                            .expect("failed to execute process");
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            } else {
                println!("Invalid argument. Use 'local' or 'global'.");
                return;
            }
        }
        _ => {
            println!("Invalid number of arguments. Use 'force' to force initialization.");
            return;
        }
    }
}
