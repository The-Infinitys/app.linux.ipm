pub mod configure;
use std::env;

pub fn ipm_work_dir() -> String{
  let work_dir = env::var("IPM_WORK_DIR").expect("Failed to get $IPM_WORK_DIR");
  return std::fs::canonicalize(work_dir)
      .expect("Failed to convert to absolute path")
      .to_str()
      .unwrap()
      .to_string();
}