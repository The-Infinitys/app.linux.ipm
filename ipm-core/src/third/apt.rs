pub mod package;
pub mod repository;

use self::package::{AptPackageInfo, DebPackageInfo};
use crate::core::package::{About, Author, DependInfo};

impl AptPackageInfo {
    /// `AptPackageInfo` を `About` に変換する関数
    pub fn to_about(&self) -> About {
        About {
            name: self.package.clone(),
            id: self.package.clone(),
            version: self.version.clone(),
            author: Author {
                name: self.maintainer.clone(),
                id: self.maintainer.clone(),
                email: String::new(), // メール情報がないため空文字列
            },
            description: self.description.clone(),
            license: String::new(), // ライセンス情報がないため空文字列
            dependencies: self
                .depends
                .iter()
                .map(|dep| DependInfo {
                    depend_type: "apt".to_string(),
                    name: dep.clone(),
                    version: String::new(), // バージョン情報がないため空文字列
                })
                .collect(),
            architecture: vec![self.architecture.clone()],
            size: self.installed_size as usize,
        }
    }
}

impl DebPackageInfo {
    /// `DebPackageInfo` を `About` に変換する関数
    pub fn to_about(&self) -> About {
        About {
            name: self.package.clone(),
            id: self.package.clone(),
            version: self.version.clone(),
            author: Author {
                name: self.maintainer.clone(),
                id: self.maintainer.clone(),
                email: String::new(), // メール情報がないため空文字列
            },
            description: self.description.clone(),
            license: String::new(), // ライセンス情報がないため空文字列
            dependencies: self
                .depends
                .iter()
                .map(|dep| DependInfo {
                    depend_type: "deb".to_string(),
                    name: dep.clone(),
                    version: String::new(), // バージョン情報がないため空文字列
                })
                .collect(),
            architecture: vec![self.architecture.clone()],
            size: self.installed_size as usize,
        }
    }
}
