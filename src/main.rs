mod card;
mod info;

use card::{render_card, BorderColor, CardConfig};
use info::SystemInfo;
use std::env;

fn print_help() {
    println!(
        "\x1b[1mcardfetch\x1b[0m - Minimalist & lightning-fast system info card

\x1b[1mUSAGE:\x1b[0m
    cardfetch [OPTIONS]

\x1b[1mOPTIONS:\x1b[0m
    \x1b[36m-b, --border <color>\x1b[0m   Card border color
                           [options: gray, blue, cyan, magenta, green, yellow, red, white]
    \x1b[36m-t, --no-title\x1b[0m         Hide user@host title header
    \x1b[36m-p, --palette-all\x1b[0m      Show both normal (0-7) and bright (8-15) palette dots
    \x1b[36m-h, --help\x1b[0m             Print this help message
    \x1b[36m-v, --version\x1b[0m          Print version information
"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = CardConfig::default();

    // Check environment variable for border color override
    if let Ok(val) = env::var("CARDFETCH_BORDER") {
        if let Some(color) = BorderColor::from_str(&val) {
            config.border_color = color;
        }
    }

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-v" | "--version" => {
                println!("cardfetch {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "-t" | "--no-title" => {
                config.show_title = false;
            }
            "-p" | "--palette-all" => {
                config.show_bright_palette = true;
            }
            "-b" | "--border" => {
                if i + 1 < args.len() {
                    i += 1;
                    if let Some(color) = BorderColor::from_str(&args[i]) {
                        config.border_color = color;
                    }
                }
            }
            arg if arg.starts_with("--border=") => {
                if let Some(color_str) = arg.strip_prefix("--border=") {
                    if let Some(color) = BorderColor::from_str(color_str) {
                        config.border_color = color;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }

    let info = SystemInfo::collect();
    let card = render_card(&info, &config);
    print!("{}", card);
}
