use chrono::Local;
use clap::Command;
use reqwest::Client;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::time::{self, Duration};
use x_win::{get_active_window, WindowInfo};

// TODO: Confirm the names
const TRACKED_WINDOWS: [&str; 8] = [
    "wezterm-gui",
    "Cursor",
    "Slack",
    "anki",
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
            log_to_file(&format!("active window: {:?}", active_window));
            Some(active_window)
        }
        Err(e) => {
            log_to_file(&format!("x-win error: {:?}", e));
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
        .subcommand(Command::new("logs").about("Show logs"))
}

fn setup_logging() -> std::io::Result<PathBuf> {
    let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| String::from(r"C:\Temp"));
    let log_dir = PathBuf::from(local_app_data).join(APP_NAME);
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir)?;
    }
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
    let mut interval = time::interval(Duration::from_secs(60 * 15));
    let client = Client::new();
    let endpoint = "https://api.whatmedoin.frixaco.com/activity";

    log_to_file("Monitor started");
    interval.tick().await;

    let ctrl_c = tokio::signal::ctrl_c();
    tokio::pin!(ctrl_c);

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
            _ = &mut ctrl_c => {
                println!("Received Ctrl+C, shutting down...");
                log_to_file("Received Ctrl+C, shutting down...");
                RUNNING.store(false, Ordering::SeqCst);
                break;
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

    let ps_command = format!(
        "$WS = New-Object -ComObject WScript.Shell; \
         $SC = $WS.CreateShortcut('{}'); \
         $SC.TargetPath = 'powershell.exe'; \
         $SC.Arguments = '-WindowStyle Hidden -Command \"\"\"$proc = Start-Process \"\"\"{}\"\"\" -ArgumentList run -WindowStyle Hidden -PassThru; $null = [System.Runtime.Interopservices.Marshal]::ReleaseComObject($proc)\"\"\"'; \
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

fn remove_from_startup_and_cleanup() -> std::io::Result<()> {
    let _ = stop();

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

    if let Ok(log_path) = setup_logging() {
        if log_path.exists() {
            std::fs::remove_file(&log_path)?;
            println!("Log file removed");
        }
    }

    Ok(())
}

fn is_running() -> bool {
    let current_pid = std::process::id();
    let check_running = std::process::Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "Get-Process | Where-Object {{ $_.ProcessName -eq '{}' -and $_.Id -ne {} }} | Select-Object -First 1",
            APP_NAME,
            current_pid
        ))
        .output();

    match check_running {
        Ok(output) => {
            let running = !output.stdout.is_empty();
            if running {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            running
        }
        Err(_) => {
            eprintln!("Failed to check if monitor is running");
            false
        }
    }
}

fn start() -> std::io::Result<()> {
    let exe_path = std::env::current_exe()?;

    if is_running() {
        println!("Monitor is already running");
        return Ok(());
    }

    // Create a detached process using PowerShell
    std::process::Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "$proc = Start-Process '{}' -ArgumentList run -WindowStyle Hidden -PassThru; \
             $null = [System.Runtime.Interopservices.Marshal]::ReleaseComObject($proc)",
            exe_path.to_string_lossy()
        ))
        .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
        .spawn()?;

    println!("Application started in background");
    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}

fn stop() -> std::io::Result<()> {
    let current_pid = std::process::id();
    let ps_command = format!(
        "Get-Process | Where-Object {{ $_.ProcessName -eq '{}' -and $_.Id -ne {} }} | Stop-Process -Force",
        APP_NAME,
        current_pid
    );

    let output = std::process::Command::new("powershell")
        .arg("-Command")
        .arg(ps_command)
        .output()?;

    std::thread::sleep(std::time::Duration::from_secs(1));

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
            if let Err(e) = start() {
                eprintln!("Failed to start in background: {}", e);
            }
        }
        Some(("stop", _)) => {
            if let Err(e) = stop() {
                eprintln!("Failed to stop monitor: {}", e);
            }
        }
        Some(("install", _)) => {
            if let Err(e) = install_to_startup() {
                eprintln!("Failed to install to startup: {}", e);
            }
        }
        Some(("uninstall", _)) => {
            if let Err(e) = remove_from_startup_and_cleanup() {
                eprintln!("Failed to remove from startup: {}", e);
            }
        }
        Some(("logs", _)) => {
            match std::fs::read_to_string(setup_logging().unwrap()) {
                Ok(content) => {
                    content.lines().for_each(|line| println!("{}", line));
                }
                Err(_) => {
                    println!("log file doesn't exist");
                }
            }
            println!("--------------------------------");
        }
        _ => unreachable!(),
    }
}
