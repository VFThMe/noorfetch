use sysinfo::*;
use whoami;
use colored::*;
use os_release::OsRelease;
use std;

// Подключаем сторонние файлы из директории UX
// #[path = "Settings/detect_icons.rs"]
// mod font_detector;
#[path = "Settings/ascii.rs"]
mod ascii;
use ascii::Distro;
#[path = "Settings/environment.rs"]
mod environment;
use environment::get_wm;
// #[path = "Settings/config.rs"]
// mod config;
fn main() {
    // let cfg = config::load_config();

    // let path = config::get_path();
    // println!("Файл конфига находится тут: {:?}", path);

    // Создаем вектор аргументов командной строки и проверяем наличие флага --legacy или -l
    // let args: Vec<String> = env::args().collect();
    // let is_legacy = args.iter().any(|arg| arg == "--legacy" || arg == "-l");

    // Identifying OS. Определяем ОС
    let os = if cfg!(target_os = "windows") {
        format!("Windows {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else if cfg!(target_os = "linux") {
        OsRelease::new().ok().and_then(|r| Some(r.pretty_name)).unwrap_or("Linux".to_string())
    } else if cfg!(target_os = "macos") {
        format!("macOS {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else if cfg!(target_os = "freebsd") {
        format!("NetBSD {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else if cfg!(target_os = "netbsd") {
        format!("OpenBSD {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else if cfg!(target_os = "openbsd") {
        format!("FreeBSD {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else {
        "Unknown".to_string()
    };
   
    // Create a variable distro where we call the function from the file ascii.rs. Создаем переменную distro, в которой мы вызываем функцию из файла ascii.rs.
    let distro = Distro::from_string(&os); // Change &os to a suitable string for debugging. // Измените &os на подходящую строку для дебага
    let art = distro.ascii_art();

    // Updating system information using sysinfo. Обновление системной информации с помощью sysinfo
    let mut sys = System::new_all();
    sys.refresh_all();
    get_wm();

    // Create variables where we store information about the user, memory, and kernel from sysinfo, whoami, and os_release.
    // Создаем переменные, в которых мы будем хранить информацию о пользователе, памяти и ядре из sysinfo, whoami и os_release.
    let username      =       whoami::username().unwrap_or_else(|_| "<unknown>".to_string());
    let total_memory  =       sys.total_memory();
    let used_memory   =       sys.used_memory();
    let total_swap    =       sys.total_swap();
    let used_swap     =       sys.used_swap();
    let cpu           =       sys.cpus().len();
    let target_proc   =       1;
    let environment   = if cfg!(target_os = "windows") {
	format!("Explorer DE")
    } else if cfg!(target_os = "macos") {
	format!("Aqua MacOS")
    } else {
	environment::get_wm().unwrap_or_else(|| "Unknown".to_string())
    };
    let cpu_brand     = sys.cpus().get(0).map(|c| c.brand()).unwrap_or("Unknown CPU");
    let hostname      = System::host_name().unwrap_or("Unknown".to_string());
    let kernel        = if cfg!(target_os = "windows") { 
        format!("Windows NT {}", System::kernel_version().unwrap_or("Unknown".to_string())) 
    } else if cfg!(target_os = "macos") { 
        format!("Darwin {}", System::kernel_version().unwrap_or("Unknown".to_string())) 
    } else if cfg!(target_os = "freebsd") {
        format!("kFreeBSD {}", System::kernel_version().unwrap_or("Unknown".to_string()))
    } else { 
        System::kernel_version().unwrap_or("Unknown".to_string())
    };
    let init = if let Some(process) = sys.process(Pid::from(target_proc)) {
        process.name().to_string_lossy().into_owned()
    } else {
        "Unknown".to_string()
    };

    // Check icon support and the presence of the --legacy or -l flag. Проверяем наличие значков и наличие флага --legacy или -l.
    // let use_icons = font_detector::nerd_font() && !is_legacy;

    // Create a vector where we specify each module of our fetch. Создаем вектор где указываем каждый модуль нашего фетча
    let noorfetch = vec![
	("os ",      os.clone(),                 Color::TrueColor { r: 220, g: 138, b: 120 } ),      //   rosewater
	("user ",    username.clone(),           Color::TrueColor { r: 221, g: 120, b: 120 } ),     //    flamingo
	("host ",    hostname.clone(),           Color::TrueColor { r: 234, g: 118, b: 203, } ),   //     pink 
	("wm/de ",   environment,                Color::TrueColor { r: 136, g: 57, b: 239 } ),    //      mauve 
	("init ",    init,                       Color::TrueColor { r: 210, g: 15, b: 57  } ),   //       red
	
	("ram ",  format!("{}/{} MB", used_memory / 1048 / 1048, total_memory / 1048 / 1048), Color::TrueColor { r: 230, g: 69, b: 83, }),   // maroon
	("swap ", format!("{}/{} MB", used_swap / 1048 / 1048, total_swap / 1048 / 1048 ),     Color::TrueColor { r: 254, g: 100, b: 11, } ), // peach
	("cpu ",  format!("{} ({})", cpu_brand, cpu),                           Color::TrueColor { r: 223, g: 142, b: 29, }),  // yellow
	
	("krnl ",    kernel,                     Color::TrueColor { r: 64, g: 160, b: 43, }), // green
    ];
    
    // Create another vector. Создаем еще один вектор
    let mut info_lines: Vec<String> = Vec::new();
    
    // Header (user@host). Заголовок (user@host)
    info_lines.push(format!("{}@{}", username, hostname));
    info_lines.push("-".repeat(username.len() + hostname.len() + 1));

    for (label, value, color) in noorfetch {
        info_lines.push(format!("{:<6} {}", label.color(color).bold(), value));
    }

    let art_lines: Vec<&str> = art.lines().collect();
    let art_width = art_lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let padding = art_width + 5; 

    // Print. Вывод
    let max_l = std::cmp::max(art_lines.len(), info_lines.len());

    println!();
for i in 0..max_l {
        let art_row = art_lines.get(i).unwrap_or(&"");
        
        let info_row = match info_lines.get(i) {
            Some(row) => row.as_str(),
            None => "",
        };

        println!("{:<width$} {}", art_row, info_row, width = padding);
    }

    // Print ASCII art at the top, defining it in the file ascii.rs. Печатаем ASCII-арт сверху, определив его в файле ascii.rs
   // println!("{}", art.cyan().bold());
   // println!("{}", "—".repeat(70).dimmed());

    // Printing a pretty border. Печатаем красивую границу
    // let prefix = "> |".green().bold();

    // for (label, value, color) in noorfetch {
    //     // Выбираем ключ: либо иконка, либо текст с выравниванием (например, 5 символов)
    //     let key = format!("   {:<4}", label); // {:<4} выровняет модули (os, ram, cpu и прочее) по ширине.

    // println!("{} {} {}", prefix, key.color(color).bold(), value);
    // Displaying the program version and license
    // println!("{}  {}", "©".cyan(), format!("Noorfetch v{} | GNU GPLv3 License | 2026", env!("CARGO_PKG_VERSION")).dimmed());
}
