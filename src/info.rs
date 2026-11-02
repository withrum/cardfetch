use std::ffi::CString;
use std::fs;
use std::mem::MaybeUninit;
use std::path::Path;

pub struct Distro {
    pub name: String,
    pub icon: &'static str,
    pub ansi_color: u8, // ANSI color code 1..7
}

pub struct SystemInfo {
    pub user: String,
    pub host: String,
    pub distro: Distro,
    pub kernel: String,
    pub uptime: String,
    pub wm: String,
    pub shell: String,
    pub memory: String,
    pub disk: String,
    pub packages: String,
}

impl SystemInfo {
    pub fn collect() -> Self {
        let (user, host) = get_user_and_host();
        let distro = get_distro();
        let kernel = get_kernel();
        let uptime = get_uptime();
        let wm = get_wm();
        let shell = get_shell();
        let memory = get_memory();
        let disk = get_disk();
        let packages = get_packages();

        SystemInfo {
            user,
            host,
            distro,
            kernel,
            uptime,
            wm,
            shell,
            memory,
            disk,
            packages,
        }
    }
}

fn get_user_and_host() -> (String, String) {
    let user = std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .unwrap_or_else(|_| "user".to_string());

    let host = fs::read_to_string("/proc/sys/kernel/hostname")
        .map(|s| s.trim().to_string())
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "localhost".to_string());

    (user, host)
}

fn get_distro() -> Distro {
    let mut pretty_name = None;
    let mut name = None;
    let mut id = None;
    let mut id_like = None;

    let os_release_paths = ["/etc/os-release", "/usr/lib/os-release"];
    for path in os_release_paths {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((k, v)) = line.split_once('=') {
                    let k = k.trim();
                    let v = v.trim().trim_matches('"').trim_matches('\'');
                    match k {
                        "PRETTY_NAME" => pretty_name = Some(v.to_string()),
                        "NAME" => name = Some(v.to_string()),
                        "ID" => id = Some(v.to_lowercase()),
                        "ID_LIKE" => id_like = Some(v.to_lowercase()),
                        _ => {}
                    }
                }
            }
            if pretty_name.is_some() || name.is_some() {
                break;
            }
        }
    }

    let os_name = pretty_name.or(name).unwrap_or_else(|| "Linux".to_string());
    let id_str = id.unwrap_or_default();
    let like_str = id_like.unwrap_or_default();

    let (icon, ansi_color) = match_distro_icon(&id_str, &like_str);

    Distro {
        name: os_name,
        icon,
        ansi_color,
    }
}

fn match_distro_icon(id: &str, id_like: &str) -> (&'static str, u8) {
    let check = |s: &str| id.contains(s) || id_like.contains(s);

    if check("endeavouros") {
        ("", 5) // Magenta
    } else if check("arch") {
        ("󰣇", 6) // Cyan
    } else if check("ubuntu") {
        ("󰕈", 3) // Yellow
    } else if check("debian") {
        ("󰣚", 1) // Red
    } else if check("fedora") {
        ("󰣛", 4) // Blue
    } else if check("nixos") {
        ("󱄅", 6) // Cyan
    } else if check("void") {
        ("󰣝", 2) // Green
    } else if check("alpine") {
        ("󰣠", 4) // Blue
    } else if check("gentoo") {
        ("󰣨", 5) // Magenta
    } else if check("opensuse") || check("suse") {
        ("󰣧", 2) // Green
    } else if check("manjaro") {
        ("󱘊", 2) // Green
    } else if check("pop") {
        ("󰣡", 6) // Cyan
    } else if check("mint") {
        ("󰣭", 2) // Green
    } else if check("rhel") || check("centos") || check("rocky") || check("alma") {
        ("󱄛", 1) // Red
    } else if check("kali") {
        ("󰠲", 4) // Blue
    } else {
        ("󰌽", 4) // Blue (Generic Linux)
    }
}

