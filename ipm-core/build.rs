use chrono::Utc;

fn main() {
    let date = Utc::now().to_rfc3339();
    println!("cargo:rustc-env=CARGO_PKG_BUILD_TIMESTAMP={}", date);
}
