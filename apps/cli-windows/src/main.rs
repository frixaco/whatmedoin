use chrono::Local;
use clap::Command;
use reqwest::Client;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::time::{self, Duration};
use x_win::{get_active_window, WindowInfo};

// TODO: Confirm the names
const TRACKED_WINDOWS: [&str; 8] = [
    "WezTerm",
    "Cursor",
    "Slack",
    "Anki",
    "Heptabase",
    "osu!",
    "Blender",
    "pwsh",
];

static RUNNING: AtomicBool = AtomicBool::new(true);
const APP_NAME: &str = "wmd";

async fn send_event(
    client: &Client,
    endpoint: &str,
    active_window: &WindowInfo,
) -> Result<(), reqwest::Error> {
    let _ = client
        .post(endpoint)
        .json(&json!({
            "platform": "desktop",
            "title": active_window.info.name,
            "url": active_window.info.path,
        }))
        .send()
        .await?;

    Ok(())
}

fn get_active_window_info() -> Option<WindowInfo> {
    match get_active_window() {
        Ok(active_window) => {
            println!("active window: {:?}", active_window);
            Some(active_window)
        }
        Err(e) => {
            println!("x-win error: {:?}", e);
            None
        }
    }
}

fn cli() -> Command {
    Command::new("wmd")
        .about("whatmedoin CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("run").about("Run the window monitor in foreground"))
        .subcommand(Command::new("start").about("Start the window monitor in background"))
        .subcommand(Command::new("stop").about("Stop the running monitor"))
        .subcommand(Command::new("install").about("Install to startup"))
        .subcommand(Command::new("uninstall").about("Remove from startup"))
}

fn setup_logging() -> std::io::Result<PathBuf> {
    let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| String::from(r"C:\Temp"));
    let log_dir = PathBuf::from(local_app_data).join(APP_NAME);
    std::fs::create_dir_all(&log_dir)?;
    let log_path = log_dir.join(format!("{}.log", APP_NAME));
    Ok(log_path)
}

fn log_to_file(message: &str) {
    if let Ok(log_path) = setup_logging() {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
            if writeln!(file, "[{}] {}", timestamp, message).is_err() {
                eprintln!("Failed to write to log file");
            }
        }
    }
}

async fn run_monitor() {
    let mut interval = time::interval(Duration::from_secs(60));
    let client = Client::new();
    let endpoint = "https://affectionate-compassion-production.up.railway.app/activity";

    log_to_file("Monitor started");
    interval.tick().await;

    // Set up Ctrl+C handler
    ctrlc::set_handler(move || {
        RUNNING.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while RUNNING.load(Ordering::SeqCst) {
        tokio::select! {
            _ = interval.tick() => {
                log_to_file("Checking active window...");
                if let Some(window_info) = get_active_window_info() {
                    if TRACKED_WINDOWS.contains(&window_info.info.name.as_str()) {
                        match send_event(&client, endpoint, &window_info).await {
                            Ok(_) => log_to_file("Successfully sent window info"),
                            Err(e) => log_to_file(&format!("Failed to send window info: {:?}", e)),
                        }
                    }
                }
            }
        }
    }
    log_to_file("Monitor stopped");
}

fn install_to_startup() -> std::io::Result<()> {
    let startup_folder = if let Ok(appdata) = std::env::var("APPDATA") {
        PathBuf::from(appdata).join(r"Microsoft\Windows\Start Menu\Programs\Startup")
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find AppData directory",
        ));
    };

    let exe_path = std::env::current_exe()?;
    let shortcut_path = startup_folder.join(format!("{}.lnk", APP_NAME));

    // Create shortcut using PowerShell with WindowStyle Hidden
    let ps_command = format!(
        "$WS = New-Object -ComObject WScript.Shell; \
         $SC = $WS.CreateShortcut('{}'); \
         $SC.TargetPath = 'powershell.exe'; \
         $SC.Arguments = '-WindowStyle Hidden -Command \"\"\"Start-Process \"\"\"{}\"\"\" -ArgumentList run -WindowStyle Hidden\"\"\"'; \
         $SC.Save()",
        shortcut_path.to_string_lossy(),
        exe_path.to_string_lossy()
    );

    std::process::Command::new("powershell")
        .arg("-Command")
        .arg(ps_command)
        .output()?;

    println!("Application installed to startup folder");
    Ok(())
}

fn remove_from_startup() -> std::io::Result<()> {
    let _ = stop_monitor();

    let startup_folder = if let Ok(appdata) = std::env::var("APPDATA") {
        PathBuf::from(appdata).join(r"Microsoft\Windows\Start Menu\Programs\Startup")
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find AppData directory",
        ));
    };

    let shortcut_path = startup_folder.join(format!("{}.lnk", APP_NAME));
    if shortcut_path.exists() {
        std::fs::remove_file(shortcut_path)?;
        println!("Application removed from startup folder");
    } else {
        println!("Application was not found in startup folder");
    }
    Ok(())
}

// Add a new function to start the process in background
fn start_background() -> std::io::Result<()> {
    let exe_path = std::env::current_exe()?;

    // First check if the process is already running
    let check_running = std::process::Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "Get-Process | Where-Object {{ $_.ProcessName -eq '{}' -and $_.Id -ne {} }}",
            APP_NAME,
            std::process::id()
        ))
        .output()?;

    if !check_running.stdout.is_empty() {
        println!("Monitor is already running");
        return Ok(());
    }

    // Use Start-Process with -NoNewWindow instead of WindowStyle Hidden
    std::process::Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "Start-Process '{}' -ArgumentList run -NoNewWindow",
            exe_path.to_string_lossy()
        ))
        .spawn()?;

    println!("Application started in background");
    std::thread::sleep(std::time::Duration::from_secs(1)); // Give time for the message to be visible
    Ok(())
}

fn stop_monitor() -> std::io::Result<()> {
    // PowerShell command to find and stop the process, excluding the current process ID
    let current_pid = std::process::id();
    let ps_command = format!(
        "Get-Process | Where-Object {{ $_.MainWindowTitle -eq '' -and $_.ProcessName -eq '{}' -and $_.Id -ne {} }} | Stop-Process",
        APP_NAME,
        current_pid
    );

    let output = std::process::Command::new("powershell")
        .arg("-Command")
        .arg(ps_command)
        .output()?;

    if output.status.success() {
        println!("Monitor process stopped successfully");
    } else {
        println!("No running monitor process found");
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("run", _)) => {
            println!("Starting window monitor...");
            run_monitor().await;
        }
        Some(("start", _)) => {
            if let Err(e) = start_background() {
                eprintln!("Failed to start in background: {}", e);
            }
        }
        Some(("stop", _)) => {
            if let Err(e) = stop_monitor() {
                eprintln!("Failed to stop monitor: {}", e);
            }
        }
        Some(("install", _)) => {
            if let Err(e) = install_to_startup() {
                eprintln!("Failed to install to startup: {}", e);
            }
        }
        Some(("uninstall", _)) => {
            if let Err(e) = remove_from_startup() {
                eprintln!("Failed to remove from startup: {}", e);
            }
        }
        _ => unreachable!(),
    }
}
