#!/usr/bin/env bash
set -euo pipefail

# cfg
APP_NAME="noorfetch"
PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"

log() { echo -e "\033[0;34m→\033[0m $*"; }
die() { echo -e "\033[0;31m✗\033[0m $*" >&2; exit 1; }

# part of the installation (root)
if [[ "${1:-}" == "--install-phase" ]]; then
  [[ $EUID -eq 0 ]] || die "Installation requires root privileges!"
  
  SOURCE_BIN="$PROJECT_DIR/target/release/$APP_NAME"
  
  if [[ ! -f "$SOURCE_BIN" ]]; then
    die "Binary file not found in $SOURCE_BIN. Did the build fail?"
  fi

  log "Copying binary file to /usr/local/bin..."
  install -Dm755 "$SOURCE_BIN" "/usr/local/bin/$APP_NAME"
  
  log "Done! Now you can run '$APP_NAME'"
  exit 0
fi

# dependency installation function
install_dependencies() {
    log "Identifying operating systems..."
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        case $ID in
            ubuntu|debian|popos|xubuntu|lubuntu|devuan)
                sudo apt-get update && sudo apt-get -y install cargo rustc build-essential ;;
            arch|manjaro|artix|cachyos|endeavouros|garuda)
                sudo pacman -Syu --noconfirm rust ;;
            fedora|nobara)
                sudo dnf install -y rust cargo ;;
            alt|altlinux)
                sudo apt-get install -y rust rust-cargo ;;
            void|voidlinux)
                sudo xbps-install -Sy cargo rust ;;
            freebsd)
                sudo pkg install -y cargo ;;
            *)
                log "The OS is not supported by the installer. Install Rust manually via your package manager."
                exit 1 ;;
        esac
    else
        die "Unknown OS!"
    fi
}

# part of the building
log "Start installing $APP_NAME from the local folder"

# check
if ! command -v cargo >/dev/null; then
    echo -ne "\033[0;33m? \033[0mDo you want to install Cargo and Rust as dependencies (required for noorfetch!)? [y/n]: "
    read -r answer
    if [[ "$answer" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        install_dependencies
    else
        die "Interrupted!"
    fi
fi

# building
log "Project build (release)..."
cd "$PROJECT_DIR"
if [[ ! -f "Cargo.toml" ]]; then
    die "Cargo.toml not found in $PROJECT_DIR. Run the script from the project folder!"
fi

cargo build --release || die "Error in building!"

# root request
log "Requesting root privileges to copy a file to the system..."
if command -v sudo >/dev/null; then
  exec sudo bash "$0" --install-phase "$PROJECT_DIR"
elif command -v doas >/dev/null; then
  exec doas bash "$0" --install-phase "$PROJECT_DIR"
else
  die "sudo or doas not found to complete the installation."
fi
