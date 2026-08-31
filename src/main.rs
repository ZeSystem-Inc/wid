mod system;
mod repo;
mod network;

use std::env;
use std::fs;
use std::path::PathBuf;

fn get_temp_dir() -> PathBuf {
    let mut temp = env::temp_dir();
    temp.push("wid_downloads");
    temp
}

fn parse_version(ver: &str) -> Option<semver::Version> {
    let mut ver = ver.trim().to_string();
    let parts: Vec<&str> = ver.split('.').collect();
    if parts.len() == 2 {
        ver.push_str(".0");
    }
    semver::Version::parse(&ver).ok()
}

fn main() {
    if !cfg!(target_os = "windows") {
        println!("❌ wid yalnızca Windows işletim sisteminde çalışır.");
        return;
    }

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("🚀 wid - Windows Downloader (Canlı İnternet Havuzlu Paket Yöneticisi)");
        println!("Kullanım:");
        println!("  wid update                 -> İnternetteki paket listesini canlı indirir");
        println!("  wid search <kelime>        -> Paket havuzunda arama yapar");
        println!("  wid install <uygulama>     -> Uygulamayı indirip kurar");
        println!("  wid upgrade <uygulama>     -> Uygulamayı yeni sürüme yükseltir (sürüm kontrolü yaparak)");
        println!("  wid clean                  -> Geçici dosyaları temizler");
        return;
    }

    let command = args[1].to_lowercase();
    let apps_repo = repo::load_sources();

    match command.as_str() {
        "update" => {
            println!("🔄 Paket listeleri internet sunucularından senkronize ediliyor...");
            match repo::update_sources_from_internet() {
                Ok(count) => {
                    println!("✨ Başarılı! 'sources.list' güncellendi.");
                    println!("📦 Toplam {} paket aktif.", count);
                }
                Err(e) => println!("❌ Havuz güncellenirken hata: {}", e),
            }
        }
        "search" => {
            if args.len() < 3 {
                println!("❌ Lütfen aranacak kelimeyi girin! (Örn: wid search 7zip)");
                return;
            }
            let query = args[2].to_lowercase();
            println!("🔍 '{}' için arama yapılıyor...", query);
            let mut found = false;
            for app in &apps_repo {
                if app.name.contains(&query) {
                    println!("  🔹 {} [Sürüm: {}] - {}", app.name, app.version, app.file_name);
                    found = true;
                }
            }
            if !found { println!("❌ Eşleşen paket bulunamadı."); }
        }
        "install" | "upgrade" => {
            if args.len() < 3 {
                println!("❌ Lütfen bir uygulama adı belirtin! (Örn: wid {} 7zip)", command);
                return;
            }
            
            let mut target_app = args[2].clone();
            if target_app.starts_with("wid://") {
                target_app = target_app.trim_start_matches("wid://").trim_end_matches('/').to_string();
            }
            target_app = target_app.to_lowercase();

            if let Some(app) = apps_repo.iter().find(|a| a.name == target_app) {
                if command == "upgrade" && app.version == "0.0.0" {
                    println!("ℹ️  '{}' paketi için sürüm bilgisi mevcut değil.", app.name);
                    println!("   'upgrade' işlemi desteklenmiyor. Sadece ilk kurulum için 'install' kullanın.");
                    return;
                }

                if command == "upgrade" {
                    if let Some(installed_ver_str) = system::get_installed_version(&app.name) {
                        println!("📌 Yüklü sürüm: {}", installed_ver_str);
                        println!("📌 Havuzdaki sürüm: {}", app.version);

                        match (parse_version(&installed_ver_str), parse_version(&app.version)) {
                            (Some(installed), Some(available)) => {
                                if available > installed {
                                    println!("🆕 Yeni sürüm mevcut, yükseltme yapılıyor...");
                                } else {
                                    println!("✅ Zaten en güncel sürüm (veya daha yenisi) kullanılıyor.");
                                    return;
                                }
                            }
                            _ => {
                                println!("⚠️  Sürüm formatı karşılaştırılamadı. Lütfen manuel kontrol edin.");
                                println!("   Yüklü: {}", installed_ver_str);
                                println!("   Havuz: {}", app.version);
                                return;
                            }
                        }
                    } else {
                        println!("ℹ️  Yüklü sürüm bulunamadı, ilk kurulum yapılıyor...");
                    }
                } else {
                    println!("🎯 Kurulum: {}", app.name);
                }

                let temp_dir = get_temp_dir();
                if let Err(e) = fs::create_dir_all(&temp_dir) {
                    println!("❌ Geçici klasör oluşturulamadı: {}", e);
                    return;
                }

                let file_path = temp_dir.join(&app.file_name);
                let file_path_str = file_path.to_str().unwrap();

                match network::download_file(&app.url, file_path_str) {
                    Ok(_) => {
                        println!("⚙️ Kurulum başlatılıyor...");
                        match system::install_package(file_path_str) {
                            Ok(_) => {
                                println!("🎉 {} başarıyla kuruldu!", app.name);
                            }
                            Err(e) => {
                                println!("❌ Kurulum hatası: {}", e);
                            }
                        }

                        if let Err(e) = fs::remove_file(file_path_str) {
                            println!("⚠️  Geçici dosya silinemedi: {}", e);
                        } else {
                            println!("🧹 Geçici dosya temizlendi.");
                        }
                    }
                    Err(e) => println!("❌ İndirme hatası: {}", e),
                }
            } else {
                println!("❌ '{}' uygulaması havuzda bulunamadı!", target_app);
            }
        }
        "clean" => {
            let temp_dir = get_temp_dir();
            if temp_dir.exists() {
                println!("🧹 Geçici klasör temizleniyor: {}", temp_dir.display());
                match fs::remove_dir_all(&temp_dir) {
                    Ok(_) => println!("✅ Temizlik tamamlandı."),
                    Err(e) => println!("❌ Temizlik sırasında hata: {}", e),
                }
            } else {
                println!("ℹ️  Geçici klasör zaten temiz.");
            }
        }
        _ => println!("❌ Bilinmeyen komut! Kullanım: update, search, install, upgrade, clean"),
    }
}