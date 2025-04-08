use std::collections::HashMap;

/// filepath: src/apt_package_info.rs
/// 構造体: AptPackageInfo
/// aptパッケージ情報を表現するための構造体
#[derive(Debug, PartialEq)]
pub struct AptPackageInfo {
    pub package: String,
    pub architecture: String,
    pub version: String,
    pub priority: String,
    pub section: String,
    pub origin: String,
    pub maintainer: String,
    pub original_maintainer: String,
    pub bugs: String,
    pub installed_size: u64,
    pub depends: Vec<String>,
    pub recommends: Vec<String>,
    pub suggests: Vec<String>,
    pub filename: String,
    pub size: u64,
    pub md5sum: String,
    pub sha1: String,
    pub sha256: String,
    pub sha512: String,
    pub homepage: String,
    pub description: String,
    pub task: Vec<String>,
    pub description_md5: String,
}

impl AptPackageInfo {
    /// 渡された文字列を解析して `AptPackageInfo` を生成する関数
    pub fn from_string(data: &str) -> Result<Self, String> {
        let mut fields: HashMap<String, String> = HashMap::new();

        // 各行を解析してフィールドを抽出
        for line in data.lines() {
            if let Some((key, value)) = line.split_once(": ") {
                fields.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        // 必須フィールドを取得し、構造体を生成
        Ok(AptPackageInfo {
            package: fields.get("Package").cloned().unwrap_or_default(),
            architecture: fields.get("Architecture").cloned().unwrap_or_default(),
            version: fields.get("Version").cloned().unwrap_or_default(),
            priority: fields.get("Priority").cloned().unwrap_or_default(),
            section: fields.get("Section").cloned().unwrap_or_default(),
            origin: fields.get("Origin").cloned().unwrap_or_default(),
            maintainer: fields.get("Maintainer").cloned().unwrap_or_default(),
            original_maintainer: fields
                .get("Original-Maintainer")
                .cloned()
                .unwrap_or_default(),
            bugs: fields.get("Bugs").cloned().unwrap_or_default(),
            installed_size: fields
                .get("Installed-Size")
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(0),
            depends: fields
                .get("Depends")
                .map(|v| v.split(", ").map(String::from).collect())
                .unwrap_or_default(),
            recommends: fields
                .get("Recommends")
                .map(|v| v.split(", ").map(String::from).collect())
                .unwrap_or_default(),
            suggests: fields
                .get("Suggests")
                .map(|v| v.split(", ").map(String::from).collect())
                .unwrap_or_default(),
            filename: fields.get("Filename").cloned().unwrap_or_default(),
            size: fields
                .get("Size")
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(0),
            md5sum: fields.get("MD5sum").cloned().unwrap_or_default(),
            sha1: fields.get("SHA1").cloned().unwrap_or_default(),
            sha256: fields.get("SHA256").cloned().unwrap_or_default(),
            sha512: fields.get("SHA512").cloned().unwrap_or_default(),
            homepage: fields.get("Homepage").cloned().unwrap_or_default(),
            description: fields.get("Description").cloned().unwrap_or_default(),
            task: fields
                .get("Task")
                .map(|v| v.split(", ").map(String::from).collect())
                .unwrap_or_default(),
            description_md5: fields.get("Description-md5").cloned().unwrap_or_default(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct DebPackageInfo {
    pub package: String,
    pub version: String,
    pub architecture: String,
    pub maintainer: String,
    pub installed_size: u64,
    pub depends: Vec<String>,
    pub conflicts: Vec<String>,
    pub breaks: Vec<String>,
    pub replaces: Vec<String>,
    pub section: String,
    pub priority: String,
    pub homepage: String,
    pub description: String,
    pub original_maintainer: String,
}

impl DebPackageInfo {
    /// 渡された文字列を解析して `DebPackageInfo` を生成する関数
    pub fn from_string(data: &str) -> Result<Self, String> {
        let mut fields: HashMap<String, String> = HashMap::new();

        // 各行を解析してフィールドを抽出
        for line in data.lines() {
            if let Some((key, value)) = line.split_once(": ") {
                fields.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        // 必須フィールドを取得し、構造体を生成
        Ok(DebPackageInfo {
            package: fields.get("Package").cloned().unwrap_or_default(),
            version: fields.get("Version").cloned().unwrap_or_default(),
            architecture: fields.get("Architecture").cloned().unwrap_or_default(),
            maintainer: fields.get("Maintainer").cloned().unwrap_or_default(),
            installed_size: fields
                .get("Installed-Size")
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(0),
            depends: fields
                .get("Depends")
                .map(|v| v.split(", ").map(String::from).collect())
                .unwrap_or_default(),
            conflicts: fields
                .get("Conflicts")
                .map(|v| v.split(", ").map(String::from).collect())
                .unwrap_or_default(),
            breaks: fields
                .get("Breaks")
                .map(|v| v.split(", ").map(String::from).collect())
                .unwrap_or_default(),
            replaces: fields
                .get("Replaces")
                .map(|v| v.split(", ").map(String::from).collect())
                .unwrap_or_default(),
            section: fields.get("Section").cloned().unwrap_or_default(),
            priority: fields.get("Priority").cloned().unwrap_or_default(),
            homepage: fields.get("Homepage").cloned().unwrap_or_default(),
            description: fields.get("Description").cloned().unwrap_or_default(),
            original_maintainer: fields
                .get("Original-Maintainer")
                .cloned()
                .unwrap_or_default(),
        })
    }
}
