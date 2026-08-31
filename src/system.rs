use std::process::Command;
use std::path::Path;
use winreg::RegKey;
use winreg::enums::*;

pub fn install_package(file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("exe");

    println!("📦 Kurulum Başlıyor | Format: .{}", extension);

    match extension {
        "exe" => {
            for flag in &["/S", "/silent", "/VERYSILENT", "/quiet"] {
                println!("🔑 RunAs ile sessiz kurulum deneniyor: {} {}", file_path, flag);
                let status = Command::new("powershell")
                    .args(&[
                        "-Command",
                        &format!(
                            "Start-Process -FilePath '{}' -ArgumentList '{}' -Verb RunAs -Wait",
                            file_path, flag
                        )
                    ])
                    .status();

                match status {
                    Ok(s) if s.success() => {
                        println!("✅ Sessiz kurulum başarılı ({})", flag);
                        return Ok(());
                    }
                    Ok(s) => {
                        println!("⚠️  Deneme başarısız (kod: {}), sonraki flag.", s.code().unwrap_or(-1));
                    }
                    Err(e) => {
                        println!("⚠️  PowerShell hatası: {}", e);
                    }
                }
            }

            println!("⚠️  Sessiz kurulum olmadı, normal GUI açılıyor...");
            let status = Command::new("powershell")
                .args(&[
                    "-Command",
                    &format!("Start-Process -FilePath '{}' -Verb RunAs -Wait", file_path)
                ])
                .status()
                .map_err(|e| format!("Kurulum başlatılamadı: {}", e))?;

            if status.success() {
                Ok(())
            } else {
                Err("Kurulum başarısız veya iptal edildi.".to_string())
            }
        }
        "msi" => {
            println!("🔑 RunAs ile MSI kurulumu...");
            let status = Command::new("powershell")
                .args(&[
                    "-Command",
                    &format!(
                        "Start-Process -FilePath 'msiexec' -ArgumentList '/i \"{}\" /quiet /norestart' -Verb RunAs -Wait",
                        file_path
                    )
                ])
                .status()
                .map_err(|e| format!("MSI başlatılamadı: {}", e))?;

            if status.success() {
                Ok(())
            } else {
                Err("MSI kurulum başarısız.".to_string())
            }
        }
        _ => Err(format!("'.{}' uzantısı desteklenmiyor (sadece .exe ve .msi).", extension)),
    }
}

pub fn get_installed_version(app_name: &str) -> Option<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let uninstall_paths = vec![
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
    ];

    for path in uninstall_paths {
        if let Ok(subkey) = hklm.open_subkey(path) {
            for key_name in subkey.enum_keys().flatten() {
                if let Ok(key) = subkey.open_subkey(&key_name) {
                    if let Ok(display_name) = key.get_value::<String, _>("DisplayName") {
                        if display_name.to_lowercase().contains(&app_name.to_lowercase()) {
                            if let Ok(version) = key.get_value::<String, _>("DisplayVersion") {
                                return Some(version);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}