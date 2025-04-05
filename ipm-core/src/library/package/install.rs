use crate::library::package::PackageInfo;
use crate::library::system;
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
    env::set_current_dir(&system::tmp_path()).expect("Failed to move current dir.");
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry = entry.file_name().into_string().unwrap_or_else(|_| {
                    eprintln!("Error: Failed to convert OsString to String.");
                    String::new()
                });
                env::set_current_dir(&system::tmp_path().join(&entry))
                    .expect("Failed to move current dir.");
                install_process();
            } else {
                eprintln!("Error: Failed to read an entry in the directory.");
            }
        }
    } else {
        eprintln!("Error: Failed to read the current directory.");
    }
}

fn install_process() {
    let mut package_info = String::new();
    let info_file_path = Path::new("./information.json");
    if info_file_path.exists() {
        let mut file = File::open(info_file_path).expect("Failed to open information file.");
        file.read_to_string(&mut package_info)
            .expect("Failed to read information file.");
    } else {
        eprintln!("Error: information file does not exist.");
        return;
    }
    let package_info: PackageInfo =
        serde_json::from_str(&package_info).expect("It is not valid PackageInfo data");
    let package_path = system::package_path().join(package_info.about.id);
    let cache_path =
        Path::new(&env::current_dir().expect("Failed to get cache_path")).to_path_buf();
    copy_directory(Path::new("./"), &package_path).expect("Failed to copy directory");
    // ディレクトリを移動、及びキャッシュの削除
    env::set_current_dir(&package_path).expect("Failed to move current dir.");
    fs::remove_dir_all(&cache_path).expect("Failed to remove cache");
    // グローバルファイル(実行ファイルなど)をリンク
    for global_file in &package_info.files.global {
        for to_path in &global_file.to {
            let from_path = package_path.join(&global_file.from);
            let from_path = std::fs::canonicalize(&from_path).expect("Failed to canonicalize");
            let to_path = Path::new("/").join(&to_path);
            println!("{:?}", &from_path);
            if to_path.exists() {
                fs::remove_file(&to_path).expect("Failed to remove existing file at to_path.");
            }
            match global_file.file_type.as_str() {
                "bin" => {
                    symlink(&from_path, to_path).expect("Failed to generate binary file.");
                }
                "data" => {
                    fs::copy(&from_path, to_path).expect("Failed to generate data file.");
                }

                "config" => {
                    fs::copy(&from_path, to_path).expect("Failed to generate config file.");
                }

                _ => {
                    eprintln!("Error: Unknown file type for global file.");
                }
            }
        }
    }
}
