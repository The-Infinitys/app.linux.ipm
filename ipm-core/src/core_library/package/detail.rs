use crate::core_library::package::PackageInfo;
use std::path::Path;
use crate::utilities::shell::color_txt;
pub fn show(name:&str){
  let package_dir = format!("{}/package/installed/{}", std::env::var("IPM_WORK_DIR").unwrap_or_default(), name);
  let package_dir = Path::new(&package_dir);
  if !package_dir.exists(){
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

  let package_info: PackageInfo = serde_json::from_str(&info_content).expect("Failed to perse Information");
  fn show_info(tag:&str,text:&str){
    println!("{}{}",color_txt(tag,0,255,0),text);
  }

  show_info("name: ",&package_info.about.id);
  show_info("version: ",&package_info.about.version);
  show_info("author: ",&package_info.about.author.id);
  let description_path = package_dir.join(&package_info.about.description);
  if description_path.exists(){
    let description = std::fs::read_to_string(&description_path).unwrap_or_else(|_| {
      println!("Failed to read description file: {:?}",description_path);
      String::new()
    });
    show_info("description: |\n\n", &description);
  } else {
    show_info("description: |\n\n",&package_info.about.description);
  }
}