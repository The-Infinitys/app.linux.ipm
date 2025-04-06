use crate::core::package::PackageInfo;

pub struct repository_info {
   name: String,
   url: String,
   repo_type: String,
   package_list: Vec<PackageInfo>,
}
