use std::fs::File;
use std::io::{Write, Read};
use reqwest::blocking::{Client, Response};
use regex::Regex;

fn is_real_binary_link(link: &str) -> bool {
    let lower = link.to_lowercase();
    (lower.ends_with(".exe") || lower.ends_with(".msi"))
        && (lower.starts_with("http://") || lower.starts_with("https://"))
}

fn extract_all_exe_links(html: &str) -> Vec<String> {
    let re = Regex::new(r#"(?:href|src|URL)\s*=\s*["']([^"']+\.(exe|msi)[^"']*)["']"#)
        .unwrap();
    let mut links = Vec::new();
    for cap in re.captures_iter(html) {
        let link = cap[1].trim().to_string();
        if is_real_binary_link(&link) {
            links.push(link);
        }
    }
    links
}

fn find_real_link(html: &str) -> Option<String> {
    if let Some(pos) = html.find("URL=") {
        let after = &html[pos + 4..];
        let cleaned = after
            .trim_start_matches('\\')
            .trim_start_matches('\'')
            .trim_start_matches('"')
            .split(|c: char| c == '\'' || c == '"' || c == ' ' || c == '>' || c == '\\' || c == ';')
            .next()
            .unwrap_or("")
            .to_string();
        if is_real_binary_link(&cleaned) {
            return Some(cleaned);
        }
    }

    let all_links = extract_all_exe_links(html);
    if let Some(link) = all_links.iter().find(|l| l.to_lowercase().contains("download")) {
        return Some(link.clone());
    }

    all_links.into_iter().next()
}

fn save_response(mut response: Response, output_path: &str) -> Result<(), String> {
    let total_size = response.content_length().unwrap_or(45_000_000);
    let mut file = File::create(output_path).map_err(|e| format!("Dosya oluşturulamadı: {}", e))?;
    let mut downloaded: u64 = 0;
    let mut buffer = [0u8; 65536];

    println!("⏳ İndirme başladı...");
    loop {
        let bytes_read = response.read(&mut buffer).map_err(|e| format!("Okuma hatası: {}", e))?;
        if bytes_read == 0 { break; }
        file.write_all(&buffer[..bytes_read]).map_err(|e| format!("Yazma hatası: {}", e))?;
        downloaded += bytes_read as u64;

        let percent = (downloaded as f64 / total_size as f64) * 100.0;
        let bar_width = 30;
        let progress = ((percent / 100.0) * bar_width as f64) as usize;
        let bar = "█".repeat(progress) + &"-".repeat(bar_width - progress);
        print!(
            "\r[{}] {:.1}% ({:.2} MB / {:.2} MB)",
            bar,
            percent.min(100.0),
            downloaded as f64 / 1024.0 / 1024.0,
            total_size as f64 / 1024.0 / 1024.0
        );
        let _ = std::io::stdout().flush();
    }
    println!("\n✅ İndirme tamamlandı!");

    if downloaded < 500_000 {
        return Err(format!(
            "Dosya çok küçük ({} byte) – muhtemelen HTML.",
            downloaded
        ));
    }
    Ok(())
}

pub fn download_file(url: &str, output_path: &str) -> Result<(), String> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
        .redirect(reqwest::redirect::Policy::limited(15))
        .timeout(std::time::Duration::from_secs(120))
        .connect_timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("İstemci başlatılamadı: {}", e))?;

    let mut target_url = url.to_string();

    for attempt in 1..=3 {
        if attempt > 1 {
            println!("🔄 Yeniden deneniyor ({}/3)...", attempt);
            std::thread::sleep(std::time::Duration::from_secs(2));
        }

        let response = match client.get(&target_url).send() {
            Ok(r) => r,
            Err(e) => {
                println!("⚠️  İstek hatası: {}", e);
                continue;
            }
        };

        if !response.status().is_success() {
            println!("⚠️  Sunucu hatası: {}", response.status());
            continue;
        }

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_lowercase();

        if content_type.contains("text/html") {
            println!("⚠️  Sayfa algılandı, indirme linki aranıyor...");
            let html = match response.text() {
                Ok(t) => t,
                Err(e) => {
                    println!("⚠️  Sayfa okunamadı: {}", e);
                    continue;
                }
            };

            match find_real_link(&html) {
                Some(real_link) => {
                    println!("🎯 Gerçek link bulundu: {}", real_link);
                    target_url = real_link;
                    // Yeni URL ile döngü başa döner (tekrar dene)
                    continue;
                }
                None => {
                    return Err("Sayfada uygun .exe/.msi bağlantısı bulunamadı.".to_string());
                }
            }
        } else {
            return save_response(response, output_path);
        }
    }

    Err("3 deneme sonrası başarısız.".to_string())
}