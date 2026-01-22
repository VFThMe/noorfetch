use sysinfo::{
    System,
};
use whoami;
use colored::Colorize;
use os_release::OsRelease;

#[path = "ux/detect_icons.rs"]
mod font_detector;

fn main() {

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

    if font_detector::nerd_font() {
        println!("{} {} {}", "> |".green().bold(), "   󰆥 :".blue().bold(), os);
        println!("{} {} {}", "> |".green().bold(), "    :".red().bold(), username);
        println!("{} {} {}/{} MB", "> |".green().bold(), "    :".yellow().bold(), used_memory / 1024 / 1024, total_memory / 1024 / 1024); 
        println!("{} {} {}", "> |".green().bold(), "    :".green().bold(), kernel);
    } else {
        println!("{} {} {}", "> |".green().bold(), "   os:".blue().bold(), os);
        println!("{} {} {}", "> |".green().bold(), "   user:".red().bold(), username);
        println!("{} {} {}/{} MB", "> |".green().bold(), "   RAM:".yellow().bold(), used_memory / 1024 / 1024, total_memory / 1024 / 1024); 
        println!("{} {} {}", "> |".green().bold(), "   krnl:".green().bold(), kernel);
        println!("{}", "Please install a Jetbrains Mono Nerd to see icons!".dimmed());
    }
    
    println!("{}  {}", "©".cyan(), format!("RSFetch v{} | GNU GPLv3 License | 2026", env!("CARGO_PKG_VERSION")).dimmed());
}