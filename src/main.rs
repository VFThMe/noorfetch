use std::env;
use std::io::{self, IsTerminal};
use std::mem;
use std::process;
use std::time::Instant;

#[path = "init/ascii.rs"]
mod ascii;
use ascii::Distro;
#[path = "init/config.rs"]
mod config;
#[path = "init/date.rs"]
mod date;
#[path = "init/environment.rs"]
mod environment;

fn read_file(path: &str) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

fn os_release_field(content: &str, key: &str) -> Option<String> {
    content
        .lines()
        .find(|l| l.starts_with(key))
        .and_then(|l| l.splitn(2, '=').nth(1))
        .map(|v| v.trim_matches('"').to_string())
}
// --- get sys. info (OS, krnl, shell, hostname and etc.) --- //
fn get_os() -> String {
    if cfg!(target_os = "linux") {
        let content = read_file("/etc/os-release")
            .or_else(|| read_file("/usr/lib/os-release"))
            .unwrap_or_default();
        let name = os_release_field(&content, "NAME").unwrap_or_else(|| "Linux".to_string());
        let version = os_release_field(&content, "VERSION_ID").unwrap_or_default();
        if version.is_empty() {
            name
        } else {
            format!("{} {}", name, version)
        }
    } else if cfg!(target_os = "freebsd") {
        let version = read_file("/etc/version")
            .or_else(|| {
                process::Command::new("uname")
                    .arg("-r")
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .map(|s| s.trim().to_string())
            })
            .unwrap_or_else(|| "Unknown".to_string());
        format!("FreeBSD {}", version.trim())
    } else {
        "Unknown".to_string()
    }
}

fn get_kernel() -> String {
    if cfg!(target_os = "freebsd") {
        let ver = process::Command::new("uname")
            .arg("-r")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        format!("kFreeBSD {}", ver)
    } else {
        read_file("/proc/version")
            .and_then(|s| s.split_whitespace().nth(2).map(str::to_string))
            .unwrap_or_else(|| "Unknown".to_string())
    }
}

fn get_hostname() -> String {
    read_file("/proc/sys/kernel/hostname")
        .or_else(|| read_file("/etc/hostname"))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_meminfo() -> (u64, u64, u64, u64) {
    let content = read_file("/proc/meminfo").unwrap_or_default();
    let mut mem_total = 0u64;
    let mut mem_avail = 0u64;
    let mut swap_total = 0u64;
    let mut swap_free = 0u64;

    for line in content.lines() {
        let parse = |l: &str| -> u64 {
            l.split_whitespace()
                .nth(1)
                .and_then(|v| v.parse().ok())
                .unwrap_or(0)
        };
        if line.starts_with("MemTotal:") {
            mem_total = parse(line);
        } else if line.starts_with("MemAvailable:") {
            mem_avail = parse(line);
        } else if line.starts_with("SwapTotal:") {
            swap_total = parse(line);
        } else if line.starts_with("SwapFree:") {
            swap_free = parse(line);
        }
    }

    (
        mem_total * 1024,
        mem_total.saturating_sub(mem_avail) * 1024,
        swap_total * 1024,
        swap_total.saturating_sub(swap_free) * 1024,
    )
}

fn get_cpu() -> (String, usize) {
    let content = read_file("/proc/cpuinfo").unwrap_or_default();
    let brand = content
        .lines()
        .find(|l| l.starts_with("model name"))
        .and_then(|l| l.splitn(2, ':').nth(1))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string());
    let cores = content
        .lines()
        .filter(|l| l.starts_with("processor"))
        .count();
    (brand, cores.max(1))
}

fn get_disks() -> Vec<(String, u64, u64)> {
    use std::collections::HashMap;
    let content = read_file("/proc/mounts").unwrap_or_default();
    let mut disk_map: HashMap<String, (u64, u64)> = HashMap::new();

    for line in content.lines() {
        let mut parts = line.split_whitespace();
        let dev = parts.next().unwrap_or("");
        let mount = parts.next().unwrap_or("");
        if !dev.starts_with("/dev/") {
            continue;
        }

        let base = dev
            .trim_end_matches(|c: char| c.is_ascii_digit())
            .trim_end_matches('p')
            .to_string();

        let (total, avail) = {
            let mut st: libc::statvfs = unsafe { mem::zeroed() };
            let path = std::ffi::CString::new(mount).unwrap_or_default();
            if unsafe { libc::statvfs(path.as_ptr(), &mut st) } != 0 {
                continue;
            }
            let bsize = st.f_frsize as u64;
            (st.f_blocks as u64 * bsize, st.f_bavail as u64 * bsize)
        };

        let entry = disk_map.entry(base).or_insert((0, 0));
        entry.0 = entry.0.max(total);
        entry.1 = entry.1.max(avail);
    }

    let mut names: Vec<String> = disk_map.keys().cloned().collect();
    names.sort();
    names
        .into_iter()
        .filter_map(|n| {
            let (t, a) = disk_map[&n];
            if t > 0 { Some((n, t, a)) } else { None }
        })
        .collect()
}

