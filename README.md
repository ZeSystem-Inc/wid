# 🚀 wid – Windows'un En Hızlı Yükleyicisi

**wid**, Windows için Rust ile yazılmış, hızlı, hafif, özgür ve takılmayan bir paket yükleyicisidir.  
`apt` ve `pacman` felsefesini Windows'a taşır, ancak bağımlılık çözmez, **scraping** ile en güncel sürümleri bulur.

---

## ⚠️ Uyarı

**v1.0.0 ve v1.2.2 artık desteklenmemektedir.**  
Lütfen en son kararlı sürüm olan **v1.5.0**'ı kullanın.

---

## ✨ Özellikler

- Rust ile yazıldı – bellek güvenli, ultra hızlı.
- ~2 MB RAM – winget, choco veya scoop'tan kat kat hafif.
- %99 bant hızı – BITS veya Schannel yok.
- Scraping – en güncel .exe ve .msi linklerini bulur.
- Bağımlılık çözmez – kırılganlık yok.
- Kendi havuzun – sources.list ile merkezi otoriteye bağlı değilsin.
- GPL v3 – tamamen özgür.

---

## 📦 Mevcut Paketler (30+)

    wid list

Kendi paketlerini eklemek için sources.list dosyasını düzenleyebilirsin.

---

## 📥 Kurulum

**Tek kurulum yöntemi:** GitHub Releases'den `wid_installer.exe` indirip çalıştır.

> cargo install desteklenmez.

---

## 🛠️ Kullanım

    wid list                  # Tüm paketleri listele
    wid info <uygulama>       # Paket detayları
    wid update                # Paket listesini güncelle
    wid search <kelime>       # Havuzda arama
    wid install <uygulama>    # Uygulama kur
    wid upgrade <uygulama>    # Uygulamayı yükselt
    wid upgrade --all         # Tüm paketleri güncelle
    wid remove <uygulama>     # Uygulamayı kaldır (UAC)
    wid clean                 # Geçici dosyaları temizle
    wid --version             # Wid sürümünü göster

**Örnekler:**

    wid install vscode
    wid upgrade git
    wid search steam
    wid remove 7zip
    wid upgrade --all

---

## 📄 Lisans

**GPL v3**

---

**Uyumluluk:** Windows Vista, 7, 8, 10, 11 (64-bit).

---

## 🔗 Bağlantılar

- GitHub: https://github.com/ZeSystem-Inc/wid
- Releases: https://github.com/ZeSystem-Inc/wid/releases/tag/v1.5.0