fn get_kernel() -> String {
    if let Ok(content) = fs::read_to_string("/proc/sys/kernel/osrelease") {
        let k = content.trim();
        if !k.is_empty() {
            return k.to_string();
        }
    }

    // Fallback: libc uname
    let mut uts = MaybeUninit::<libc::utsname>::uninit();
    if unsafe { libc::uname(uts.as_mut_ptr()) } == 0 {
        let uts = unsafe { uts.assume_init() };
        let release = unsafe { std::ffi::CStr::from_ptr(uts.release.as_ptr()) };
        if let Ok(s) = release.to_str() {
            if !s.is_empty() {
                return s.to_string();
            }
        }
    }

    "unknown".to_string()
}

fn get_uptime() -> String {
    let content = match fs::read_to_string("/proc/uptime") {
        Ok(c) => c,
        Err(_) => return "unknown".to_string(),
    };

    let first_token = match content.split_whitespace().next() {
        Some(t) => t,
        None => return "unknown".to_string(),
    };

    let sec: f64 = match first_token.parse() {
        Ok(s) => s,
        Err(_) => return "unknown".to_string(),
    };

    let total_secs = sec as u64;
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else if mins > 0 {
        format!("{}m", mins)
    } else {
        format!("{}s", secs)
    }
}

fn get_wm() -> String {
    // 1. Wayland specific env vars
    if std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        return "Hyprland".to_string();
    }
    if std::env::var("SWAYSOCK").is_ok() {
        return "sway".to_string();
    }
    if std::env::var("RIVER_SOCKET").is_ok() {
        return "river".to_string();
    }
    if std::env::var("WAYFIRE_CONFIG_FILE").is_ok() {
        return "wayfire".to_string();
    }

    // 2. XDG_CURRENT_DESKTOP
    if let Ok(val) = std::env::var("XDG_CURRENT_DESKTOP") {
        let clean = val.trim();
        if !clean.is_empty() {
            // Might be colon-separated, e.g. "ubuntu:GNOME"
            let primary = clean.split(':').last().unwrap_or(clean).trim();
            if !primary.is_empty() && !primary.eq_ignore_ascii_case("x-generic") {
                return format_wm_name(primary);
            }
        }
    }

    // 3. DESKTOP_SESSION
    if let Ok(val) = std::env::var("DESKTOP_SESSION") {
        let clean = val.trim();
        if !clean.is_empty() && !clean.eq_ignore_ascii_case("default") {
            let base = clean.split('/').last().unwrap_or(clean);
            return format_wm_name(base);
        }
    }

    // 4. WINDOWMANAGER
    if let Ok(val) = std::env::var("WINDOWMANAGER") {
        let clean = val.trim();
        if !clean.is_empty() {
            let base = clean.split('/').last().unwrap_or(clean);
            return format_wm_name(base);
        }
    }

    // 5. Scan /proc for known WMs
    if let Some(wm) = find_wm_from_proc() {
        return format_wm_name(&wm);
    }

    "unknown".to_string()
}

fn format_wm_name(raw: &str) -> String {
    match raw.to_lowercase().as_str() {
        "kde" | "plasma" => "KDE Plasma".to_string(),
        "gnome" => "GNOME".to_string(),
        "xfce" | "xfce4" => "XFCE".to_string(),
        "mate" => "MATE".to_string(),
        "cinnamon" => "Cinnamon".to_string(),
        "lxqt" => "LXQt".to_string(),
        "cosmic" => "COSMIC".to_string(),
        "hyprland" => "Hyprland".to_string(),
        "niri" => "niri".to_string(),
        "sway" => "sway".to_string(),
        "river" => "river".to_string(),
        "wayfire" => "wayfire".to_string(),
        "i3" => "i3".to_string(),
        "bspwm" => "bspwm".to_string(),
        "dwm" => "dwm".to_string(),
        "awesome" => "awesome".to_string(),
        "xmonad" => "xmonad".to_string(),
        "openbox" => "Openbox".to_string(),
        "qtile" => "qtile".to_string(),
        _ => raw.to_string(),
    }
}

