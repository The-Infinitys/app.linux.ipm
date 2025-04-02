use crate::core_library::package::PackageInfo;
use crate::utilities::shell::color_txt;
use std::path::Path;
use termimad::inline as md;

pub fn show(name: &str) {
    let package_dir = format!(
        "{}/package/{}",
        std::env::var("IPM_WORK_DIR").unwrap_or_default(),
        name
    );
    let package_dir = Path::new(&package_dir);
    if !package_dir.exists() {
        println!("Package does not exist: {:?}", package_dir);
        return;
    }
    let info_path = package_dir.join("information.json");
    if !info_path.exists() {
        println!("Information file does not exist: {:?}", info_path);
        return;
    }

    let info_content = std::fs::read_to_string(&info_path).unwrap_or_else(|_| {
        println!("Failed to read information file: {:?}", info_path);
        String::new()
    });

    let package_info: PackageInfo =
        serde_json::from_str(&info_content).expect("Failed to perse Information");
    show_from_info(&package_info);
}

pub fn show_from_info(package_info: &PackageInfo) {
    fn show_info(tag: &str, text: &str) {
        println!("{}{}", color_txt(tag, 0, 255, 0), text);
    }
    show_info(
        "name: ",
        &format!("{} ({})", &package_info.about.name, &package_info.about.id),
    );
    show_info("version: ", &package_info.about.version);
    show_info(
        "author: ",
        &format!(
            "{} ({})",
            &package_info.about.author.name, &package_info.about.author.id
        ),
    );
    let formatted_description = package_info
      .about
      .description
      .lines()
      .map(|line| format!("    {}", line))
      .collect::<Vec<_>>()
      .join("\n");
    let formatted_description = md(&formatted_description);
    let formatted_description = format!("{}",formatted_description);
    show_info("description: |\n", &formatted_description);
    let mut depend_info = String::new();
    for command_depend in &package_info.about.dependencies.command {
        depend_info = depend_info + &format!("    Command: {}\n", command_depend);
    }
    for package_depend in &package_info.about.dependencies.package {
        depend_info = depend_info
            + &format!(
                "    Package: {} (version: {},type: {})\n",
                package_depend.name, package_depend.version, package_depend.package_type
            );
    }
    show_info("dependencies: |\n", &depend_info);
    show_info("License: ", &package_info.about.license);
}
