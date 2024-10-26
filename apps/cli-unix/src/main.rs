use clap::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::time::{self, Duration};

use reqwest::Client;
use serde_json::json;
use x_win::{get_active_window, WindowInfo};

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
        .subcommand(Command::new("start").about("Starts daemon"))
        .subcommand(Command::new("stop").about("Stops daemon"))
        .subcommand(Command::new("logs").about("Shows daemon logs"))
}

fn read_pid() -> Result<i32, std::io::Error> {
    std::fs::read_to_string("/tmp/wmd.pid")?
        .trim()
        .parse()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

fn is_running() -> bool {
    read_pid().is_ok()
}

static RUNNING: AtomicBool = AtomicBool::new(true);

async fn run_periodic_task() {
    use futures::StreamExt;

    let mut interval = time::interval(Duration::from_secs(10));
    let client = Client::new();
    let endpoint = "https://affectionate-compassion-production.up.railway.app/activity";

    let mut signals = signal_hook_tokio::Signals::new(&[
        signal_hook::consts::SIGTERM,
        signal_hook::consts::SIGINT,
    ])
    .unwrap();
    let handle = signals.handle();

    interval.tick().await;

    while RUNNING.load(Ordering::SeqCst) {
        tokio::select! {
            _ = interval.tick() => {
                println!("Checking active window...");
                if let Some(window_info) = get_active_window_info() {
                    match send_event(&client, endpoint, &window_info).await {
                        Ok(_) => println!("Successfully sent window info"),
                        Err(e) => eprintln!("Failed to send window info: {:?}", e),
                    }
                }
            }
            Some(_) = signals.next() => {
                println!("Shutdown signal received");
                RUNNING.store(false, Ordering::SeqCst);
                break;
            }
        }
    }

    handle.close();
    println!("Daemon shutting down gracefully");
}

async fn start_daemon(log_file: std::fs::File) -> std::io::Result<()> {
    // Instead of using the daemonize crate, we'll just spawn a new process
    let current_exe = std::env::current_exe()?;

    std::process::Command::new(current_exe)
        .arg("daemon-worker")
        .stdout(log_file.try_clone()?)  // Clone the file handle for stdout
        .stderr(log_file)               // Use original handle for stderr
        .spawn()?;

    Ok(())
}

fn main() {
    if std::env::args().any(|arg| arg == "daemon-worker") {
        // We're in the daemon process
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let pid = std::process::id();
            std::fs::write("/tmp/wmd.pid", pid.to_string()).unwrap();
            run_periodic_task().await;
        });
        return;
    }

    // For CLI commands
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let matches = cli().get_matches();

        match matches.subcommand() {
            Some(("start", _)) => {
                if is_running() {
                    println!("daemon is already running");
                    return;
                }

                let log_file = match std::fs::File::create("/tmp/wmd.log") {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("failed to create log file: {:?}", e);
                        return;
                    }
                };

                if let Err(e) = start_daemon(log_file).await {
                    eprintln!("failed to start daemon: {:?}", e);
                    return;
                }
                println!("daemon started");
            }
            Some(("stop", _)) => {
                if let Ok(pid) = read_pid() {
                    unsafe {
                        libc::kill(pid, libc::SIGTERM);
                    }
                    let _ = std::fs::remove_file("/tmp/wmd.pid");

                    let mut attempts = 0;
                    while attempts < 10 {
                        if !is_running() {
                            break;
                        }
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        attempts += 1;
                    }

                    let _ = std::fs::remove_file("/tmp/wmd.log");
                    let _ = std::fs::remove_file("/tmp/wmd.pid");

                    println!("daemon stopped");
                } else {
                    println!("daemon is not running");
                }
            }
            Some(("logs", _)) => {
                match std::fs::read_to_string("/tmp/wmd.pid") {
                    Ok(pid) => println!("daemon pid: {}", pid),
                    Err(_) => println!("pid file doesn't exist"),
                }

                println!("--------------------------------");
                match std::fs::read_to_string("/tmp/wmd.log") {
                    Ok(content) => {
                        content.lines().for_each(|line| println!("{}", line));
                    }
                    Err(_) => {
                        println!("log file doesn't exist");
                    }
                }
            }
            _ => unreachable!(),
        }
    });
}