fn find_wm_from_proc() -> Option<String> {
    let known_wms = [
        "niri", "hyprland", "sway", "river", "wayfire", "labwc", "weston",
        "i3", "bspwm", "dwm", "awesome", "xmonad", "openbox", "qtile",
        "fluxbox", "herbstluftwm", "spectrwm", "compiz", "mutter",
        "kwin_wayland", "kwin_x11", "kwin", "enlightenment", "pekwm", "icewm",
    ];

    let proc_dir = fs::read_dir("/proc").ok()?;
    for entry in proc_dir.filter_map(|e| e.ok()) {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if !name_str.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let comm_path = entry.path().join("comm");
        if let Ok(comm) = fs::read_to_string(comm_path) {
            let comm = comm.trim();
            for &wm in &known_wms {
                if comm.eq_ignore_ascii_case(wm) {
                    return Some(wm.to_string());
                }
            }
        }
    }
    None
}

fn get_shell() -> String {
    if let Ok(val) = std::env::var("SHELL") {
        let clean = val.trim();
        if !clean.is_empty() {
            let name = clean.split('/').last().unwrap_or(clean);
            return name.to_string();
        }
    }

    // Fallback: check parent process comm
    if let Ok(stat) = fs::read_to_string("/proc/self/stat") {
        let fields: Vec<&str> = stat.split_whitespace().collect();
        if fields.len() > 3 {
            let ppid = fields[3];
            if let Ok(comm) = fs::read_to_string(format!("/proc/{}/comm", ppid)) {
                let clean = comm.trim();
                if !clean.is_empty() {
                    return clean.to_string();
                }
            }
        }
    }

    "unknown".to_string()
}

fn get_memory() -> String {
    let content = match fs::read_to_string("/proc/meminfo") {
        Ok(c) => c,
        Err(_) => return "unknown".to_string(),
    };

    let mut mem_total_kb: Option<u64> = None;
    let mut mem_avail_kb: Option<u64> = None;
    let mut mem_free_kb: Option<u64> = None;
    let mut buffers_kb: Option<u64> = None;
    let mut cached_kb: Option<u64> = None;
    let mut sreclaimable_kb: Option<u64> = None;
    let mut shmem_kb: Option<u64> = None;

    for line in content.lines() {
        if let Some((k, v)) = line.split_once(':') {
            let k = k.trim();
            let v_num = v.split_whitespace().next().and_then(|n| n.parse::<u64>().ok());
            match k {
                "MemTotal" => mem_total_kb = v_num,
                "MemAvailable" => mem_avail_kb = v_num,
                "MemFree" => mem_free_kb = v_num,
                "Buffers" => buffers_kb = v_num,
                "Cached" => cached_kb = v_num,
                "SReclaimable" => sreclaimable_kb = v_num,
                "Shmem" => shmem_kb = v_num,
                _ => {}
            }
        }
    }

    let total_kb = match mem_total_kb {
        Some(t) if t > 0 => t,
        _ => return "unknown".to_string(),
    };

    let avail_kb = mem_avail_kb.unwrap_or_else(|| {
        let free = mem_free_kb.unwrap_or(0);
        let buffers = buffers_kb.unwrap_or(0);
        let cached = cached_kb.unwrap_or(0);
        let srec = sreclaimable_kb.unwrap_or(0);
        let shmem = shmem_kb.unwrap_or(0);
        free + buffers + cached + srec.saturating_sub(shmem)
    });

    let used_kb = total_kb.saturating_sub(avail_kb);

    let total_gib = total_kb as f64 / 1_048_576.0;
    let used_gib = used_kb as f64 / 1_048_576.0;
    let pct = (used_kb as f64 / total_kb as f64) * 100.0;

    format!("{:.2} GiB / {:.2} GiB ({:.0}%)", used_gib, total_gib, pct)
}

