# 🚀 wid – Windows'un En Hızlı Yükleyicisi

**wid**, Windows için Rust ile yazılmış, **hızlı, hafif, özgür ve takılmayan** bir paket yükleyicisidir.  
`apt` ve `pacman` felsefesini Windows'a taşır, ancak bağımlılık çözmez, **scraping** ile en güncel sürümleri bulur.

---

## ✨ Özellikler

- 🦀 **Rust ile yazıldı** – bellek güvenli, sıfır maliyetli, ultra hızlı.
- ⚡ **~2 MB RAM** – `winget`, `choco` veya `scoop`'tan kat kat hafif.
- 🌐 **Doğrudan TCP + rustls** – **%99 bant hızı** (BITS veya Schannel yok).
- 🕵️ **Scraping (Web Kazıma)** – HTML sayfalarından gerçek `.exe` ve `.msi` linklerini cımbızlar.
- 📦 **Bağımlılık Çözmez** – Bu işi uygulamanın kendi installer'ına bırakarak en büyük kırılganlık kaynağını ortadan kaldırır.
- 📝 **Sizin Havuzunuz** – `sources.list`'i kendiniz yönetirsiniz, merkezi bir otoriteye bağlı değilsiniz.
- 🔓 **GPL v3** – Tamamen özgür, isteyen fork'layıp geliştirebilir.

---

## 📦 Mevcut Paketler

Paket listesi sürekli güncellenmektedir. Güncel listeyi görmek için:

```bash
wid list
```

Kendi paketlerinizi eklemek için `sources.list` dosyasını düzenleyebilirsiniz.

---

## 📥 Kurulum

### 1️⃣ Hazır İndirici (Önerilen)
[En son sürümü GitHub Releases'den indirin](https://github.com/Z.eSystem-Inc/wid/releases) ve `wid_installer.exe`'yi çalıştırın.

### 2️⃣ Cargo ile (Geliştiriciler için)
```bash
cargo install wid
```

---

## 🛠️ Kullanım

```bash
wid list                  # Havuzdaki tüm paketleri listele
wid info <uygulama>       # Paket hakkında detaylı bilgi
wid update                # Paket listesini güncelle
wid search <kelime>       # Havuzda arama yap
wid install <uygulama>    # Uygulamayı indir ve kur
wid upgrade <uygulama>    # Sürüm kontrolü yaparak yükselt
wid upgrade --all         # Tüm paketleri güncelle
wid remove <uygulama>     # Kurulu uygulamayı kaldır (UAC)
wid clean                 # Geçici dosyaları temizle
wid --version             # Wid sürümünü göster
```

**Örnekler:**

```bash
wid install vscode
wid upgrade git
wid search steam
wid remove 7zip
wid upgrade --all
```

---

## 📄 Lisans

**GPL v3** – Özgürce kullan, fork'la, geliştir.

---

## 💬 Katkı

Her türlü katkıya açığız! Hata raporları, özellik önerileri ve pull request'ler memnuniyetle karşılanır.

---

**Compatibility:** Windows Vista, 7, 8, 10, 11 (64-bit).