use sysinfo::{
    System,
};
use whoami;
use colored::Colorize;
use os_release::OsRelease;
use std::env;

// Подключаем сторонние файлы из директории UX
#[path = "ux/detect_icons.rs"]
mod font_detector;
#[path = "ux/ascii.rs"]
mod ascii;
use ascii::Distro;
fn main() {

    // Создаем вектор аргументов командной строки и проверяем наличие флага --legacy или -l
    let args: Vec<String> = env::args().collect();
    let is_legacy = args.iter().any(|arg| arg == "--legacy" || arg == "-l");

    // Определяем ОС
    let os = if cfg!(target_os = "windows") {
        format!("Windows {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else if cfg!(target_os = "linux") {
        OsRelease::new().ok().and_then(|r| Some(r.pretty_name)).unwrap_or("Linux".to_string())
    } else if cfg!(target_os = "macos") {
        format!("macOS {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else {
        "Unknown".to_string()
    };
    
    // Создаем переменную distro куда вызываем функцию из файла ascii.rs
    let distro = Distro::from_string(&os); // Измените &os на подходящую строку для дебага
    let art = distro.ascii_art();

    // Обновляем информацию о системе путем sysinfo
    let mut sys = System::new_all();
    sys.refresh_all();

    // Создаем переменные где храним информацию о пользователе, памяти и ядре из sysinfo, whoami и os_release
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

    // Выводим ASCII арт с границей
    println!("{}", art.cyan().bold());
    println!("{}", "—".repeat(70).dimmed());

    // Проверяем поддержку иконок и наличие флага --legacy или -l
    let use_icons = font_detector::nerd_font() && !is_legacy;

    // Выводим информацию с иконками или без них
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
    
    // Выводим версию программы и лицензию
    println!("{}  {}", "©".cyan(), format!("RSFetch v{} | GNU GPLv3 License | 2026", env!("CARGO_PKG_VERSION")).dimmed());
}