fn get_init() -> String {
    read_file("/proc/1/comm")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn main() {
    let startup = Instant::now();

    let args: Vec<String> = env::args().collect();
    let debug = args.iter().any(|a| a == "--debug" || a == "-d");
    let no_color = args.iter().any(|a| a == "--no-color" || a == "-nc");

    if args.iter().any(|a| a == "--help" || a == "-h") {
        help_program();
        process::exit(0);
    }

    let isatty = io::stdout().is_terminal();
    let cfg = config::load_config();
    let os = get_os();

    let requested_logo = args
        .iter()
        .find(|a| a.starts_with("--logo="))
        .and_then(|a| a.strip_prefix("--logo=").map(str::to_string));

    let custom_art: Option<String> = cfg
        .custom_art
        .as_deref()
        .filter(|p| !p.is_empty())
        .and_then(|path| {
            if !path.ends_with(".txt") {
                eprintln!(
                    "warning: custom_art '{}' is not a .txt file, ignoring",
                    path
                );
                return None;
            }
            let expanded = if path.starts_with("~/") {
                let home = env::var("HOME").unwrap_or_default();
                format!("{}/{}", home, &path[2..])
            } else {
                path.to_string()
            };
            match std::fs::read_to_string(&expanded) {
                Ok(content) => Some(content),
                Err(e) => {
                    eprintln!(
                        "warning: could not read custom_art '{}': {}, falling back to logo",
                        expanded, e
                    );
                    None
                }
            }
        });

    let art = if let Some(custom) = custom_art {
        custom
    } else {
        let logo_name = requested_logo
            .or_else(|| {
                if cfg.logo != "default" {
                    Some(cfg.logo.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| os.clone());

        let mut distro = Distro::from_string(&logo_name);
        if matches!(distro, Distro::Unknown) && logo_name != "Unknown" {
            eprintln!(
                "warning: logo '{}' not recognized, falling back to auto-detection",
                logo_name
            );
            distro = Distro::from_string(&os);
        }
        distro.ascii_art()
    };

    let use_color = !no_color && isatty;

    let username = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "<unknown>".to_string());

    let (total_memory, used_memory, total_swap, used_swap) = get_meminfo();
    let (cpu_brand, cpu_count) = get_cpu();
    let hostname = get_hostname();
    let days = date::get_install_days();
    let kernel = get_kernel();
    let init = get_init();

    let environment = if isatty {
        let wm = environment::get_wm().unwrap_or_else(|| "Unknown".to_string());
        if wm.trim().is_empty() || wm.to_lowercase() == "tty" || wm == "Unknown" {
            "TTY".to_string()
        } else {
            wm
        }
    } else {
        "TTY".to_string()
    };

    let shell = environment::get_shell().unwrap_or_else(|| "Unknown".to_string());

    let mut entries: Vec<(u32, String, String, (u8, u8, u8))> = Vec::new();

    let add_simple = |entries: &mut Vec<_>,
                      key: &str,
                      fallback_label: &str,
                      value: String,
                      fallback_color: (u8, u8, u8)| {
        if let Some(m) = cfg.modules.get(key) {
            if m.display {
                let label = m.resolve_label(fallback_label).to_string();
                let color = m.resolve_color(fallback_color);
                let value = m.format_value(&[("value", &value)]);
                entries.push((m.order, label, value, color));
            }
        }
    };

    let push_memory_module = |entries: &mut Vec<_>,
                              key: &str,
                              used_bytes: u64,
                              total_bytes: u64,
                              fallback_color: (u8, u8, u8)| {
        if let Some(m) = cfg.modules.get(key) {
            if m.display {
                let used = (used_bytes / 1024 / 1024).to_string();
                let total = (total_bytes / 1024 / 1024).to_string();
                let value = m.format_value(&[
                    ("used", &used),
                    ("total", &total),
                    ("value", &format!("{}/{} MiB", used, total)),
                ]);
                entries.push((
                    m.order,
                    m.resolve_label(key).to_string(),
                    value,
                    m.resolve_color(fallback_color),
                ));
            }
        }
    };

    add_simple(&mut entries, "os", "os", os.clone(), (220, 138, 120));
    add_simple(
        &mut entries,
        "user",
        "user",
        username.clone(),
        (221, 120, 120),
    );
    add_simple(
        &mut entries,
        "hostname",
        "host",
        hostname.clone(),
        (234, 118, 203),
    );
    add_simple(
        &mut entries,
        "shell",
        "shell",
        shell.clone(),
        (32, 159, 181),
    );
    add_simple(
        &mut entries,
        "wm",
        "wm/de",
        environment.clone(),
        (136, 57, 239),
    );

    push_memory_module(
        &mut entries,
        "ram",
        used_memory,
        total_memory,
        (230, 69, 83),
    );

    if used_swap > 0 {
        push_memory_module(&mut entries, "swap", used_swap, total_swap, (254, 100, 11));
    }

    if let Some(m) = cfg.modules.get("cpu") {
        if m.display {
            let cores = cpu_count.to_string();
            let value = m.format_value(&[
                ("brand", &cpu_brand),
                ("cores", &cores),
                ("value", &format!("{} ({})", cpu_brand, cores)),
            ]);
            entries.push((
                m.order,
                m.resolve_label("cpu").to_string(),
                value,
                m.resolve_color((223, 142, 29)),
            ));
        }
    }

    if let Some(m) = cfg.modules.get("disk") {
        if m.display {
            let color = m.resolve_color((137, 180, 250));

            for (name, total, avail) in get_disks() {
                let used = total.saturating_sub(avail);
                let pct = (used as f64 / total as f64 * 100.0) as u32;

                let bar_total = 10usize;
                let bar_filled = (pct as usize * bar_total / 100).min(bar_total);

                let (br, bg, bb) = if pct < 50 {
                    (166u8, 227u8, 161u8)
                } else if pct < 80 {
                    (249u8, 226u8, 175u8)
                } else {
                    (243u8, 139u8, 168u8)
                };

                let filled_block = if use_color {
                    format!(
                        "\x1b[38;2;{};{};{}m{}\x1b[0m",
                        br,
                        bg,
                        bb,
                        "█".repeat(bar_filled)
                    )
                } else {
                    "█".repeat(bar_filled)
                };

                let bar = format!("[{}{}]", filled_block, " ".repeat(bar_total - bar_filled));
                let used_gb = used / 1024 / 1024 / 1024;
                let total_gb = total / 1024 / 1024 / 1024;
                let value = format!("{} {} {}/{} GB ({}%)", name, bar, used_gb, total_gb, pct);

                entries.push((m.order, "disks".to_string(), value, color));
            }
        }
    }

    add_simple(&mut entries, "krnl", "krnl", kernel.clone(), (64, 160, 43));

    if let Some(days_val) = days {
        add_simple(&mut entries, "days", "days", days_val, (23, 146, 153));
    }

    add_simple(&mut entries, "init", "init", init.clone(), (4, 165, 229));

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

    let art_lines: Vec<&str> = art.lines().collect();
    let art_width = art_lines.iter().map(|l| visible_len(l)).max().unwrap_or(0);
    let padding = art_width + 5;
    let max_l = art_lines.len().max(info_lines.len());

    println!();
    for i in 0..max_l {
        let art_row = art_lines.get(i).copied().unwrap_or("");
        let info_row = info_lines.get(i).map_or("", |s| s.as_str());

        let visible = visible_len(art_row);
        let current_padding = padding.saturating_sub(visible);

        println!(
            "{}{:<width$} {}",
            art_row,
            "",
            info_row,
            width = current_padding
        );
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
    let mut in_ansi = false;
    let mut count = 0;
    let mut prev = '\0';
    for ch in s.chars() {
        if prev == '\x1b' && ch == '[' {
            in_ansi = true;
        } else if in_ansi {
            if ch.is_ascii_alphabetic() {
                in_ansi = false;
            }
        } else if ch != '\x1b' {
            count += if is_wide_char(ch) { 2 } else { 1 };
        }
        prev = ch;
    }
    count
}

fn is_wide_char(c: char) -> bool {
    let cp = c as u32;
    matches!(cp,
        0x1100..=0x115F  |
        0x2E80..=0x303E  |
        0x3041..=0x33BF  |
        0x33FF..=0xA4C6  |
        0xA960..=0xA97C  |
        0xAC00..=0xD7A3  |
        0xF900..=0xFAFF  |
        0xFE10..=0xFE19  |
        0xFE30..=0xFE6B  |
        0xFF01..=0xFF60  |
        0xFFE0..=0xFFE6  |
        0x1B000..=0x1B001|
        0x1F000..=0x1F9FF|
        0x20000..=0x3FFFD
    )
}

fn help_program() {
    let version = env!("CARGO_PKG_VERSION");
    println!(
        r#"
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

Noorfetch v{}. 2026. limforge."#,
        version
    );
}
