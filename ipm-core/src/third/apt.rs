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
                    depend_type: "must".to_string(),
                    name: parse_dependency(&dep.clone()).0,
                    version: parse_dependency(&dep.clone()).1, // バージョン情報がないため空文字列
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
                    depend_type: "apt".to_string(),
                    name: parse_dependency(&dep.clone()).0,
                    version: parse_dependency(&dep.clone()).1, // バージョン情報がないため空文字列
                })
                .collect(),
            architecture: vec![self.architecture.clone()],
            size: self.installed_size as usize,
        }
    }
}

fn parse_dependency(dep: &str) -> (String, String) {
    if let Some(start) = dep.find('(') {
        let name = dep[..start].trim().to_string();
        let version = dep[start + 1..dep.len() - 1].trim().to_string();
        (name, version)
    } else {
        (dep.to_string(), "*".to_string())
    }
}
