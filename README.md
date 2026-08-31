# 🚀 wid – Windows için Hızlı ve Özgür Paket Yükleyici

**wid**, Windows için Rust ile yazılmış, **hızlı, hafif, özgür ve takılmayan** bir paket yükleyicisidir.  
`apt` ve `pacman` felsefesini Windows'a taşır, ancak bağımlılık çözmez, **scraping** ile en güncel sürümleri bulur.

## ✨ Özellikler

- 🦀 **Rust ile yazıldı** – bellek güvenli, sıfır maliyetli, ultra hızlı.
- ⚡ **~2 MB RAM** ile çalışır, `winget` ve `choco`'dan kat kat hafif.
- 🌐 **Doğrudan TCP + rustls** ile **%99 bant hızı** (BITS veya Schannel yok).
- 🕵️ **Scraping (Web Kazıma)** – HTML sayfalarından gerçek `.exe` ve `.msi` linklerini cımbızlar.
- 📦 **Bağımlılık Çözmez** – Bu işi uygulamanın kendi installer'ına bırakarak en büyük kırılganlık kaynağını ortadan kaldırır.
- 📝 **Sizin Havuzunuz** – `sources.list`'i kendiniz yönetirsiniz, merkezi bir otoriteye bağlı değilsiniz.
- 🔓 **GPL v3** – Tamamen özgür, isteyen fork'layıp geliştirebilir.

## 📥 Kurulum

**Tek kurulum yöntemi:** [GitHub Releases](https://github.com/Z.eSystem-Inc/wid/releases) sayfasından `wid_installer.exe` dosyasını indirip çalıştırın.

> ⚠️ **Not:** `cargo install` ile kurulum desteklenmemektedir. Sadece hazır `wid_installer.exe` kullanılmalıdır.

## 🛠️ Kullanım

```bash
wid update               # Paket listesini güncelle
wid search <kelime>      # Havuzda arama yap
wid install <uygulama>   # Uygulamayı indir ve kur
wid upgrade <uygulama>   # Sürüm kontrolü yaparak yükselt
wid clean                # Geçici dosyaları temizle
