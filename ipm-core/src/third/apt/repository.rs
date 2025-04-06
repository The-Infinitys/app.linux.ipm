use reqwest;
use std::io::Read;
use flate2::read::GzDecoder; // 変更

pub struct AptRepositoryInfo {
    name: String,
    url: String,
    suites: Vec<String>,
    components: Vec<String>,
    architectures: Vec<String>,
    signed_by: String,
    options: Vec<String>,
}

pub fn get_info(repo_info: AptRepositoryInfo) -> String {
    println!("Fetching repository info for: {}", repo_info.name);
    println!("Repository URL: {}", repo_info.url);
    println!("Suites: {:?}", repo_info.suites);
    println!("Components: {:?}", repo_info.components);
    println!("Architectures: {:?}", repo_info.architectures);
    println!("Options: {:?}", repo_info.options);
    println!("Signed by: {}", repo_info.signed_by);
    println!("Fetching repository info...");
    let mut result = String::new();
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
                                            result.push_str(&decompressed_data);
                                        }
                                        Err(e) => {
                                            println!("Failed to decompress data from {}: {}", url, e);
                                            println!("Compressed data size: {}", compressed_data.len());
                                            println!("First few bytes: {:?}", &compressed_data[..std::cmp::min(32, compressed_data.len())]);
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
    result
}

pub fn test() {
    let ubuntu_apt = AptRepositoryInfo {
        name: "ubuntu".to_string(),
        url: "http://archive.ubuntu.com/ubuntu/".to_string(),
        suites: vec!["focal".to_string(), "bionic".to_string()],
        components: vec!["main".to_string(), "universe".to_string()],
        architectures: vec!["amd64".to_string(), "i386".to_string()],
        options: vec!["trusted=yes".to_string()],
        signed_by: "".to_string(),
    };
    let result = get_info(ubuntu_apt);
    println!("Result:\n{}", result);
}
