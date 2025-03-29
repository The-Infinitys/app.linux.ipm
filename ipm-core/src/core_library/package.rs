pub fn install_package(args: Vec<String>) {
    // Function to install a package
    // Check if the first argument is a package name
    println!("Installing package...");
    if args.len() == 0 {
        println!("No package name provided.");
        return;
    }
    for arg in &args {
        println!("Argument: {}", arg);
    }
}

// pub fn uninstall_package(args: Vec<String>) {
//     // Function to uninstall a package
//     println!("Uninstalling package...");
// }
