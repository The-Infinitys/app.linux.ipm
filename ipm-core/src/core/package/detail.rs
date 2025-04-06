use crate::core::package::PackageInfo;
use crate::core::system;
use crate::utils::shell::color_txt;
use termimad::inline as md;

// IPM write system
use crate::write_error;
use crate::write_warn;

pub fn show(name: &str) {
    let package_dir = system::package_path().join(name);
    if !package_dir.exists() {
        write_warn!("Package does not exist: {:?}", package_dir);
        return;
    }
    let info_path = package_dir.join("information.json");
    if !info_path.exists() {
        write_warn!("Information file does not exist: {:?}", info_path);
        return;
    }

    let info_content = std::fs::read_to_string(&info_path).unwrap_or_else(|_| {
        write_error!("Failed to read information file: {:?}", info_path);
        String::new()
    });

    let package_info: PackageInfo =
        serde_json::from_str(&info_content).expect("Failed to parse Information");
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
    let formatted_description = format!("{}", formatted_description);
    show_info("description: |\n", &formatted_description);
    let mut depend_info_txt = String::new();
    for depend_info in &package_info.about.dependencies {
        depend_info_txt = depend_info_txt
            + &format!(
                "    {depend_type}: {name}(version: {version})\n",
                depend_type = depend_info.depend_type,
                name = depend_info.name,
                version = depend_info.version
            );
    }
    show_info("dependencies: |\n", &depend_info_txt);
    show_info("License: ", &package_info.about.license);
    let mut architecture_info = String::new();
    for architecture in &package_info.about.architecture {
        architecture_info = architecture_info + &format!("    {}\n", architecture);
    }
    show_info("Architecture: ", &architecture_info);
}
