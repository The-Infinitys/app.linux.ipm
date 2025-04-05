use chrono::Utc;

fn main(){
  let date = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
  println!("cargo:rustc-env=CARGO_PKG_BUILD_TIMESTAMP={}", date);
}
