use clap::Parser;
use x_win::{get_active_window, XWinError};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    match get_active_window() {
        Ok(active_window) => {
            println!("active window: {:#?}", active_window);
        }
        Err(XWinError) => {
            println!("error occurred while getting the active window");
        }
    }
}
