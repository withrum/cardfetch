use crate::info::SystemInfo;
use unicode_width::UnicodeWidthStr;

pub const RESET: &str = "\x1b[0m";

#[derive(Clone, Copy)]
pub enum BorderColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
}

impl BorderColor {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "black" => Some(BorderColor::Black),
            "red" => Some(BorderColor::Red),
            "green" => Some(BorderColor::Green),
            "yellow" => Some(BorderColor::Yellow),
            "blue" => Some(BorderColor::Blue),
            "magenta" | "purple" => Some(BorderColor::Magenta),
            "cyan" => Some(BorderColor::Cyan),
            "white" => Some(BorderColor::White),
            "gray" | "grey" | "dim" => Some(BorderColor::Gray),
            _ => None,
        }
    }

    pub fn ansi_code(&self) -> &'static str {
        match self {
            BorderColor::Black => "\x1b[30m",
            BorderColor::Red => "\x1b[31m",
            BorderColor::Green => "\x1b[32m",
            BorderColor::Yellow => "\x1b[33m",
            BorderColor::Blue => "\x1b[34m",
            BorderColor::Magenta => "\x1b[35m",
            BorderColor::Cyan => "\x1b[36m",
            BorderColor::White => "\x1b[37m",
            BorderColor::Gray => "\x1b[90m", // Bright black / terminal gray
        }
    }
}

pub struct CardConfig {
    pub show_title: bool,
    pub border_color: BorderColor,
    pub show_bright_palette: bool,
}

impl Default for CardConfig {
    fn default() -> Self {
        CardConfig {
            show_title: true,
            border_color: BorderColor::Gray,
            show_bright_palette: false,
        }
    }
}

struct CardRow {
    icon: &'static str,
    icon_color: u8,
    label: &'static str,
    value: String,
}

