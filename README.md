# RSFetch

![Static Badge](https://img.shields.io/badge/release-0.1.0-green?style=for-the-badge)![Static Badge](https://img.shields.io/badge/license-GNU%20GPL--v3.0--only-green?style=for-the-badge)


**RSFetch** a minimalistic and fast summary of information about your computer, written in Rust!

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
cargo build --release
```

Activate and run RSFetch
```bash
chmod +x ./target/release/rsfetch
./target/release/rsfetch
```
or
```bash
cargo run --quiet
```

## License
RSFetch is distributed under the **GNU General Public License v3.0 or later**.

This project incorporates the following Rust libraries:
* [sysinfo-0.37.2](https://crates.io/crates/sysinfo) — MIT License
* [whoami-2.0.2](https://crates.io/crates/whoami) — Apache-2.0, BSL-1.0, or MIT License
* [colored-3.1.1](https://crates.io/crates/colored) — MPL-2.0 License
* [os-release-0.1.0](https://crates.io/crates/os-release) — MIT License