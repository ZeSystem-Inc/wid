# 🚀 wid – The Fastest Installer for Windows

**wid** is a fast, lightweight, free, and non-blocking package installer for Windows, written in Rust.  
It brings the philosophy of `apt` and `pacman` to Windows, but does not resolve dependencies — instead, it uses **scraping** to find the latest versions.

---

## ⚠️ Warning

**v1.0.0 and v1.2.2 are no longer supported.**  
Please use the latest stable version: **v1.5.0**.

---

## ✨ Features

- Written in Rust – memory-safe, zero-cost, ultra-fast.
- ~2 MB RAM – significantly lighter than winget, choco, or scoop.
- ~99% bandwidth – no BITS or Schannel overhead.
- Scraping – finds real .exe and .msi links from HTML pages.
- No dependency resolution – eliminates fragility.
- Your own repository – manage your own sources.list file.
- GPL v3 – fully free and open-source.

---

## 📦 Available Packages (30+)

    wid list

You can also add your own packages by editing the sources.list file.

---

## 📥 Installation

**The only supported method:** Download `wid_installer.exe` from GitHub Releases and run it.

> cargo install is not supported.

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

**GPL v3**

---

**Compatibility:** Windows Vista, 7, 8, 10, 11 (64-bit).

---

## 🔗 Links

- GitHub: https://github.com/ZeSystem-Inc/wid
- Releases: https://github.com/ZeSystem-Inc/wid/releases/tag/v1.5.0
