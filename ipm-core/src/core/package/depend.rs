use crate::core::package::{PackageInfo, DependInfo};
use crate::core::package::list;

#[derive(Debug, Clone)]
pub struct PackageDepend {
    name: String,
    info: Vec<DependInfo>,
}

#[derive(Debug, Clone)]
pub struct DependGraph {
    depends: Vec<PackageDepend>,
    count: usize,
}

pub fn complete_dependencies() -> Result<(), String> {
    // This function checks the dependencies of the core.
    // It will ensure that all required dependencies are present and up to date.
    // If any dependencies are missing or outdated, it will return an error.
    // Otherwise, it will return a success message.
    let depend_graph = get_dependencies_from_list(&list::data());

    // Check for missing or outdated dependencies
    for package_depend in depend_graph.depends {
        for depend_info in package_depend.info {
            // Here you would implement the logic to check if the dependency
            // specified by depend_info is present and up-to-date.
            // For this example, we'll just print the dependency info.
            println!("Checking dependency: {} for package: {}", depend_info.name, package_depend.name);

            // Example check (replace with your actual logic):
            if !is_dependency_satisfied(&depend_info) {
                return Err(format!("Dependency {} for package {} is not satisfied.", depend_info.name, package_depend.name));
            }
        }
    }

    Ok(()) // All dependencies are satisfied
}

pub fn get_dependencies_from_list(data: &Vec<PackageInfo>) -> DependGraph {
    let mut graph = DependGraph {
        depends: Vec::with_capacity(data.len()),
        count: data.len(),
    };
    for package in data {
        let package_depend = PackageDepend {
            name: package.about.id.clone(),
            info: package.about.dependencies.clone(),
        };
        graph.depends.push(package_depend);
    }
    graph
}

// Placeholder function for checking if a dependency is satisfied
fn is_dependency_satisfied(depend_info: &DependInfo) -> bool {
    // Replace this with your actual dependency checking logic.
    // This is just a placeholder that always returns true.
    // You might check if the dependency is installed, if the version is correct, etc.
    println!("Checking if dependency {} is satisfied", depend_info.name);
    true
}
