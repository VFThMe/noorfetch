# Noorfetch

![Static Badge](https://img.shields.io/badge/release-3.0.0-green?style=flat) ![Static Badge](https://img.shields.io/badge/license-GNU_LGPL--v3.0-green?style=flat) ![Static Badge](https://img.shields.io/badge/Available_on-Homebrew-yellow) ![Static Badge](https://img.shields.io/badge/Available_on-Cargo-red)

**Noorfetch (arab. نور)** is a minimalistic and fast summary of information about your computer, written in Rust!

![RSFetch screenshot](https://codeberg.org/limforge/noorfetch/raw/branch/main/screenshots/noorfetch_screenshot.png)

## Goals
See GOALS.md file
## Installation
### Dependencies

**Rust language** version `2024`

**Cargo** version `2024`

**Git** version `2.52.0`

### 🍺 Homebrew
```bash
brew tap vfthme/lim
brew install noorfetch
```

### 🦀 Cargo
```bash
cargo install noorfetch
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
noorfetch is distributed under the **GNU Lesser General Public License v3.0 or later**.

This project incorporates the following Rust libraries (crates):
* [sysinfo-0.38.1](https://crates.io/crates/sysinfo) - MIT License
* [whoami-v2.1.1](https://crates.io/crates/whoami) - Apache-2.0, BSL-1.0, or MIT License
* [colored-3.1.1](https://crates.io/crates/colored) - MPL-2.0 License
* [chrono-0.4.43](https://crates.io/crates/chrono) - Apache-2.0 or MIT License
