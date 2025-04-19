use crate::core::www;
use crate::utils::shell::color_txt;
use crate::write_error;
use crate::write_info;

pub fn search_packages(query: &str) {
    write_info!("Searching for packages matching: {}", query);

    let package_list = www::package_list();
    let mut found = false;

    for package in package_list {
        if package
            .about
            .name
            .to_lowercase()
            .contains(&query.to_lowercase())
        {
            found = true;
            println!("\nPackage: {}", color_txt(&package.about.name, 0, 255, 0));
            println!("URL: {}", color_txt(&package.package_url, 0, 200, 255));
            println!("Type: {}", color_txt(&package.package_type, 255, 200, 0));

            println!("----------------------------------------");
        }
    }

    if !found {
        write_error!("No packages found matching: {}", query);
    }
}
