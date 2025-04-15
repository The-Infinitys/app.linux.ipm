pub mod package;
pub mod repository;

use std::collections::LinkedList;

use self::package::{AptPackageInfo, DebPackageInfo};
use crate::core::package::{About, Author, DependInfo};

impl AptPackageInfo {
    /// `AptPackageInfo` を `About` に変換する関数
    pub fn to_about(&self) -> About {
        let mut dep_info: LinkedList<DependInfo> = LinkedList::new();
        let mut depend_count = 0;
        for dep in &self.depends {
            depend_count += 1;
            for dep in dep.split("|") {
                let dep = dep.trim();
                let (name, version) = parse_dependency(dep);
                let adding_dep_info = DependInfo {
                    depend_type: format!("must.index.{}", depend_count),
                    name: name.to_string(),
                    version: version.to_string(),
                };
                dep_info.push_back(adding_dep_info);
            }
        }
        let mut depend_count = 0;
        for dep in &self.suggests {
            depend_count += 1;
            for dep in dep.split("|") {
                let dep = dep.trim();
                let (name, version) = parse_dependency(dep);
                let adding_dep_info = DependInfo {
                    depend_type: format!("should.index.{}", depend_count),
                    name: name.to_string(),
                    version: version.to_string(),
                };
                dep_info.push_back(adding_dep_info);
            }
        }
        let mut depend_count = 0;
        for dep in &self.recommends {
            depend_count += 1;
            for dep in dep.split("|") {
                let dep = dep.trim();
                let (name, version) = parse_dependency(dep);
                let adding_dep_info = DependInfo {
                    depend_type: format!("may.index.{}", depend_count),
                    name: name.to_string(),
                    version: version.to_string(),
                };
                dep_info.push_back(adding_dep_info);
            }
        }
        let mut depend_count = 0;
        for dep in &self.conflicts {
            depend_count += 1;
            for dep in dep.split("|") {
                let dep = dep.trim();
                let (name, version) = parse_dependency(dep);
                let adding_dep_info = DependInfo {
                    depend_type: format!("cannot.conflict.index.{}", depend_count),
                    name: name.to_string(),
                    version: version.to_string(),
                };
                dep_info.push_back(adding_dep_info);
            }
        }
        let mut depend_count = 0;
        for dep in &self.breaks {
            depend_count += 1;
            for dep in dep.split("|") {
                let dep = dep.trim();
                let (name, version) = parse_dependency(dep);
                let adding_dep_info = DependInfo {
                    depend_type: format!("cannot.break.index.{}", depend_count),
                    name: name.to_string(),
                    version: version.to_string(),
                };
                dep_info.push_back(adding_dep_info);
            }
        }

        let mut depend_count = 0;
        for dep in &self.replaces {
            depend_count += 1;
            for dep in dep.split("|") {
                let dep = dep.trim();
                let (name, version) = parse_dependency(dep);
                let adding_dep_info = DependInfo {
                    depend_type: format!("replace.index.{}", depend_count),
                    name: name.to_string(),
                    version: version.to_string(),
                };
                dep_info.push_back(adding_dep_info);
            }
        }
        let mut depend_count = 0;
        for dep in &self.enhances {
            depend_count += 1;
            for dep in dep.split("|") {
                let dep = dep.trim();
                let (name, version) = parse_dependency(dep);
                let adding_dep_info = DependInfo {
                    depend_type: format!("extension.index.{}", depend_count),
                    name: name.to_string(),
                    version: version.to_string(),
                };
                dep_info.push_back(adding_dep_info);
            }
        }
        let dep_info = dep_info.into_iter().collect();
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
            dependencies: dep_info,
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
