use std::env;
mod core_library;
use core_library::welcome::show_welcome_msg;

fn main() {
    // Prints each argument on a separate line
    for argument in env::args() {
        println!("{argument}");
    }
    show_welcome_msg();
}
