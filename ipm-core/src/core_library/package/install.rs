use crate::core_library::package::Dependencies;
use crate::core_library::package::PackageInfo;
use crate::core_library::package::detail;
use serde_json;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::os::unix::fs::symlink;
use std::path::Path;

fn copy_directory(src: &Path, dest: &Path) -> io::Result<()> {
    if !src.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source is not a directory",
        ));
    }

    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if entry_path.is_dir() {
            copy_directory(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}

pub fn install_packages() {
    println!("Starting package installation...");
    if let Ok(work_dir) = env::var("IPM_WORK_DIR") {
        if let Err(e) = env::set_current_dir(&work_dir) {
            eprintln!("Error: Failed to change directory to {}: {}", work_dir, e);
        } else {
            println!("Successfully changed directory to {}", work_dir);
        }
    } else {
        eprintln!("Error: IPM_WORK_DIR environment variable is not set.");
    }
    if let Err(e) = env::set_current_dir("./tmp") {
        eprintln!("Error: Failed to change directory to ./tmp: {}", e);
    } else {
        println!("Successfully changed directory to ./tmp");
    }
    // ここでパッケージのインストール処理を行う
    if let Ok(entries) = fs::read_dir(".") {
        let package_count = fs::read_dir(".").into_iter().count();
        println!(
            "Installing {package_count} packages...",
            package_count = package_count
        );
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    println!("Directory: {}", path.display());
                    if let Err(e) = env::set_current_dir(&path) {
                        eprintln!(
                            "Error: Failed to change directory to {}: {}",
                            path.display(),
                            e
                        );
                    } else {
                        println!("Successfully changed directory to {}", path.display());
                        // ここでパッケージのインストール処理を行う
                        install_process();
                    }
                    env::set_current_dir("..").unwrap_or_else(|e| {
                        eprintln!("Error: Failed to change directory to ..: {}", e);
                    });
                } else {
                    println!("File: {}", path.display());
                }
            }
        }
    } else {
        eprintln!("Error: Failed to read the directory.");
    }
}

fn install_process() {
    let mut package_info = String::new();
    let info_file_path = Path::new("information.json");
    if info_file_path.exists() {
        if let Ok(mut file) = File::open(&info_file_path) {
            if let Err(e) = file.read_to_string(&mut package_info) {
                eprintln!("Error: Failed to read 'information.json': {}", e);
            } else {
                println!("Successfully loaded package information.");
            }
        } else {
            eprintln!("Error: Failed to open 'information.json'.");
        }
    } else {
        eprintln!("Error: 'information.json' does not exist.");
    }
    let package_info: Result<PackageInfo, _> = serde_json::from_str(&package_info);
    match package_info {
        Ok(info) => {
            detail::show_from_info(&info);
            if !check_dependencies(info.about.dependencies) {
                eprintln!("依存関係を修正できません。");
                return;
            }
            let destination_dir = Path::new("../../package");
            if !destination_dir.exists() {
                if let Err(e) = fs::create_dir_all(&destination_dir) {
                    eprintln!(
                        "Error: Failed to create destination directory {}: {}",
                        destination_dir.display(),
                        e
                    );
                    return;
                } else {
                    println!(
                        "Successfully created destination directory: {}",
                        destination_dir.display()
                    );
                }
            }

            let package_dir_name = info.about.id.clone();
            let destination_path = destination_dir.join(&package_dir_name);
            if let Err(e) = copy_directory(Path::new("."), &destination_path) {
                eprintln!(
                    "Error: Failed to move package to {}: {}",
                    destination_path.display(),
                    e
                );
            } else {
                println!(
                    "Successfully moved package to {}",
                    destination_path.display()
                );
            }
            let cache_dir = env::current_dir().expect("Failed to get current dir.");
            env::set_current_dir(&destination_path).expect("Failed to set current dir.");
            {
                // Install process.
                // Global installation.
                for global_file_set in info.files.global {
                    for to_path in global_file_set.to {
                        let absolute_to_path = Path::new("/").join(to_path);
                        if let Some(parent) = absolute_to_path.parent() {
                            if !parent.exists() {
                                if let Err(e) = fs::create_dir_all(parent) {
                                    eprintln!(
                                        "Error: Failed to create directory {}: {}",
                                        parent.display(),
                                        e
                                    );
                                    continue;
                                } else {
                                    println!(
                                        "Successfully created directory: {}",
                                        parent.display()
                                    );
                                }
                            }
                        }
                        if let Err(e) = fs::remove_file(&absolute_to_path) {
                            if e.kind() != io::ErrorKind::NotFound {
                                eprintln!(
                                    "Error: Failed to remove existing file {}: {}",
                                    absolute_to_path.display(),
                                    e
                                );
                            }
                        } else {
                            println!(
                                "Successfully removed existing file: {}",
                                absolute_to_path.display()
                            );
                        }

                        let absolute_from_path = Path::new(&env::current_dir().expect("failed"))
                            .join(&global_file_set.from);
                        if let Err(e) = symlink(&absolute_from_path, &absolute_to_path) {
                            eprintln!(
                                "Error: Failed to create symlink from {:?} to {}: {}",
                                global_file_set.from,
                                absolute_to_path.display(),
                                e
                            );
                        } else {
                            println!(
                                "Successfully created symlink from {:?} to {}",
                                global_file_set.from,
                                absolute_to_path.display()
                            );
                        }
                    }
                }
                // Local installation
                for local_file_set in info.files.local {
                    for to_path in local_file_set.to {
                        let home_dirs = fs::read_dir("/home").unwrap();
                        for home_dir in home_dirs {
                            if let Ok(home_dir) = home_dir {
                                let user_home = home_dir.path();
                                if user_home.is_dir() {
                                    let absolute_to_path = user_home.join(&to_path);
                                    if let Some(parent) = absolute_to_path.parent() {
                                        if !parent.exists() {
                                            if let Err(e) = fs::create_dir_all(parent) {
                                                eprintln!(
                                                    "Error: Failed to create directory {}: {}",
                                                    parent.display(),
                                                    e
                                                );
                                                continue;
                                            } else {
                                                println!(
                                                    "Successfully created directory: {}",
                                                    parent.display()
                                                );
                                            }
                                        }
                                    }
                                    if let Err(e) =
                                        fs::copy(&local_file_set.from, &absolute_to_path)
                                    {
                                        eprintln!(
                                            "Error: Failed to copy file from {} to ~/{}: {}",
                                            local_file_set.from, to_path, e
                                        );
                                    } else {
                                        println!(
                                            "Successfully copied file from {} to ~/{}",
                                            local_file_set.from, to_path
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            env::set_current_dir(cache_dir).expect("Failed to set current dir.");
        }
        Err(e) => {
            eprintln!("Error: Failed to parse package information: {}", e);
            return;
        }
    }
}

fn check_dependencies(info: Dependencies) -> bool {
    for _depend_cmd in info.command {}
    return true;
}
