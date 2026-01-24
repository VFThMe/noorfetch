use sysinfo::{
    System,
};
use whoami;
use colored::Colorize;
use os_release::OsRelease;
use std::{ env };

// Подключаем сторонние файлы из директории UX
#[path = "Settings/detect_icons.rs"]
mod font_detector;
#[path = "Settings/ascii.rs"]
mod ascii;
use ascii::Distro;
// #[path = "Settings/config.rs"]
// mod config;
fn main() {
    // let cfg = config::load_config();

    // let path = config::get_path();
    // println!("Файл конфига находится тут: {:?}", path);

    // Создаем вектор аргументов командной строки и проверяем наличие флага --legacy или -l
    let args: Vec<String> = env::args().collect();
    let is_legacy = args.iter().any(|arg| arg == "--legacy" || arg == "-l");

    // Identifying OS
    let os = if cfg!(target_os = "windows") {
        format!("Windows {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else if cfg!(target_os = "linux") {
        OsRelease::new().ok().and_then(|r| Some(r.pretty_name)).unwrap_or("Linux".to_string())
    } else if cfg!(target_os = "macos") {
        format!("macOS {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else {
        "Unknown".to_string()
    };
    
    // Create a variable distro where we call the function from the file ascii.rs
    let distro = Distro::from_string(&os); // Измените &os на подходящую строку для дебага
    let art = distro.ascii_art();

    // Updating system information using sysinfo
    let mut sys = System::new_all();
    sys.refresh_all();

    // Create variables where we store information about the user, memory, and kernel from sysinfo, whoami, and os_release.
    let username = whoami::realname().unwrap_or_else(|_| "<unknown>".to_string());
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let cpu = sys.cpus().len();
    let cpu_brand = sys.cpus().get(0).map(|c| c.brand()).unwrap_or("Unknown CPU");
    let hostname = System::host_name().unwrap_or("Unknown".to_string());
    let kernel = if cfg!(target_os = "windows") { 
        format!("Windows NT {}", System::kernel_version().unwrap_or("Unknown".to_string())) 
    } else if cfg!(target_os = "macos") { 
        format!("Darwin {}", System::kernel_version().unwrap_or("Unknown".to_string())) 
    } else { 
        System::kernel_version().unwrap_or("Unknown".to_string())
    };

    // Display ASCII art with a border
    println!("{}", art.cyan().bold());
    println!("{}", "—".repeat(70).dimmed());

    // Check icon support and the presence of the --legacy or -l flag
    let use_icons = font_detector::nerd_font() && !is_legacy;

    // Display information with or without icons
    if use_icons {
       println!("{} {} {}", "> |".green().bold(), "   󰆥 :".blue().bold(), os);
        println!("{} {} {}", "> |".green().bold(), "    :".red().bold(), username);
        println!("{} {} {}", "> |".green().bold(), "   󰆋 :".white().bold(), hostname);
        println!("{} {} {}/{} MB", "> |".green().bold(), "    :".yellow().bold(), used_memory / 1024 / 1024, total_memory / 1024 / 1024);
        println!("{} {} {}/{} MB", "> |".green().bold(), "    :".magenta().bold(), used_swap / 1024 / 1024, total_swap / 1024 / 1024);
        println!("{} {} {} ({})", "> |".green().bold(), "    :".red().bold(), cpu_brand, cpu);
        println!("{} {} {}", "> |".green().bold(), "    :".green().bold(), kernel);
    } else {
        println!("{} {} {}", "> |".green().bold(), "   os:".blue().bold(), os);
        println!("{} {} {}", "> |".green().bold(), "   user:".red().bold(), username);
        println!("{} {} {}", "> |".green().bold(), "   host:".white().bold(), hostname);
        println!("{} {} {}/{} MB", "> |".green().bold(), "   ram:".yellow().bold(), used_memory / 1024 / 1024, total_memory / 1024 / 1024);
        println!("{} {} {}/{} MB", "> |".green().bold(), "   swap:".magenta().bold(), used_swap / 1024 / 1024, total_swap / 1024 / 1024);
        println!("{} {} {} ({})", "> |".green().bold(), "   cpu: ".red().bold(), cpu_brand, cpu); 
        println!("{} {} {}", "> |".green().bold(), "   krnl:".green().bold(), kernel);
    }
    
    // Displaying the program version and license
    println!("{}  {}", "©".cyan(), format!("RSFetch v{} | GNU GPLv3 License | 2026", env!("CARGO_PKG_VERSION")).dimmed());
}