pub fn render_card(info: &SystemInfo, config: &CardConfig) -> String {
    let border = config.border_color.ansi_code();

    // Prepare rows
    let rows = vec![
        CardRow {
            icon: info.distro.icon,
            icon_color: info.distro.ansi_color,
            label: "os",
            value: info.distro.name.clone(),
        },
        CardRow {
            icon: "󰒋",
            icon_color: 5, // Magenta
            label: "kernel",
            value: info.kernel.clone(),
        },
        CardRow {
            icon: "󰔟",
            icon_color: 3, // Yellow
            label: "uptime",
            value: info.uptime.clone(),
        },
        CardRow {
            icon: "󰧨",
            icon_color: 2, // Green
            label: "wm",
            value: info.wm.clone(),
        },
        CardRow {
            icon: "󰞷",
            icon_color: 6, // Cyan
            label: "shell",
            value: info.shell.clone(),
        },
        CardRow {
            icon: "󰏖",
            icon_color: 5, // Magenta
            label: "pkgs",
            value: info.packages.clone(),
        },
        CardRow {
            icon: "󰍛",
            icon_color: 3, // Yellow
            label: "memory",
            value: info.memory.clone(),
        },
        CardRow {
            icon: "󰋊",
            icon_color: 4, // Blue
            label: "disk",
            value: info.disk.clone(),
        },
    ];

    let label_col_width = 7; // Fixed label width for clean alignment

    // Calculate maximum content width
    // Structure of each line:
    // icon (width) + "  " (2) + label (width 7) + "  " (2) + value (width)
    let max_row_width = rows
        .iter()
        .map(|r| r.icon.width() + 2 + label_col_width + 2 + r.value.width())
        .max()
        .unwrap_or(30);

    let title_plain = format!("{}@{}", info.user, info.host);
    let title_width = title_plain.width();

    let dots_width = 15; // 8 dots + 7 spaces = 15 columns

    let min_inner_width = 38;
    let mut inner_width = max_row_width.max(dots_width).max(min_inner_width);
    if config.show_title {
        inner_width = inner_width.max(title_width);
    }

    let mut out = String::new();

    // Top border: ╭─────────────────────────╮
    out.push_str(border);
    out.push('╭');
    out.push_str(&"─".repeat(inner_width + 4));
    out.push('╮');
    out.push_str(RESET);
    out.push('\n');

    // Title line: │  user@host  │
    if config.show_title {
        let title_colored = format!(
            "\x1b[1;32m{}\x1b[0m\x1b[90m@\x1b[0m\x1b[1;36m{}\x1b[0m",
            info.user, info.host
        );
        let title_pad = inner_width - title_width;
        out.push_str(border);
        out.push('│');
        out.push_str(RESET);
        out.push_str("  ");
        out.push_str(&title_colored);
        out.push_str(&" ".repeat(title_pad));
        out.push_str("  ");
        out.push_str(border);
        out.push('│');
        out.push_str(RESET);
        out.push('\n');

        // Separator: ├─────────────────────────┤
        out.push_str(border);
        out.push('├');
        out.push_str(&"─".repeat(inner_width + 4));
        out.push('┤');
        out.push_str(RESET);
        out.push('\n');
    }

    // Rows
    for row in &rows {
        let row_content_width = row.icon.width() + 2 + label_col_width + 2 + row.value.width();
        let row_pad = inner_width.saturating_sub(row_content_width);

        let icon_colored = format!("\x1b[{}m{}\x1b[0m", 30 + (row.icon_color % 8), row.icon);
        let label_pad_count = label_col_width.saturating_sub(row.label.width());
        let label_padded = format!(
            "\x1b[1m{}\x1b[0m{}",
            row.label,
            " ".repeat(label_pad_count)
        );

        out.push_str(border);
        out.push('│');
        out.push_str(RESET);
        out.push_str("  ");
        out.push_str(&icon_colored);
        out.push_str("  ");
        out.push_str(&label_padded);
        out.push_str("  ");
        out.push_str(&row.value);
        out.push_str(&" ".repeat(row_pad));
        out.push_str("  ");
        out.push_str(border);
        out.push('│');
        out.push_str(RESET);
        out.push('\n');
    }

    // Empty padding row before palette dots
    out.push_str(border);
    out.push('│');
    out.push_str(RESET);
    out.push_str(&" ".repeat(inner_width + 4));
    out.push_str(border);
    out.push('│');
    out.push_str(RESET);
    out.push('\n');

    // System color palette dots (ANSI 0-7)
    let mut dots_str = String::new();
    for i in 0..8 {
        dots_str.push_str(&format!("\x1b[{}m●\x1b[0m", 30 + i));
        if i < 7 {
            dots_str.push(' ');
        }
    }
    let dots_left_pad = (inner_width - dots_width) / 2;
    let dots_right_pad = inner_width - dots_width - dots_left_pad;

    out.push_str(border);
    out.push('│');
    out.push_str(RESET);
    out.push_str("  ");
    out.push_str(&" ".repeat(dots_left_pad));
    out.push_str(&dots_str);
    out.push_str(&" ".repeat(dots_right_pad));
    out.push_str("  ");
    out.push_str(border);
    out.push('│');
    out.push_str(RESET);
    out.push('\n');

    // Optional bright palette row (ANSI 8-15)
    if config.show_bright_palette {
        let mut bright_dots = String::new();
        for i in 0..8 {
            bright_dots.push_str(&format!("\x1b[{}m●\x1b[0m", 90 + i));
            if i < 7 {
                bright_dots.push(' ');
            }
        }
        out.push_str(border);
        out.push('│');
        out.push_str(RESET);
        out.push_str("  ");
        out.push_str(&" ".repeat(dots_left_pad));
        out.push_str(&bright_dots);
        out.push_str(&" ".repeat(dots_right_pad));
        out.push_str("  ");
        out.push_str(border);
        out.push('│');
        out.push_str(RESET);
        out.push('\n');
    }

    // Bottom border: ╰─────────────────────────╯
    out.push_str(border);
    out.push('╰');
    out.push_str(&"─".repeat(inner_width + 4));
    out.push('╯');
    out.push_str(RESET);
    out.push('\n');

    out
}
