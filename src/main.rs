use sysinfo::{
    System,
};
use whoami;
use colored::Colorize;
use os_release::OsRelease;
use std::env;

#[path = "ux/detect_icons.rs"]
mod font_detector;
#[path = "ux/ascii.rs"]
mod ascii;
use ascii::Distro;
fn main() {

    // Make a --legacy flag to disable icons
    let args: Vec<String> = env::args().collect();
    let is_legacy = args.iter().any(|arg| arg == "--legacy" || arg == "-l");

    // Examination of OS
    let os = if cfg!(target_os = "windows") {
        format!("Windows {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else if cfg!(target_os = "linux") {
        OsRelease::new().ok().and_then(|r| Some(r.pretty_name)).unwrap_or("Linux".to_string())
    } else if cfg!(target_os = "macos") {
        format!("macOS {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else {
        "Unknown".to_string()
    };
    
    let distro = Distro::from_string(&os);
    let art = distro.ascii_art();

    // Update the information of system
    let mut sys = System::new_all();
    sys.refresh_all();

    let username = whoami::realname().unwrap_or_else(|_| "<unknown>".to_string());
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let kernel = if cfg!(target_os = "windows") { 
        format!("Windows NT {}", System::kernel_version().unwrap_or("Unknown".to_string())) 
    } else if cfg!(target_os = "macos") { 
        format!("Darwin {}", System::kernel_version().unwrap_or("Unknown".to_string())) 
    } else { 
        System::kernel_version().unwrap_or("Unknown".to_string())
    };

    println!("{}", art.cyan().bold());
    println!("{}", "—".repeat(70).dimmed());

    let use_icons = font_detector::nerd_font() && !is_legacy;

    if use_icons {
        println!("{} {} {}", "> |".green().bold(), "   󰆥 :".blue().bold(), os);
        println!("{} {} {}", "> |".green().bold(), "    :".red().bold(), username);
        println!("{} {} {}/{} MB", "> |".green().bold(), "    :".yellow().bold(), used_memory / 1024 / 1024, total_memory / 1024 / 1024); 
        println!("{} {} {}", "> |".green().bold(), "    :".green().bold(), kernel);
    } else {
        println!("{} {} {}", "> |".green().bold(), "   os:".blue().bold(), os);
        println!("{} {} {}", "> |".green().bold(), "   user:".red().bold(), username);
        println!("{} {} {}/{} MB", "> |".green().bold(), "   RAM:".yellow().bold(), used_memory / 1024 / 1024, total_memory / 1024 / 1024); 
        println!("{} {} {}", "> |".green().bold(), "   krnl:".green().bold(), kernel);
    }
    
    println!("{}  {}", "©".cyan(), format!("RSFetch v{} | GNU GPLv3 License | 2026", env!("CARGO_PKG_VERSION")).dimmed());
}