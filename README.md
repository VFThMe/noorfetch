# RSFetch

![Static Badge](https://img.shields.io/badge/release-0.2.0-green?style=flat) ![Static Badge](https://img.shields.io/badge/license-GNU_GPL--v3.0-green?style=flat)

**RSFetch** a minimalistic and fast summary of information about your computer, written in Rust!

![RSFetch screenshot](https://codeberg.org/limforge/RSFetch/raw/branch/main/screenshots/rsfetch_screenshot.png)

## Goals
- [ ] Create configuration file
- [ ] Create package in AUR
- [ ] Create RSFetch installer
- [ ] Create initial ASCII art 

## Installation
### Dependencies

**Rust language** version `2024`

**Cargo** version `2024`

**Git** version `2.52.0`

### Step-by-step instructions

Instal the RSFetch from the Codeberg
```bash
git clone https://codeberg.org/limforge/rsfetch
cd rsfetch
```

Compile the project using Cargo
```bash
cargo install --path .
```

And run RSFetch
```bash
rsfetch
```

## License
RSFetch is distributed under the **GNU General Public License v3.0 or later**.

This project incorporates the following Rust libraries:
* [sysinfo-0.37.2](https://crates.io/crates/sysinfo) — MIT License
* [whoami-2.0.2](https://crates.io/crates/whoami) — Apache-2.0, BSL-1.0, or MIT License
* [colored-3.1.1](https://crates.io/crates/colored) — MPL-2.0 License
* [os-release-0.1.0](https://crates.io/crates/os-release) — MIT License
* [font-kit-0.14.3](https://crates.io/crates/font-kit) - MIT License or Apache 2.0