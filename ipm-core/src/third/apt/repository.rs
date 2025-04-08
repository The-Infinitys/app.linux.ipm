use super::package::AptPackageInfo;
use flate2::read::GzDecoder;
use reqwest;
use std::io::Read;

#[derive(Debug, PartialEq)]
pub struct AptReleaseInfo {
    pub hash: String,
    pub origin: String,
    pub label: String,
    pub suite: String,
    pub version: String,
    pub codename: String,
    pub date: String,
    pub architectures: Vec<String>,
    pub components: Vec<String>,
    pub description: String,
}

impl AptReleaseInfo {
    /// `Release`ファイルの内容を解析して `AptReleaseInfo` を生成する関数
    pub fn from_string(data: &str) -> Result<Self, String> {
        let mut fields = std::collections::HashMap::new();

        // 各行を解析してフィールドを抽出
        for line in data.lines() {
            if let Some((key, value)) = line.split_once(": ") {
                fields.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        // 必須フィールドを取得し、構造体を生成
        Ok(AptReleaseInfo {
            hash: fields.get("Hash").cloned().unwrap_or_default(),
            origin: fields.get("Origin").cloned().unwrap_or_default(),
            label: fields.get("Label").cloned().unwrap_or_default(),
            suite: fields.get("Suite").cloned().unwrap_or_default(),
            version: fields.get("Version").cloned().unwrap_or_default(),
            codename: fields.get("Codename").cloned().unwrap_or_default(),
            date: fields.get("Date").cloned().unwrap_or_default(),
            architectures: fields
                .get("Architectures")
                .map(|v| v.split_whitespace().map(String::from).collect())
                .unwrap_or_default(),
            components: fields
                .get("Components")
                .map(|v| v.split_whitespace().map(String::from).collect())
                .unwrap_or_default(),
            description: fields.get("Description").cloned().unwrap_or_default(),
        })
    }
}

pub struct AptRepositoryInfo {
    name: String,
    url: String,
    suites: Vec<String>,
    components: Vec<String>,
    architectures: Vec<String>,
    signed_by: String,
    options: Vec<String>,
}

pub fn get_info(repo_info: AptRepositoryInfo) -> Vec<AptPackageInfo> {
    println!("Fetching repository info for: {}", repo_info.name);
    println!("Repository URL: {}", repo_info.url);
    println!("Suites: {:?}", repo_info.suites);
    println!("Components: {:?}", repo_info.components);
    println!("Architectures: {:?}", repo_info.architectures);
    println!("Options: {:?}", repo_info.options);
    println!("Signed by: {}", repo_info.signed_by);
    println!("Fetching repository info...");
    let mut apt_index_data = String::new();
    for suite in &repo_info.suites {
        for component in &repo_info.components {
            for architecture in &repo_info.architectures {
                let url = format!(
                    "{}/dists/{}/{}/binary-{}/Packages.gz", // 拡張子を .gz に変更
                    repo_info.url, suite, component, architecture
                );
                println!("Fetching: {}", url);
                let response = reqwest::blocking::get(&url);
                match response {
                    Ok(res) => {
                        if res.status().is_success() {
                            println!("Successfully fetched: {}", url);
                            match res.bytes() {
                                Ok(compressed_data) => {
                                    let mut decoder = GzDecoder::new(&compressed_data[..]); // GzDecoder を使用
                                    let mut decompressed_data = String::new();
                                    match decoder.read_to_string(&mut decompressed_data) {
                                        Ok(_) => {
                                            apt_index_data.push_str(&decompressed_data);
                                            apt_index_data.push_str("\n");
                                        }
                                        Err(e) => {
                                            println!(
                                                "Failed to decompress data from {}: {}",
                                                url, e
                                            );
                                            println!(
                                                "Compressed data size: {}",
                                                compressed_data.len()
                                            );
                                            println!(
                                                "First few bytes: {:?}",
                                                &compressed_data
                                                    [..std::cmp::min(32, compressed_data.len())]
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Failed to read bytes from {}: {}", url, e);
                                }
                            }
                        } else {
                            println!("Failed to fetch {} with status: {}", url, res.status());
                        }
                    }
                    Err(e) => {
                        println!("Error during request to {}: {}", url, e);
                    }
                }
            }
        }
    }

    // パッケージ情報を解析して AptPackageInfo のリストを生成
    let apt_info_strs = apt_index_data.split("\n\n");
    let mut apt_package_infos = Vec::with_capacity(apt_info_strs.clone().into_iter().count() - 1);
    for apt_info_str in apt_info_strs {
        if let Ok(package_info) = AptPackageInfo::from_string(apt_info_str) {
            if apt_package_infos.len() != apt_package_infos.capacity() {
                apt_package_infos.push(package_info);
            }
        }
    }
    apt_package_infos
}

pub fn test() {
    let ubuntu_apt = AptRepositoryInfo {
        name: "ubuntu".to_string(),
        url: "http://archive.ubuntu.com/ubuntu/".to_string(),
        suites: vec!["noble".to_string()],
        components: vec!["main".to_string()],
        architectures: vec!["amd64".to_string()],
        options: vec!["trusted=yes".to_string()],
        signed_by: "".to_string(),
    };
    let apt_package_infos = get_info(ubuntu_apt);
    for package_info in apt_package_infos {
        println!("{:#?}", package_info);
    }
}
