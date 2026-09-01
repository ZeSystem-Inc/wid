# 🚀 wid v1.2.2 – The Fastest Installer for Windows

**wid** is a fast, lightweight, free, and non-blocking package installer for Windows, written in Rust.  
It brings the philosophy of apt and pacman to Windows, but does not resolve dependencies — instead, it uses scraping to find the latest versions.

---

## ✨ Features

- 🦀 Written in Rust – memory-safe, zero-cost, ultra-fast.
- ⚡ ~2 MB RAM – significantly lighter than winget, choco, or scoop.
- 🌐 Direct TCP + rustls – achieves ~99% bandwidth (no BITS or Schannel overhead).
- 🕵️ Scraping – extracts real .exe and .msi links from HTML pages.
- 📦 No Dependency Resolution – leaves dependency handling to the app's own installer.
- 📝 Your Own Repository – you manage your own sources.list file.
- 🔓 GPL v3 – fully free and open-source.

---

## 📦 Available Packages

The package list is continuously updated. To see the current list:

    wid list

You can also add your own packages by editing the sources.list file.

---

## 📥 Installation

**The only supported installation method:** Download `wid_installer.exe` from the [GitHub Releases](https://github.com/ZeSystem-Inc/wid/releases/tag/v1.2.2) page and run it.

> ⚠️ **Note:** Installation via `cargo install` is **not supported**. You must use the provided `wid_installer.exe` binary.

---

## 🛠️ Usage

    wid list                  # List all packages
    wid info <app>            # Show package details
    wid update                # Update package list
    wid search <keyword>      # Search repository
    wid install <app>         # Download and install
    wid upgrade <app>         # Upgrade a package
    wid upgrade --all         # Upgrade all packages
    wid remove <app>          # Remove an app (UAC)
    wid clean                 # Clean temporary files
    wid --version             # Show wid version

**Examples:**

    wid install vscode
    wid upgrade git
    wid search steam
    wid remove 7zip
    wid upgrade --all

---

## 📄 License

**GPL v3** – Use it freely, fork it, improve it.

---

**Compatibility:** Windows Vista, 7, 8, 10, 11 (64-bit).

---

## 🔗 Links

- GitHub: https://github.com/ZeSystem-Inc/wid
- Releases: https://github.com/ZeSystem-Inc/wid/releases/tag/v1.2.2
