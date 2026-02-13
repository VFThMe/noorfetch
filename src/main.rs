use sysinfo::*;
use whoami;
use colored::*;
//use os_release::OsRelease;
use std::*;
use std::time::Instant;
use std::io::{self, IsTerminal};

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
    
    let startup = Instant::now();

    // -- flags -- //
    let args: Vec<String> = env::args().collect();
    // debug
    let debug = args.iter().any(|a| a == "--debug" || a == "-d");
    // no color
    let no_color = args.iter().any(|a| a == "--no-color" || a == "-nc");
    // help
    if args.iter().any(|a| a == "--help" || a == "-h") {
        help_program();
        process::exit(0);
    }

    // -- check tty status -- //
    let isatty = io::stdout().is_terminal();
    
    // -- load config -- // 
    let cfg = config::load_config();
    
    // -- identifying OS -- //
    let os_name = System::name().unwrap_or("Unknown".to_string());
    let os_version = System::os_version().unwrap_or("?".to_string());

    let os = match () {
	_ if cfg!(target_os = "linux")   => format!("{} {}", os_name, os_version),
	_ if cfg!(target_os = "macos")   => format!("macOS {}", os_version),
	_ if cfg!(target_os = "freebsd") => format!("FreeBSD {}", os_version),
	_ if cfg!(target_os = "windows") => format!("Windows {}", os_version),
	_ => os_name,
    };
    
    // -- logo -- //
    let requested_logo = args.iter()
        .find(|&a| a.starts_with("--logo="))
        .and_then(|a| a.strip_prefix("--logo=").map(str::to_string));

    let logo_name = if let Some(flag_name) = requested_logo {
	flag_name
    } else if cfg.logo != "default" {
	cfg.logo.clone()
    } else {
	os.clone()
    };

    let mut distro = Distro::from_string(&logo_name);

    if matches!(distro, Distro::Unknown) && logo_name != "Unknown" {
	eprintln!("warning: logo '{}' not recognized, falling back to auto-detection", logo_name);
	distro = Distro::from_string(&os);
    }

    let art = distro.ascii_art();
    
    // -- Updating system information using sysinfo -- // 
    let mut sys = System::new_all();
    sys.refresh_all();
    get_wm();

    // -- no_color and isatty check -- //
    let use_color = !no_color && isatty;
    
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
   /* let environment   = if cfg!(target_os = "macos") {
	format!("Aqua MacOS")
    } else {
            let wm = environment::get_wm().unwrap_or_else(|| "Unknown".to_string());
            if wm.trim().is_empty() || wm.to_lowercase() == "tty" || wm == "Unknown" {
                "TTY".to_string()
            } else {
                wm
            }
    } else {
        "TTY".to_string()
    }; */

    let environment = if isatty {
        if cfg!(target_os = "macos") {
            "Aqua macOS".to_string()
        } else {
            let wm = environment::get_wm().unwrap_or_else(|| "Unknown".to_string());
            if wm.trim().is_empty() || wm.to_lowercase() == "tty" || wm == "Unknown" {
                "TTY".to_string()
            } else {
                wm
            }
        }
    } else {
        "TTY".to_string()
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

    if days != "Unknown".to_lowercase() && days != "0 days" && days != "0" {
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
    if debug {
        let ms = startup.elapsed().as_secs_f64() * 1000.0;
        println!("\nStartup time: {:.2} ms", ms);
    }
}

fn help_program() {
    let version = env!("CARGO_PKG_VERSION");
    println!(r#"
Noorfetch - a blazingly fetch, written in Rust!

Usage: noorfetch [OPTION]..

Options:
  -h,  --help        Display this help and exit
  -d,  --debug       Shows the time it took for the program to start
  -nc, --no-color    Disable color for module
  --logo=[DISTRO]    Display the ASCII art you specified

Noorfetch is licensed under GNU GPL v3.0 or later.
Official source in Codeberg: https://codeberg.org/limforge/noorfetch
Official source in Github: https://github.com/VFThMe/noorfetch 

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
