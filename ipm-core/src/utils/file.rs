use std::fs;
use std::path::Path;

pub fn delete_all_in_directory(path: &Path) -> std::io::Result<()> {
  if path.is_dir() {
      for entry in fs::read_dir(path)? {
          let entry = entry?;
          let entry_path = entry.path();
          if entry_path.is_dir() {
              fs::remove_dir_all(&entry_path)?; // Recursively remove directories
          } else {
              fs::remove_file(&entry_path)?; // Remove files
          }
      }
  }
  Ok(())
}