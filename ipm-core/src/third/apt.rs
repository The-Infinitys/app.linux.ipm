pub mod package;
pub mod repository;

use std::collections::LinkedList;

use self::package::{AptPackageInfo, DebPackageInfo};
use crate::core::package::{About, Author, DependInfo, DependencyType};

impl AptPackageInfo {
    /// `AptPackageInfo` を `About` に変換する関数
    pub fn to_about(&self) -> About {
        let mut dep_info: LinkedList<DependInfo> = LinkedList::new();

        // Depends (必須依存関係)
        let mut depend_count = 0;
        for dep in &self.depends {
            depend_count += 1;
            for dep in dep.split('|') {
                let dep = dep.trim();
                let (name, version) = parse_dependency(dep);
                dep_info.push_back(DependInfo {
                    depend_type: DependencyType::Must,
                    name,
                    version,
                    index: Some(depend_count),
                });
            }
        }

        // Suggests (推奨依存関係)
        let mut depend_count = 0;
        for dep in &self.suggests {
            depend_count += 1;
            for dep in dep.split('|') {
                let dep = dep.trim();
                let (name, version) = parse_dependency(dep);
                dep_info.push_back(DependInfo {
                    depend_type: DependencyType::Should,
                    name,
                    version,
                    index: Some(depend_count),
                });
            }
        }

        // Recommends (推奨依存関係)
        let mut depend_count = 0;
        for dep in &self.recommends {
            depend_count += 1;
            for dep in dep.split('|') {
                let dep = dep.trim();
                let (name, version) = parse_dependency(dep);
                dep_info.push_back(DependInfo {
                    depend_type: DependencyType::May,
                    name,
                    version,
                    index: Some(depend_count),
                });
            }
        }

        // Conflicts (競合依存関係)
        let mut depend_count = 0;
        for dep in &self.conflicts {
            depend_count += 1;
            for dep in dep.split('|') {
                let dep = dep.trim();
                let (name, version) = parse_dependency(dep);
                dep_info.push_back(DependInfo {
                    depend_type: DependencyType::CannotConflict,
                    name,
                    version,
                    index: Some(depend_count),
                });
            }
        }

        // Breaks (破壊依存関係)
        let mut depend_count = 0;
        for dep in &self.breaks {
            depend_count += 1;
            for dep in dep.split('|') {
                let dep = dep.trim();
                let (name, version) = parse_dependency(dep);
                dep_info.push_back(DependInfo {
                    depend_type: DependencyType::CannotBreak,
                    name,
                    version,
                    index: Some(depend_count),
                });
            }
        }

        About {
            name: self.package.clone(),
            id: self.package.clone(),
            version: self.version.clone(),
            author: Author {
                name: self.maintainer.clone(),
                id: self.maintainer.clone(),
                email: String::new(), // メール情報は利用不可
            },
            description: self.description.clone(),
            license: String::new(), // ライセンス情報は利用不可
            dependencies: dep_info.into_iter().collect(),
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
                email: String::new(), // メール情報は利用不可
            },
            description: self.description.clone(),
            license: String::new(), // ライセンス情報は利用不可
            dependencies: self
                .depends
                .iter()
                .map(|dep| {
                    let (name, version) = parse_dependency(dep);
                    DependInfo {
                        depend_type: DependencyType::Must,
                        name,
                        version,
                        index: None,
                    }
                })
                .collect(),
            architecture: vec![self.architecture.clone()],
            size: self.installed_size as usize,
        }
    }
}

/// 依存関係文字列を名前とバージョンに分割する
///
/// # Arguments
/// * `dep` - 依存関係文字列 (例: "package (>= 1.0.0)")
///
/// # Returns
/// パッケージ名とバージョン要件のタプル
fn parse_dependency(dep: &str) -> (String, String) {
    if let Some(start) = dep.find('(') {
        let name = dep[..start].trim().to_string();
        let version = dep[start + 1..dep.len() - 1].trim().to_string();
        (name, version)
    } else {
        (dep.to_string(), "*".to_string())
    }
}
