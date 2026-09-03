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

fn is_admin() -> bool {
    use std::process::Command;
    let output = Command::new("net")
        .args(&["session"])
        .output();
    output.is_ok()
}

fn main() {
    if !cfg!(target_os = "windows") {
        println!("❌ wid yalnızca Windows işletim sisteminde çalışır.");
        return;
    }

    if !is_admin() {
        println!("🔑 Yönetici yetkisi gerekiyor, UAC yükseltiliyor...");
        let exe_path = std::env::current_exe().unwrap().to_str().unwrap().to_string();
        let status = std::process::Command::new("powershell")
            .args(&[
                "-Command",
                &format!(
                    "Start-Process -FilePath '{}' -Verb RunAs -Wait",
                    exe_path
                )
            ])
            .status()
            .unwrap();
        std::process::exit(status.code().unwrap_or(0));
    }

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("🚀 wid - Windows Downloader v1.2.3");
        println!("Kullanım:");
        println!("  wid list                  -> Havuzdaki tüm paketleri listele");
        println!("  wid info <uygulama>       -> Paket hakkında detaylı bilgi");
        println!("  wid update                -> Paket listesini güncelle");
        println!("  wid search <kelime>       -> Havuzda arama yap");
        println!("  wid install <uygulama>    -> Uygulama kur");
        println!("  wid upgrade <uygulama>    -> Uygulamayı yükselt (akıllı sürüm kontrolü)");
        println!("  wid upgrade all           -> Tüm paketleri güncelle (akıllı sürüm kontrolü)");
        println!("  wid upgrade --all         -> Tüm paketleri güncelle (akıllı sürüm kontrolü)");
        println!("  wid remove <uygulama>     -> Kurulu uygulamayı kaldır (UAC)");
        println!("  wid clean                 -> Geçici dosyaları temizle");
        println!("  wid format                -> Öğrenilen flag'leri sıfırla (hafıza temizleme)");
        println!("  wid --version             -> Wid sürümünü göster");
        return;
    }

    let command = args[1].to_lowercase();
    let apps_repo = repo::load_sources();

    match command.as_str() {
        "list" => {
            println!("📦 Havuzdaki paketler ({} adet):", apps_repo.len());
            for app in &apps_repo {
                println!("  {} [{}]", app.name, app.version);
            }
        }
        "info" => {
            if args.len() < 3 {
                println!("❌ Lütfen bir uygulama adı girin.");
                return;
            }
            let query = args[2].to_lowercase();
            if let Some(app) = apps_repo.iter().find(|a| a.name == query) {
                println!("📌 {}:", app.name);
                println!("  Sürüm: {}", app.version);
                println!("  URL: {}", app.url);
                println!("  Dosya: {}", app.file_name);
            } else {
                println!("❌ '{}' bulunamadı.", query);
            }
        }
        "update" => {
            println!("🔄 Paket listeleri güncelleniyor...");
            match repo::update_sources_from_internet() {
                Ok(count) => {
                    println!("✅ sources.list güncellendi ({} paket)", count);
                }
                Err(e) => println!("❌ Hata: {}", e),
            }
        }
        "search" => {
            if args.len() < 3 {
                println!("❌ Lütfen bir kelime girin.");
                return;
            }
            let query = args[2].to_lowercase();
            println!("🔍 '{}' aranıyor...", query);
            let mut found = false;
            for app in &apps_repo {
                if app.name.contains(&query) {
                    println!("  🔹 {} [{}] - {}", app.name, app.version, app.file_name);
                    found = true;
                }
            }
            if !found {
                println!("❌ Eşleşen paket bulunamadı.");
            }
        }
        "install" => {
            if args.len() < 3 {
                println!("❌ Lütfen bir uygulama adı girin.");
                return;
            }
            let target_app = args[2].to_lowercase();
            if let Some(app) = apps_repo.iter().find(|a| a.name == target_app) {
                println!("🎯 Kurulum: {}", app.name);
                perform_installation(app);
            } else {
                println!("❌ '{}' bulunamadı.", target_app);
            }
        }
        "upgrade" => {
            if args.len() < 3 {
                println!("❌ Lütfen bir uygulama adı girin veya 'all' / '--all' kullanın.");
                return;
            }

            if args[2] == "--all" || args[2] == "all" {
                println!("🔄 Tüm paketler güncelleniyor...");
                let mut updated = 0;
                for app in &apps_repo {
                    if system::is_upgrade_needed(&app.name) {
                        if let Some(installed_ver) = system::get_installed_version(&app.name) {
                            println!("  ⬆️  {}: {} → {}", app.name, installed_ver, app.version);
                            perform_installation(app);
                            updated += 1;
                        }
                    }
                }
                if updated == 0 {
                    println!("✅ Tüm paketler zaten güncel.");
                }
                return;
            }

            let target_app = args[2].to_lowercase();
            if let Some(app) = apps_repo.iter().find(|a| a.name == target_app) {
                if system::is_upgrade_needed(&app.name) {
                    if let Some(installed_ver) = system::get_installed_version(&app.name) {
                        println!("🔄 Yükseltme: {} ({} → {})", app.name, installed_ver, app.version);
                    } else {
                        println!("🔄 Yükseltme: {} (ilk kurulum)", app.name);
                    }
                    perform_installation(app);
                } else {
                    if let Some(installed_ver) = system::get_installed_version(&app.name) {
                        println!("✅ {} zaten güncel (sürüm: {}).", app.name, installed_ver);
                    } else {
                        println!("ℹ️  {} yüklü değil. 'install' komutunu kullanın.", app.name);
                    }
                }
            } else {
                println!("❌ '{}' bulunamadı.", target_app);
            }
        }
        "remove" => {
            if args.len() < 3 {
                println!("❌ Lütfen bir uygulama adı girin.");
                return;
            }
            let app_name = args[2].to_lowercase();
            println!("🗑️  {} kaldırılıyor...", app_name);
            match system::remove_package(&app_name) {
                Ok(_) => println!("✅ {} başarıyla kaldırıldı.", app_name),
                Err(e) => println!("❌ Kaldırma hatası: {}", e),
            }
        }
        "clean" => {
            let temp_dir = get_temp_dir();
            if temp_dir.exists() {
                println!("🧹 Geçici dosyalar temizleniyor...");
                match fs::remove_dir_all(&temp_dir) {
                    Ok(_) => println!("✅ Temizlik tamamlandı."),
                    Err(e) => println!("❌ Temizlik hatası: {}", e),
                }
            } else {
                println!("ℹ️  Geçici klasör zaten temiz.");
            }
        }
        "format" => {
            println!("ℹ️  'format' komutu artık kullanılmıyor.");
        }
        "--version" => {
            println!("wid v1.2.3");
        }
        _ => println!("❌ Bilinmeyen komut."),
    }
}

fn perform_installation(app: &repo::AppInfo) {
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
            match system::install_package(file_path_str, &app.name) {
                Ok(_) => println!("🎉 {} başarıyla kuruldu!", app.name),
                Err(e) => println!("❌ Kurulum hatası: {}", e),
            }
            if let Err(e) = fs::remove_file(file_path_str) {
                println!("⚠️  Geçici dosya silinemedi: {}", e);
            }
        }
        Err(e) => println!("❌ İndirme hatası: {}", e),
    }
}