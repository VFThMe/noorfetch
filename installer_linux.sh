#!/bin/bash

# Colour to input
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Colour

echo -e "${BLUE}=== Noorfetch Installer ===${NC}"

# Root check (we ask in advance so as not to be interrupted)
if [ "$EUID" -ne 0 ]; then
  echo -e "${BLUE}[1/4] Please enter your password to obtain root privileges...${NC}"
  # Просто проверяем возможность выполнения sudo
  sudo -v || { echo -e "${RED}Ошибка: необходимы права sudo${NC}"; exit 1; }
fi

# Determining the distribution and installing Rust/Cargo
echo -e "${BLUE}[2/4] Checking the environment and dependencies...${NC}"

if ! command -v cargo &> /dev/null; then
    echo -e "${GREEN}Cargo not found. Attempting to install dependencies...${NC}"
    
    # Determine the distribution
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        case $ID in
            ubuntu|debian)
                sudo apt-get update
                sudo apt-get install -y cargo rustc build-essential
                ;;
            arch)
                sudo pacman -Syu --noconfirm rust
                ;;
            fedora)
                sudo dnf install -y rust cargo
                ;;
            gentoo)
                echo -e "${GREEN}Gentoo detected. Emerging dev-lang/rust...${NC}"
                sudo emerge --ask=n dev-lang/rust
                ;;
            *)
                echo -e "${RED}The $ID distribution is not supported by this script.${NC}"
                exit 1
                ;;
        esac
    else
        echo -e "${RED}The operating system could not be identified.${NC}"
        exit 1
    fi
else
    echo -e "${GREEN}Rust and Cargo are already installed.${NC}"
fi

# Project compilation
echo -e "${BLUE}[3/4] Compiling rsfetch (release)...${NC}"
if [ -f "Cargo.toml" ]; then
    cargo build --release
    if [ $? -ne 0 ]; then
        echo -e "${RED}Error during project compilation.${NC}"
        exit 1
    fi
else
    echo -e "${RED}Error: Cargo.toml file not found in the current directory!${NC}"
    exit 1
fi

# Installing the binary file
echo -e "${BLUE}[4/4] Copying binary file to/usr/local/bin...${NC}"
sudo cp target/release/noorfetch /usr/local/bin/noorfetch
sudo chmod +x /usr/local/bin/noorfetch

echo -e "${GREEN}=== Installation completed successfully! ===${NC}"
echo -e "Now you can run the programme by entering: ${BLUE}noorfetch${NC}"
