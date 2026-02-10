# Noorfetch

![Static Badge](https://img.shields.io/badge/release-2.7.10-green?style=flat) ![Static Badge](https://img.shields.io/badge/license-GNU_GPL--v3.0-green?style=flat) ![Static Badge](https://img.shields.io/badge/Available_on-AUR-blue)

**Noorfetch** (arab. نور) is a minimalistic and fast summary of information about your computer, written in Rust!

![RSFetch screenshot](https://codeberg.org/limforge/noorfetch/raw/branch/main/screenshots/noorfetch_screenshot.png)

## Goals
See GOALS.md file
## Installation
### Dependencies

**Rust language** version `2024`

**Cargo** version `2024`

**Git** version `2.52.0`

### AUR
Get **Noorfetch** from **AUR** (Arch User Repository) right now!
```bash
git clone https://aur.archlinux.org/noorfetch.git
cd noorfetch
makepkg -si
```

**Or**
```bash
yay -Syu  noorfetch
```
```bash
paru -Syu noorfetch
```

### Run the installer
**Run** ``installer_unix.sh``.

### Build from source

Instal the Noorfetch from the Codeberg
```bash
git clone https://codeberg.org/limforge/noorfetch
cd noorfetch
```

Compile the project using Cargo
```bash
cargo install --path .
```

And run Noorfetch
```bash
noorfetch
```

## License
noorfetch is distributed under the **GNU General Public License v3.0 or later**.

This project incorporates the following Rust libraries (crates):
* [sysinfo-0.37.2](https://crates.io/crates/sysinfo) - MIT License
* [whoami-v2.1.0](https://crates.io/crates/whoami) - Apache-2.0, BSL-1.0, or MIT License
* [colored-3.1.1](https://crates.io/crates/colored) - MPL-2.0 License
* [os-release-0.1.0](https://crates.io/crates/os-release) - MIT License
* [chrono-0.4.43](https://crates.io/crates/chrono) - Apache-2.0 or MIT License
