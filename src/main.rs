use sysinfo::{
    System,
};
use whoami;
use colored::{ Colorize, Color };
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

    // Identifying OS. Определяем ОС
    let os = if cfg!(target_os = "windows") {
        format!("Windows {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else if cfg!(target_os = "linux") {
        OsRelease::new().ok().and_then(|r| Some(r.pretty_name)).unwrap_or("Linux".to_string())
    } else if cfg!(target_os = "macos") {
        format!("macOS {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else if cfg!(target_os = "freebsd") {
        format!("FreeBSD {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else {
        "Unknown".to_string()
    };
    
    // Create a variable distro where we call the function from the file ascii.rs. Создаем переменную distro, в которой мы вызываем функцию из файла ascii.rs.
    let distro = Distro::from_string(&os); // Измените &os на подходящую строку для дебага
    let art = distro.ascii_art();

    // Updating system information using sysinfo. Обновление системной информации с помощью sysinfo
    let mut sys = System::new_all();
    sys.refresh_all();

    // Create variables where we store information about the user, memory, and kernel from sysinfo, whoami, and os_release.
    // Создаем переменные, в которых мы будем хранить информацию о пользователе, памяти и ядре из sysinfo, whoami и os_release.
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
    } else if cfg!(target_os = "freebsd") {
        format!("kFreeBSD {}", System::kernel_version().unwrap_or("Unknown".to_string()))
    } else { 
        System::kernel_version().unwrap_or("Unknown".to_string())
    };

    // Check icon support and the presence of the --legacy or -l flag. Проверяем наличие значков и наличие флага --legacy или -l.
    let use_icons = font_detector::nerd_font() && !is_legacy;

    // Create a vector where we specify each module of our fetch. Создаем вектор где указываем каждый модуль нашего фетча
    let rsfetch = vec![
    ("os :",   "󰆥 :", os.clone(),               Color::Blue),
    ("user :", " :", username.clone(),         Color::Red),
    ("host :", "󰆋 :", hostname.clone(),         Color::White),
    ("ram :",  " :", format!("{}/{} MB", used_memory.clone() / 1024 / 1024, total_memory.clone() / 1024 / 1024), Color::Yellow),
    ("swap :", " :", format!("{}/{} MB", used_swap.clone() / 1024 / 1024, total_swap.clone() / 1024 / 1024), Color::Magenta),
    ("cpu :",  " :", format!("{} ({})", cpu_brand, cpu.clone()), Color::Red),
    ("krnl :", " :", kernel.clone(),           Color::Green),
];

    // Print ASCII art at the top, defining it in the file ascii.rs. Печатаем ASCII-арт сверху, определив его в файле ascii.rs
    println!("{}", art.cyan().bold());
    println!("{}", "—".repeat(70).dimmed());

    // Printing a pretty border. Печатаем красивую границу
    let prefix = "> |".green().bold();

    for (label, icon, value, color) in rsfetch {
        // Выбираем ключ: либо иконка, либо текст с выравниванием (например, 5 символов)
        let key = if use_icons {
        format!("   {}", icon)
    } else {
        format!("   {:<4}", label) // {:<4} выровняет модули (os, ram, cpu и прочее) по ширине.
    };

    println!("{} {} {}", prefix, key.color(color).bold(), value);
}
    // Displaying the program version and license
    println!("{}  {}", "©".cyan(), format!("RSFetch v{} | GNU GPLv3 License | 2026", env!("CARGO_PKG_VERSION")).dimmed());
}