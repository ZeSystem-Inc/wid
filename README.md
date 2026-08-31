# 🚀 wid – Fast & Free Package Installer for Windows

**wid** is a **fast, lightweight, free, and non-blocking** package installer for Windows, written in Rust.  
It brings the philosophy of `apt` and `pacman` to Windows, but does not resolve dependencies — instead, it uses **scraping** to find the latest versions.

## ✨ Features

- 🦀 **Written in Rust** – memory-safe, zero-cost abstractions, ultra-fast.
- ⚡ **~2 MB RAM** usage – significantly lighter than `winget` and `choco`.
- 🌐 **Direct TCP + rustls** – achieves **~99% bandwidth** (no BITS or Schannel overhead).
- 🕵️ **Scraping (Web Scraping)** – extracts real `.exe` and `.msi` links from HTML pages.
- 📦 **No Dependency Resolution** – leaves dependency handling to the application's own installer, eliminating a major source of fragility.
- 📝 **Your Own Repository** – you manage your own `sources.list` file, independent of any central authority.
- 🔓 **GPL v3** – fully free and open-source; fork and modify as you wish.

## 📥 Installation

**The only supported installation method:** Download `wid_installer.exe` from the [GitHub Releases](https://github.com/Z.eSystem-Inc/wid/releases) page and run it.

> ⚠️ **Note:** Installation via `cargo install` is **not supported**. You must use the provided `wid_installer.exe` binary.

## 🛠️ Usage

```bash
wid update               # Update the package list
wid search <keyword>     # Search the repository
wid install <app>        # Download and install an application
wid upgrade <app>        # Check for updates and upgrade if available
wid clean                # Clean temporary files
