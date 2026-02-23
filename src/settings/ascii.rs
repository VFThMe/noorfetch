use strum_macros::{EnumString, EnumIter};
use std::path::PathBuf;
use std::fs;

const G: &str = "\x1b[32m";
const J: &str = "\x1b[36m";
const Y: &str = "\x1b[33m";
const O: &str = "\x1b[38;5;208m";
const R: &str = "\x1b[31m";
const M: &str = "\x1b[35m";
const B: &str = "\x1b[34m";
const RESET: &str = "\x1b[0m";

// Создаем список ОС
#[derive(Debug, Clone, Copy, PartialEq, EnumString, EnumIter)]
pub enum Distro {
	#[strum(to_string = "macos")]
    MacOS,
    #[strum(to_string = "ubuntu")]
    Ubuntu,
    #[strum(to_string = "arch")]
    Arch,
    #[strum(to_string = "fedora")]
    Fedora,
    #[strum(to_string = "debian")]
    Debian,
    #[strum(to_string = "gentoo")]
    Gentoo,
    #[strum(to_string = "cachyos")]
    CachyOS,
    #[strum(to_string = "endeavouros")]
    EndeavourOS,
    #[strum(to_string = "trisquel")]
    Trisquel,
    #[strum(to_string = "nixos")]
    NixOS,
    #[strum(to_string = "bazzite")]
    Bazzite,
    #[strum(to_string = "manjaro")]
    Manjaro,
    #[strum(to_string = "artix")]
    Artix,
    #[strum(to_string = "void")]
    Void,
    #[strum(to_string = "darwin")]
    ALT,
    #[strum(to_string = "guix")]
    Guix,
    #[strum(to_string = "kali")]
    Kali,
    #[strum(to_string = "opensuse")]
    OpenSUSE,
    #[strum(to_string = "lubuntu")]
    Lubuntu,
    #[strum(to_string = "xubuntu")]
    Xubuntu,
    #[strum(to_string = "vanilla")]
    Vanilla,
    #[strum(to_string = "garuda")]
    Garuda,
    #[strum(to_string = "deepin")]
    Deepin,
    #[strum(to_string = "nobara")]
    Nobara,
    #[strum(to_string = "tails")]
    Tails,
    #[strum(to_string = "rhel")]
    RedHat,
    #[strum(to_string = "calculate")]
    Calculate,
    #[strum(to_string = "devuan")]
    Devuan,
    #[strum(to_string = "centos")]
    CentOS,
    #[strum(to_string = "elementary")]
    ElementaryOS,
    #[strum(to_string = "popos")]
    PopOS,
    #[strum(to_string = "freebsd")]
    FreeBSD,
    #[strum(to_string = "netbsd")]
    NetBSD,
    #[strum(to_string = "openbsd")]
    OpenBSD,
    #[strum(to_string = "gnu")]
    GNU,
    #[strum(to_string = "tux")]
    Tux,
    #[strum(to_string = "slackware")]
    Slackware,
    #[strum(to_string = "unknown")]
    Unknown,
}

impl Distro {
	// Возвращаем ASCII арт для каждого дистрибутива
	pub fn ascii_art(&self, path: &PathBuf) -> String {
    	let path = path.join(format!("{:?}.txt", self).to_lowercase());

     	let ascii_art = match fs::read_to_string(&path) {
        	Ok(string) => string,
         	Err(_) => return String::new(),
      	};

     	ascii_art
        	.replace("{G}", G)
         	.replace("{J}", J)
         	.replace("{Y}", Y)
         	.replace("{O}", O)
         	.replace("{R}", R)
         	.replace("{M}", M)
         	.replace("{B}", B)
          	.replace("{RESET}", RESET)
	}
}
