use sysinfo::{
    System, Pid
};
use std::*;
use std::time::Instant;
use std::io::{self, IsTerminal};

#[path = "Settings/ascii.rs"]
mod ascii;
use ascii::Distro;
#[path = "Settings/environment.rs"]
mod environment;
#[path = "Settings/date.rs"]
mod date;
#[path = "Settings/config.rs"]
mod config;

fn main() {

    let startup = Instant::now();

    // -- flags -- //
    let args: Vec<String> = env::args().collect();
    let debug    = args.iter().any(|a| a == "--debug"    || a == "-d");
    let no_color = args.iter().any(|a| a == "--no-color" || a == "-nc");

    if args.iter().any(|a| a == "--help" || a == "-h") {
        help_program();
        process::exit(0);
    }

    // -- check tty status -- //
    let isatty = io::stdout().is_terminal();

    // -- load config -- //
    let cfg = config::load_config();

    // -- identifying OS -- //
    let os_name    = System::name().unwrap_or("Unknown".to_string());
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

    // -- custom ASCII art from .txt file -- //
    let custom_art: Option<String> = cfg.custom_art
        .as_deref()
        .filter(|p| !p.is_empty())
        .and_then(|path| {
            if !path.ends_with(".txt") {
                eprintln!("warning: custom_art '{}' is not a .txt file, ignoring", path);
                return None;
            }
            let expanded = if path.starts_with("~/") {
                let home = std::env::var("HOME").unwrap_or_default();
                format!("{}/{}", home, &path[2..])
            } else {
                path.to_string()
            };
            match std::fs::read_to_string(&expanded) {
                Ok(content) => Some(content),
                Err(e) => {
                    eprintln!("warning: could not read custom_art '{}': {}, falling back to logo", expanded, e);
                    None
                }
            }
        });

    let art = if let Some(custom) = custom_art {
        custom
    } else {
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

        distro.ascii_art()
    };

    // -- Updating system information -- //
    let mut sys = System::new();
    sys.refresh_memory();
    sys.refresh_cpu_list(sysinfo::CpuRefreshKind::nothing());
    sys.refresh_processes_specifics(
        sysinfo::ProcessesToUpdate::Some(&[Pid::from(1)]),
        false,
        sysinfo::ProcessRefreshKind::nothing(),
    );

    // -- no_color and isatty check -- //
    let use_color = !no_color && isatty;

    // -- gather info -- //
    let username = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "<unknown>".to_string());

    let total_memory = sys.total_memory();
    let used_memory  = sys.used_memory();
    let total_swap   = sys.total_swap();
    let used_swap    = sys.used_swap();
    let cpu_count    = sys.cpus().len();
    let cpu_brand    = sys.cpus().get(0).map(|c| c.brand()).unwrap_or("Unknown CPU");
    let hostname     = System::host_name().unwrap_or("Unknown".to_string());
    let days         = date::get_install_days();

    let kernel = if cfg!(target_os = "windows") {
        format!("Windows NT {}", System::kernel_version().unwrap_or("Unknown".to_string()))
    } else if cfg!(target_os = "macos") {
        format!("Darwin {}", System::kernel_version().unwrap_or("Unknown".to_string()))
    } else if cfg!(target_os = "freebsd") {
        format!("kFreeBSD {}", System::kernel_version().unwrap_or("Unknown".to_string()))
    } else {
        System::kernel_version().unwrap_or("Unknown".to_string())
    };

    let init = if let Some(process) = sys.process(Pid::from(1)) {
        process.name().to_string_lossy().into_owned()
    } else {
        "Unknown".to_string()
    };

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

    let mut entries: Vec<(u32, String, String, (u8, u8, u8))> = Vec::new();

    macro_rules! add_simple {
        ($key:expr, $fallback_label:expr, $value:expr, $fallback_color:expr) => {
            if let Some(m) = cfg.modules.get($key) {
                if m.display {
                    let label = m.resolve_label($fallback_label).to_string();
                    let color = if m.color.is_some() {
                        m.resolve_color()
                    } else {
                        $fallback_color
                    };
                    let value = m.format_value(&[("value", &$value)]);
                    entries.push((m.order, label, value, color));
                }
            }
        };
    }

    add_simple!("os",       "os",    os.clone(),          (220, 138, 120));
    add_simple!("user",     "user",  username.clone(),    (221, 120, 120));
    add_simple!("hostname", "host",  hostname.clone(),    (234, 118, 203));
    add_simple!("wm",       "wm/de", environment.clone(), (136,  57, 239));

    if let Some(m) = cfg.modules.get("ram") {
        if m.display {
            let used  = (used_memory  / 1024 / 1024).to_string();
            let total = (total_memory / 1024 / 1024).to_string();
            let value = m.format_value(&[
                ("used",  &used),
                ("total", &total),
                ("value", &format!("{}/{} MiB", used, total)),
            ]);
            let label = m.resolve_label("ram").to_string();
            let color = if m.color.is_some() { m.resolve_color() } else { (230, 69, 83) };
            entries.push((m.order, label, value, color));
        }
    }
    
    if used_swap > 0 {
        if let Some(m) = cfg.modules.get("swap") {
            if m.display {
                let used  = (used_swap  / 1024 / 1024).to_string();
                let total = (total_swap / 1024 / 1024).to_string();
                let value = m.format_value(&[
                    ("used",  &used),
                    ("total", &total),
                    ("value", &format!("{}/{} MiB", used, total)),
                ]);
                let label = m.resolve_label("swap").to_string();
                let color = if m.color.is_some() { m.resolve_color() } else { (254, 100, 11) };
                entries.push((m.order, label, value, color));
            }
        }
    }

    if let Some(m) = cfg.modules.get("cpu") {
        if m.display {
            let cores = cpu_count.to_string();
            let value = m.format_value(&[
                ("brand",  cpu_brand),
                ("cores",  &cores),
                ("value",  &format!("{} ({})", cpu_brand, cores)),
            ]);
            let label = m.resolve_label("cpu").to_string();
            let color = if m.color.is_some() { m.resolve_color() } else { (223, 142, 29) };
            entries.push((m.order, label, value, color));
        }
    }

    add_simple!("krnl", "krnl", kernel.clone(), (64, 160, 43));
    
    if days != "unknown" && days != "0 days" && days != "0" {
        add_simple!("days", "days", days.clone(), (23, 146, 153));
    }

    add_simple!("init", "init", init.clone(), (4, 165, 229));
    entries.sort_by_key(|(order, ..)| *order);

    let noorfetch: Vec<(String, String, (u8, u8, u8))> = entries
        .into_iter()
        .map(|(_, label, value, color)| (label, value, color))
        .collect();

    let mut info_lines: Vec<String> = Vec::new();

    info_lines.push(format!("{}@{}", username, hostname));
    info_lines.push("-".repeat(username.len() + hostname.len() + 1));

    let label_width = noorfetch.iter().map(|(l, _, _)| l.len()).max().unwrap_or(6);

    if use_color {
        for (label, value, (r, g, b)) in &noorfetch {
            let colored = ansi_bold_color(label, *r, *g, *b);
            let pad = label_width.saturating_sub(visible_len(&colored));
            info_lines.push(format!("{}{} {}", colored, " ".repeat(pad), value));
        }
    } else {
        for (label, value, _) in &noorfetch {
            let bold = ansi_bold(label);
            let pad = label_width.saturating_sub(visible_len(&bold));
            info_lines.push(format!("{}{} {}", bold, " ".repeat(pad), value));
        }
    }

    let art_lines: Vec<String> = art.lines().map(|s| s.to_string()).collect();
    let art_width  = art_lines.iter().map(|l| visible_len(l)).max().unwrap_or(0);
    let padding    = art_width + 5;
    let max_l      = std::cmp::max(art_lines.len(), info_lines.len());

    println!();
    for i in 0..max_l {
        let art_row  = art_lines.get(i).cloned().unwrap_or_default();
        let info_row = info_lines.get(i).map_or("", |s| s.as_str());

        let visible          = visible_len(&art_row);
        let current_padding  = if padding > visible { padding - visible } else { 0 };

        println!("{}{:<width$} {}", art_row, "", info_row, width = current_padding);
    }

    if debug {
        let ms = startup.elapsed().as_secs_f64() * 1000.0;
        println!("\nStartup time: {:.2} ms", ms);
    }
}

fn ansi_bold_color(s: &str, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[1;38;2;{};{};{}m{}\x1b[0m", r, g, b, s)
}

fn ansi_bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

fn visible_len(s: &str) -> usize {
    let mut is_ansi = false;
    let mut count   = 0;
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\x1b' && i + 1 < chars.len() && chars[i + 1] == '[' {
            is_ansi = true;
            i += 2;
            continue;
        }
        if is_ansi {
            if chars[i].is_ascii_alphabetic() { is_ansi = false; }
            i += 1;
            continue;
        }
        count += 1;
        i += 1;
    }
    count
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

Noorfetch is licensed under GNU LGPL v3.0 or later.
Official source in Codeberg: https://codeberg.org/limforge/noorfetch
Official source in Github: https://github.com/VFThMe/noorfetch

Noorfetch v{}. 2026. limforge."#, version);
}
