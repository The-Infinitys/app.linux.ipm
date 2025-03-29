use flate2::read::GzDecoder;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use tar::Archive;
mod install;
mod uninstall;

pub fn install_package(args: Vec<String>) {
    // Function to install a package
    println!("Installing package...");
    if args.is_empty() {
        println!("No package name or file path provided.");
        return;
    }

    for arg in &args {
        let path = Path::new(arg);
        if path.exists(){
            println!("File found: {}. Importing as local package...", arg);
            import_package_from_local(arg);
        } else {
            println!("Installing package: {}", arg);
        }
    }
}

pub fn import_package_from_local(file_path: &str) {
    // 1. ファイル、フォルダのデータを先に取得する
    let path = Path::new(file_path);
    if !path.exists() {
        eprintln!("File or directory does not exist: {}", file_path);
        return;
    }

    // データの取得
    let is_directory = path.is_dir();

    // 2. カレントディレクトリを ipm のワークディレクトリに変更する
    let ipm_work_dir =
        env::var("IPM_WORK_DIR").expect("環境変数 IPM_WORK_DIR が設定されていません");
    let current_dir = env::current_dir().unwrap();
    env::set_current_dir(&ipm_work_dir).expect("Failed to set current directory");

    // 3. 予め取得したデータを ./package/cache にコピーする
    let cache_dir = Path::new("./package/cache");
    if !cache_dir.exists() {
        fs::create_dir_all(cache_dir).expect("Failed to create cache directory");
    }

    if is_directory {
        // ディレクトリの場合
        println!("Detected directory. Copying to ./package/cache...");
        if let Err(e) = copy_directory(path, cache_dir) {
            eprintln!("Failed to copy directory: {}", e);
        } else {
            println!("Successfully copied directory to ./package/cache");
        }
    } else if let Some(extension) = path.extension() {
        // ファイルの場合
        match extension.to_str() {
            Some("gz") => {
                if let Some(parent_extension) =
                    path.file_stem().and_then(|s| Path::new(s).extension())
                {
                    if parent_extension == "tar" {
                        println!("Detected .tar.gz file. Extracting to ./package/cache...");
                        if let Err(e) = extract_tar_gz_to(file_path, cache_dir) {
                            eprintln!("Failed to extract .tar.gz file: {}", e);
                        }
                    } else {
                        eprintln!("Unsupported .gz file format: {}", file_path);
                    }
                }
            }
            Some("tar") => {
                println!("Detected .tar file. Extracting to ./package/cache...");
                if let Err(e) = extract_tar_to(file_path, cache_dir) {
                    eprintln!("Failed to extract .tar file: {}", e);
                }
            }
            _ => {
                println!("Copying file to ./package/cache...");
                if let Err(e) = fs::copy(path, cache_dir.join(path.file_name().unwrap())) {
                    eprintln!("Failed to copy file: {}", e);
                } else {
                    println!("Successfully copied file to ./package/cache");
                }
            }
        }
    } else {
        eprintln!("File has no extension: {}", file_path);
    }

    // 元のカレントディレクトリに戻す
    env::set_current_dir(current_dir).expect("Failed to set current directory");
}

fn copy_directory(src: &Path, dest: &Path) -> io::Result<()> {
    if !src.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source is not a directory",
        ));
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if entry_path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_directory(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }
    Ok(())
}

fn extract_tar_gz_to(file_path: &str, dest: &Path) -> io::Result<()> {
    let tar_gz = File::open(file_path)?;
    let decompressed = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(decompressed);
    archive.unpack(dest)?; // 指定されたディレクトリに展開
    println!("Successfully extracted .tar.gz file to {:?}", dest);
    Ok(())
}

fn extract_tar_to(file_path: &str, dest: &Path) -> io::Result<()> {
    let tar = File::open(file_path)?;
    let mut archive = Archive::new(tar);
    archive.unpack(dest)?; // 指定されたディレクトリに展開
    println!("Successfully extracted .tar file to {:?}", dest);
    Ok(())
}
