use sysinfo::*;
use whoami;
use colored::*;
use os_release::OsRelease;
use std::*;

// Подключаем сторонние файлы из директории UX
// #[path = "Settings/detect_icons.rs"]
// mod font_detector;
#[path = "Settings/ascii.rs"]
mod ascii;
use ascii::Distro;
#[path = "Settings/environment.rs"]
mod environment;
use environment::get_wm;
#[path = "Settings/date.rs"]
mod date;
// #[path = "Settings/config.rs"]
// mod config;
fn main() {
    // let cfg = config::load_config();

    // let path = config::get_path();
    // println!("Файл конфига находится тут: {:?}", path);

    // Создаем вектор аргументов командной строки и проверяем наличие флага --legacy или -l
    let args: Vec<String> = env::args().collect();
    let no_color = args.iter().any(|arg| arg == "--no-color" || arg == "-nc");

    // Identifying OS. Определяем ОС
    let os = if  cfg!(target_os = "linux") {
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
//    let target_proc   =       1;
    let days          =       date::get_install_days();
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
/*    let init = if let Some(process) = sys.process(Pid::from(target_proc)) {
        process.name().to_string_lossy().into_owned()
    } else {
        "Unknown".to_string()
    }; */

    // Check icon support and the presence of the --no-color or -nc flag. Проверяем наличие флага --no-color или -nc.
    let use_color = !no_color;

    let mut noorfetch = Vec::new();
	noorfetch.push(( "os ".to_string(),      os.clone(),       Color::TrueColor { r: 220, g: 138, b: 120  }  ));
        noorfetch.push(( "user ".to_string(),    username.clone(), Color::TrueColor { r: 221, g: 120, b: 120  }  ));
        noorfetch.push(( "host ".to_string(),    hostname.clone(), Color::TrueColor { r: 234, g: 118, b: 203 }   ));
        noorfetch.push(( "wm/de ".to_string(),   environment,      Color::TrueColor { r: 136, g: 57,  b: 239  }  ));
    
    	noorfetch.push(("ram ".to_string(),  format!("{}/{} MB", used_memory / 1048 / 1048, total_memory / 1048 / 1048), Color::TrueColor { r: 230, g: 69, b: 83, }));
        if used_swap > 0 {
	    noorfetch.push((
	    "swap ".to_string(), format!("{}/{} MB",
	    used_swap / 1048 / 1048,
	    total_swap / 1048 / 1048 ),
	    Color::TrueColor { r: 254, g: 100, b: 11, } )); // peach
        } else {}
        noorfetch.push((  "cpu ".to_string(),     format!("{} ({})", cpu_brand, cpu),  Color::TrueColor { r: 223, g: 142, b: 29, }));
        noorfetch.push((  "krnl ".to_string(),    kernel,                     Color::TrueColor { r: 64, g: 160, b: 43, }));
    if days != "Unknown".to_lowercase() && days != "0" {
        noorfetch.push(( "days ".to_string(),    days,                       Color::TrueColor { r: 23, g: 146, b: 153, }));
} else {}
            
    // Create another vector. Создаем еще один вектор
    let mut info_lines: Vec<String> = Vec::new();
    
    // Header (user@host). Заголовок (user@host)
    info_lines.push(format!("{}@{}", username, hostname));
    info_lines.push("-".repeat(username.len() + hostname.len() + 1));

    if use_color { 
	for (label, value, color) in noorfetch {
            info_lines.push(format!("{:<6} {}", label.color(color).bold(), value))
	}
    } else {
	for (label, value, _color) in noorfetch {
	    info_lines.push(format!("{:<6} {}", label.bold(), value))
	}
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
}
