use std::process::Command;
use std::path::Path;
use winreg::RegKey;
use winreg::enums::*;
use semver::Version;

use crate::repo;

fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

fn verify_installation(app_name: &str) -> bool {
    if get_installed_version(app_name).is_some() {
        return true;
    }

    let program_files = std::env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".to_string());
    let program_files_x86 = std::env::var("ProgramFiles(x86)").unwrap_or_else(|_| "C:\\Program Files (x86)".to_string());
    let local_appdata = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| "C:\\Users\\Default\\AppData\\Local".to_string());
    let userprofile = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:\\Users\\Default".to_string());

    let possible_paths = vec![
        format!("{}\\{}", program_files, app_name),
        format!("{}\\{}", program_files_x86, app_name),
        format!("{}\\{}", local_appdata, app_name),
        format!("{}\\Desktop\\{}", userprofile, app_name),
        format!("{}\\AppData\\Local\\{}", userprofile, app_name),
    ];

    for path in possible_paths {
        if std::path::Path::new(&path).exists() {
            return true;
        }
    }

    false
}

fn try_silent_install(file_path: &str, flag: &str, app_name: &str) -> bool {
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            &format!(
                r#"$ErrorActionPreference = 'SilentlyContinue'; $p = Start-Process -FilePath '{}' -ArgumentList '{}' -Verb RunAs -Wait -WindowStyle Hidden -PassThru; exit $p.ExitCode"#,
                file_path, flag
            )
        ])
        .output();

    if output.is_err() || !output.as_ref().map(|o| o.status.success()).unwrap_or(false) {
        return false;
    }

    std::thread::sleep(std::time::Duration::from_secs(3));

    verify_installation(app_name)
}

fn open_gui(file_path: &str, app_name: &str) -> Result<(), String> {
    println!("🔄 Sessiz kurulum mümkün değil, GUI açılıyor...");

    let output = Command::new("powershell")
        .args(&[
            "-Command",
            &format!(
                r#"$ErrorActionPreference = 'SilentlyContinue'; $p = Start-Process -FilePath '{}' -Verb RunAs -Wait -PassThru; exit $p.ExitCode"#,
                file_path
            )
        ])
        .output()
        .map_err(|e| format!("GUI başlatılamadı: {}", e))?;

    if output.status.success() {
        if verify_installation(app_name) {
            println!("✅ GUI kurulum tamamlandı.");
            Ok(())
        } else {
            println!("❌ GUI kurulum tamamlanmadı veya doğrulanamadı.");
            Err("GUI kurulum başarısız.".to_string())
        }
    } else {
        Err("Kurulum iptal edildi veya başarısız.".to_string())
    }
}

pub fn install_package(file_path: &str, app_name: &str) -> Result<(), String> {
    let path = Path::new(file_path);
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("exe");

    println!("📦 Kurulum Başlıyor | Format: .{}", extension);

    match extension {
        "exe" => {
            if try_silent_install(file_path, "/S", app_name) {
                println!("✅ Sessiz kurulum başarılı! (flag: /S)");
                return Ok(());
            }

            if try_silent_install(file_path, "/quiet", app_name) {
                println!("✅ Sessiz kurulum başarılı! (flag: /quiet)");
                return Ok(());
            }

            open_gui(file_path, app_name)
        }
        "msi" => {
            println!("🔑 RunAs ile MSI kurulumu...");
            let output = Command::new("powershell")
                .args(&[
                    "-Command",
                    &format!(
                        r#"$ErrorActionPreference = 'SilentlyContinue'; $p = Start-Process -FilePath 'msiexec' -ArgumentList '/i "{}" /quiet /norestart' -Verb RunAs -Wait -WindowStyle Hidden -PassThru; exit $p.ExitCode"#,
                        file_path
                    )
                ])
                .output()
                .map_err(|e| format!("MSI başlatılamadı: {}", e))?;

            if output.status.success() && verify_installation(app_name) {
                println!("✅ MSI kurulum başarılı.");
                Ok(())
            } else {
                println!("⚠️  MSI kurulum başarısız veya doğrulanamadı.");
                Err("MSI kurulum başarısız.".to_string())
            }
        }
        _ => Err(format!("'.{}' uzantısı desteklenmiyor.", extension)),
    }
}

