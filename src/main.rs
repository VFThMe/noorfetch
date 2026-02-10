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
#[path = "Settings/config.rs"]
mod config;
// use config::load_config;

fn main() {
    // Config. Конфигурационный файл
    let cfg = config::load_config();
    
    // Создаем вектор аргументов командной строки и проверяем наличие флага --legacy или -l
    let args: Vec<String> = env::args().collect();
    let no_color = args.iter().any(|arg| arg == "--no-color" || arg == "-nc");

    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
	help_program();
	std::process::exit(0);
    }
    // Identifying OS. Определяем ОС
    let os = if  cfg!(target_os = "linux") {
        OsRelease::new().ok().and_then(|r| Some(r.pretty_name)).unwrap_or("Linux".to_string())
    } else if cfg!(target_os = "macos") {
        format!("macOS {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else if cfg!(target_os = "freebsd") {
        format!("FreeBSD {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else if cfg!(target_os = "netbsd") {
        format!("NetBSD {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else if cfg!(target_os = "openbsd") {
        format!("OpenBSD {}", System::os_version().unwrap_or("Unknown".to_string()))
    } else {
        "Unknown".to_string()
    };

    let requested_logo = args.iter()
        .find(|&a| a.starts_with("--logo="))
        .and_then(|a| a.strip_prefix("--logo=").map(str::to_string));

    let distro = if let Some(name) = requested_logo {
        let d = Distro::from_string(&name);
        if matches!(d, Distro::Unknown) && !name.trim().is_empty() {
            eprintln!("warning: logo '{}' not recognized, using auto-detection", name);
            Distro::from_string(&os)
        } else {
            d
        }
    } else {
        Distro::from_string(&os)
    };

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
    let init = if let Some(process) = sys.process(Pid::from(target_proc)) {
        process.name().to_string_lossy().into_owned()
    } else {
        "Unknown".to_string()
    };

    // Check icon support and the presence of the --no-color or -nc flag. Проверяем наличие флага --no-color или -nc.
    let use_color = !no_color;
    
    let mut noorfetch: Vec<(String, String, Color)> = Vec::new();

    let add_if_enabled = |noorfetch: &mut Vec<(String, String, Color)>, key: &str, label: &str, value: String, color: Color| {
	if cfg.modules.get(key).map_or(true, |m| m.display) {
            noorfetch.push((label.to_string(), value, color));
	}
    };

    add_if_enabled(&mut noorfetch, "os", "os ", os.clone(), Color::TrueColor { r: 220, g: 138, b: 120 });
    add_if_enabled(&mut noorfetch, "user", "user ", username.clone(), Color::TrueColor { r: 221, g: 120, b: 120 });
    add_if_enabled(&mut noorfetch, "hostname", "host ", hostname.clone(), Color::TrueColor { r: 234, g: 118, b: 203 });
    add_if_enabled(&mut noorfetch, "wm", "wm/de ", environment.clone(), Color::TrueColor { r: 136, g: 57, b: 239 });

    add_if_enabled(
	&mut noorfetch,
	"ram",
	"ram ",
	format!("{}/{} MiB", used_memory / 1024 / 1024, total_memory / 1024 / 1024),
	Color::TrueColor { r: 230, g: 69, b: 83 },
    );
    if used_swap > 0 && cfg.modules.get("swap").map_or(true, |m| m.display) {
	noorfetch.push((
        "swap ".to_string(),
        format!("{}/{} MiB", used_swap / 1024 / 1024, total_swap / 1024 / 1024),
        Color::TrueColor { r: 254, g: 100, b: 11 },
    ));
}

    add_if_enabled(
	&mut noorfetch,
	"cpu",
	"cpu ",
	format!("{} ({})", cpu_brand, cpu),
	Color::TrueColor { r: 223, g: 142, b: 29 },
);


    add_if_enabled(&mut noorfetch, "krnl", "krnl ", kernel.clone(), Color::TrueColor { r: 64, g: 160, b: 43 });

    if days != "Unknown".to_lowercase() && days != "0" {
	add_if_enabled(&mut noorfetch, "days", "days ", days.clone(), Color::TrueColor { r: 23, g: 146, b: 153 });
    }
    
    add_if_enabled(&mut noorfetch, "init", "init", init, Color::TrueColor { r: 4, g: 165, b: 229 });

    // Create a new vector. Создаем новый вектор
    let mut info_lines: Vec<String> = Vec::new();

    info_lines.push(format!("{}@{}", username, hostname));
    info_lines.push("-".repeat(username.len() + hostname.len() + 1));

    // Проверяем наличие флага --no-color/nc и выводим фетч
    if use_color {
        for (label, value, color) in noorfetch {
            info_lines.push(format!("{:<6} {}", label.color(color).bold(), value));
        }
    } else {
        for (label, value, _) in noorfetch {
            info_lines.push(format!("{:<6} {}", label.bold(), value));
        }
    }

/*    let art_lines: Vec<&str> = art.lines().collect();
    let art_width = art_lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let padding = art_width + 5;
     */
    let art_lines: Vec<String> = art.lines().map(|s| s.to_string()).collect();
    let art_width = art_lines.iter().map(|l| visible_len(l)).max().unwrap_or(0);
    let padding = art_width + 5;
    let max_l = std::cmp::max(art_lines.len(), info_lines.len());

    println!();
    for i in 0..max_l {
        let art_row = art_lines.get(i).cloned().unwrap_or_default();
        let info_row = info_lines.get(i).map_or("", |s| s.as_str());
        
        // Считаем, сколько реально пробелов нужно добавить
        let visible = visible_len(&art_row);
        let current_padding = if padding > visible { padding - visible } else { 0 };
        
        println!("{}{:<width$} {}", art_row, "", info_row, width = current_padding);
    }
}

fn help_program() {
    let version = env!("CARGO_PKG_VERSION");
    println!(r#"
Noorfetch - a blazingly fetch, written in Rust!

Usage:
   noorfetch [FLAG]

Flags:
   -h,  --help        Help flag
   -nc, --no-color    Disable colo(u)r for module
   --logo=[DISTRO]    Display the ASCII art you specified

  Noorfetch is licensed under GNU GPL v3.0 or later.
  Official source: https://codeberg.org/limforge/noorfetch

Noorfetch v{}. 2026. limforge."#, version);
}
 fn visible_len(s: &str) -> usize {
    let mut is_ansi = false;
    let mut count = 0;
    
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\x1b' && i + 1 < chars.len() && chars[i+1] == '[' {
            is_ansi = true;
            i += 2;
            continue;
        }
        if is_ansi {
            if chars[i].is_ascii_alphabetic() {
                is_ansi = false;
            }
            i += 1;
            continue;
        }
        count += 1;
        i += 1;
    }
    count
}
