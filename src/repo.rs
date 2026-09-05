use std::fs::{File, self};
use std::path::PathBuf;
use std::io::{Write, BufReader, BufRead};
use reqwest::blocking::Client;

#[derive(Debug, Clone)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub url64: String,
    pub file64: String,
    pub url32: Option<String>,
    pub file32: Option<String>,
}

fn get_sources_path() -> PathBuf {
    let mut path = std::env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    path.push("wid");
    path.push("sources.list");
    path
}

pub fn update_sources_from_internet() -> Result<usize, String> {
    println!("🌐 Paket havuzu güncelleniyor...");
    let client = Client::builder()
        .user_agent("wid-core/2.1")
        .build()
        .map_err(|e| e.to_string())?;

    let remote_url = "https://gist.githubusercontent.com/ZeSystem-Inc/7c6d8add3143e6dff7cd619dd9c1085d/raw";
    println!("📡 Uzak havuz: {}", remote_url);

    let response = client.get(remote_url).send()
        .map_err(|e| format!("İnternet hatası: {}", e))?;
    if !response.status().is_success() {
        return Err(format!("HTTP {}", response.status()));
    }

    let content = response.text().map_err(|e| e.to_string())?;
    if content.trim().len() < 30 {
        return Err("Uzak dosya boş veya geçersiz.".to_string());
    }

    let path = get_sources_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if path.exists() {
        let _ = fs::copy(&path, path.with_extension("bak"));
        println!("💾 Eski sources.list yedeklendi.");
    }

    let mut file = File::create(&path).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;

    let count = load_sources().len();
    println!("✅ sources.list güncellendi ({} paket)", count);
    Ok(count)
}

pub fn load_sources() -> Vec<AppInfo> {
    let mut apps = Vec::new();
    let path = get_sources_path();

    if !path.exists() {
        let _ = update_sources_from_internet();
    }

    if let Ok(file) = File::open(&path) {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }
            let parts: Vec<String> = trimmed.split('|').map(|s| s.trim().to_string()).collect();

            if parts.len() == 6 {
                apps.push(AppInfo {
                    name: parts[0].to_lowercase(),
                    version: parts[1].clone(),
                    url64: parts[2].clone(),
                    file64: parts[3].clone(),
                    url32: if parts[4].is_empty() { None } else { Some(parts[4].clone()) },
                    file32: if parts[5].is_empty() { None } else { Some(parts[5].clone()) },
                });
            }
        }
    }
    apps
}