pub fn remove_package(app_name: &str) -> Result<(), String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let uninstall_paths = vec![
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
    ];

    let search_term = normalize(app_name);

    for path in uninstall_paths {
        if let Ok(subkey) = hklm.open_subkey(path) {
            for key_name in subkey.enum_keys().flatten() {
                if let Ok(key) = subkey.open_subkey(&key_name) {
                    if let Ok(display_name) = key.get_value::<String, _>("DisplayName") {
                        let normalized_display = normalize(&display_name);
                        if normalized_display.contains(&search_term) {
                            if let Ok(uninstall_cmd) = key.get_value::<String, _>("UninstallString") {
                                if !uninstall_cmd.is_empty() {
                                    println!("🔑 Kaldırma komutu bulundu: {}", uninstall_cmd);

                                    // MSI ise
                                    if uninstall_cmd.to_lowercase().contains("msiexec") {
                                        let guid = uninstall_cmd
                                            .split('{')
                                            .nth(1)
                                            .and_then(|s| s.split('}').next())
                                            .unwrap_or("")
                                            .to_string();

                                        if !guid.is_empty() {
                                            let status = Command::new("msiexec")
                                                .args(&[
                                                    "/x",
                                                    &format!("{{{}}}", guid),
                                                    "/quiet",
                                                    "/norestart"
                                                ])
                                                .status()
                                                .map_err(|e| format!("MSI kaldırma başlatılamadı: {}", e))?;

                                            if status.success() && !verify_installation(app_name) {
                                                println!("✅ MSI sessiz kaldırma başarılı.");
                                                return Ok(());
                                            } else if status.success() && verify_installation(app_name) {
                                                println!("⚠️  MSI kaldırma başarısız, uygulama hala yüklü.");
                                            } else {
                                                println!("🔄 MSI sessiz kaldırma başarısız, GUI açılıyor...");
                                                let status = Command::new("msiexec")
                                                    .args(&[
                                                        "/x",
                                                        &format!("{{{}}}", guid),
                                                    ])
                                                    .status()
                                                    .map_err(|e| format!("MSI GUI kaldırma başlatılamadı: {}", e))?;

                                                if status.success() && !verify_installation(app_name) {
                                                    println!("✅ MSI GUI kaldırma başarılı.");
                                                    return Ok(());
                                                }
                                            }
                                        }
                                    } else {
                                        let cmd = uninstall_cmd
                                            .replace("/I", "")
                                            .replace("/x", "")
                                            .replace("\"", "")
                                            .trim()
                                            .to_string();

                                        if cmd.is_empty() {
                                            continue;
                                        }

                                        let status = Command::new("powershell")
                                            .args(&[
                                                "-Command",
                                                &format!(
                                                    r#"$ErrorActionPreference = 'SilentlyContinue'; $p = Start-Process -FilePath '{}' -ArgumentList '/S' -Verb RunAs -Wait -WindowStyle Hidden -PassThru; exit $p.ExitCode"#,
                                                    cmd
                                                )
                                            ])
                                            .status()
                                            .map_err(|e| format!("Kaldırma başlatılamadı: {}", e))?;

                                        if status.success() && !verify_installation(app_name) {
                                            println!("✅ Sessiz kaldırma başarılı.");
                                            return Ok(());
                                        } else {
                                            println!("🔄 Sessiz kaldırma başarısız, GUI açılıyor...");
                                            let status = Command::new("powershell")
                                                .args(&[
                                                    "-Command",
                                                    &format!(
                                                        r#"Start-Process -FilePath '{}' -Verb RunAs -Wait"#,
                                                        cmd
                                                    )
                                                ])
                                                .status()
                                                .map_err(|e| format!("GUI kaldırma başlatılamadı: {}", e))?;

                                            if status.success() && !verify_installation(app_name) {
                                                println!("✅ GUI kaldırma başarılı.");
                                                return Ok(());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Err(format!(
        "'{}' için kaldırma komutu bulunamadı.\n\
         Lütfen Denetim Masası > Program Kaldır bölümünden manuel olarak kaldırın.",
        app_name
    ))
}

/// Kayıt defterinden yüklü sürümü okur (normalize ile)
pub fn get_installed_version(app_name: &str) -> Option<String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let uninstall_paths = vec![
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
    ];

    let java_paths = vec![
        r"SOFTWARE\JavaSoft\Java Runtime Environment",
        r"SOFTWARE\JavaSoft\Java Development Kit",
        r"SOFTWARE\WOW6432Node\JavaSoft\Java Runtime Environment",
        r"SOFTWARE\WOW6432Node\JavaSoft\Java Development Kit",
    ];

    for path in java_paths {
        if let Ok(key) = hklm.open_subkey(path) {
            if let Ok(version) = key.get_value::<String, _>("CurrentVersion") {
                return Some(version);
            }
        }
    }

    let search_term = normalize(app_name);

    for path in uninstall_paths {
        if let Ok(subkey) = hklm.open_subkey(path) {
            for key_name in subkey.enum_keys().flatten() {
                if let Ok(key) = subkey.open_subkey(&key_name) {
                    if let Ok(display_name) = key.get_value::<String, _>("DisplayName") {
                        let normalized_display = normalize(&display_name);
                        if normalized_display.contains(&search_term) {
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

/// Sürüm yükseltme kontrolü
pub fn is_upgrade_needed(app_name: &str) -> bool {
    let installed = get_installed_version(app_name);
    let available = get_available_version_from_repo(app_name);

    if let (Some(installed), Some(available)) = (installed, available) {
        if let (Ok(installed_ver), Ok(available_ver)) = (
            Version::parse(&installed),
            Version::parse(&available),
        ) {
            return available_ver > installed_ver;
        }
    }
    false
}

/// sources.list'ten sürüm bilgisini al
fn get_available_version_from_repo(app_name: &str) -> Option<String> {
    let apps = repo::load_sources();
    for app in apps {
        if app.name == app_name {
            return Some(app.version);
        }
    }
    None
}