fn get_disk() -> String {
    let path = match CString::new("/") {
        Ok(p) => p,
        Err(_) => return "unknown".to_string(),
    };

    let mut stat = MaybeUninit::<libc::statvfs>::uninit();
    let res = unsafe { libc::statvfs(path.as_ptr(), stat.as_mut_ptr()) };
    if res != 0 {
        return "unknown".to_string();
    }

    let stat = unsafe { stat.assume_init() };
    let frsize = stat.f_frsize as u64;
    let total_bytes = (stat.f_blocks as u64) * frsize;
    let free_bytes = (stat.f_bfree as u64) * frsize;

    if total_bytes == 0 {
        return "unknown".to_string();
    }

    let used_bytes = total_bytes.saturating_sub(free_bytes);

    let total_gib = total_bytes as f64 / 1_073_741_824.0;
    let used_gib = used_bytes as f64 / 1_073_741_824.0;
    let pct = (used_bytes as f64 / total_bytes as f64) * 100.0;

    format!("{:.1} GiB / {:.1} GiB ({:.0}%)", used_gib, total_gib, pct)
}

fn get_packages() -> String {
    let mut parts = Vec::new();

    // 1. Pacman
    if let Ok(entries) = fs::read_dir("/var/lib/pacman/local") {
        let count = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .count();
        if count > 0 {
            parts.push(format!("{} (pacman)", count));
        }
    }

    // 2. Dpkg
    if let Ok(content) = fs::read("/var/lib/dpkg/status") {
        let count = content.windows(9).filter(|w| *w == b"\nPackage:").count()
            + if content.starts_with(b"Package:") { 1 } else { 0 };
        if count > 0 {
            parts.push(format!("{} (dpkg)", count));
        }
    }

    // 3. Flatpak
    let mut flatpak_count = 0;
    if let Ok(entries) = fs::read_dir("/var/lib/flatpak/app") {
        flatpak_count += entries.filter_map(|e| e.ok()).filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false)).count();
    }
    if let Ok(home) = std::env::var("HOME") {
        let user_flatpak = Path::new(&home).join(".local/share/flatpak/app");
        if let Ok(entries) = fs::read_dir(user_flatpak) {
            flatpak_count += entries.filter_map(|e| e.ok()).filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false)).count();
        }
    }
    if flatpak_count > 0 {
        parts.push(format!("{} (flatpak)", flatpak_count));
    }

    // 4. Snap
    if let Ok(entries) = fs::read_dir("/var/lib/snapd/snaps") {
        let count = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|ext| ext == "snap").unwrap_or(false))
            .count();
        if count > 0 {
            parts.push(format!("{} (snap)", count));
        }
    }

    // 5. Alpine APK
    if let Ok(content) = fs::read("/lib/apk/db/installed") {
        let count = content.windows(3).filter(|w| *w == b"\nP:").count()
            + if content.starts_with(b"P:") { 1 } else { 0 };
        if count > 0 {
            parts.push(format!("{} (apk)", count));
        }
    }

    // 6. Homebrew
    for brew_path in ["/home/linuxbrew/.linuxbrew/Cellar", "/opt/homebrew/Cellar"] {
        if let Ok(entries) = fs::read_dir(brew_path) {
            let count = entries.filter_map(|e| e.ok()).filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false)).count();
            if count > 0 {
                parts.push(format!("{} (brew)", count));
                break;
            }
        }
    }

    // 7. Nix (user profile)
    if let Ok(home) = std::env::var("HOME") {
        let nix_bin = Path::new(&home).join(".nix-profile/bin");
        if let Ok(entries) = fs::read_dir(nix_bin) {
            let count = entries.filter_map(|e| e.ok()).count();
            if count > 0 {
                parts.push(format!("{} (nix)", count));
            }
        }
    }

    // 8. XBPS
    if let Ok(entries) = fs::read_dir("/var/db/xbps") {
        let count = entries.filter_map(|e| e.ok()).filter(|e| e.file_name().to_string_lossy().starts_with("pkgdb")).count();
        if count > 0 {
            parts.push(format!("{} (xbps)", count));
        }
    }

    if parts.is_empty() {
        "unknown".to_string()
    } else {
        parts.join(", ")
    }
}
