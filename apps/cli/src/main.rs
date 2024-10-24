use clap::Command;
use daemonize::Daemonize;
use std::env;
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
    let response = client
        .post(endpoint)
        .json(&json!({
            "platform": "desktop",
            "title": active_window.info.name,
            "url": active_window.info.path,
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
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
    std::fs::read_to_string("/tmp/active-window-monitor.pid")?
        .trim()
        .parse()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

fn is_running() -> bool {
    read_pid().is_ok()
}

static RUNNING: AtomicBool = AtomicBool::new(true);

async fn run_periodic_task() {
    let mut interval = time::interval(Duration::from_secs(300));
    let client = Client::new();
    let endpoint = "https://affectionate-compassion-production.up.railway.app";

    let mut term_signal = signal_hook_tokio::Signals::new(&[
        signal_hook::consts::SIGTERM,
        signal_hook::consts::SIGINT,
    ])
    .unwrap();

    let handle = term_signal.handle();
    let signal_task = tokio::spawn(async move {
        use futures::StreamExt;
        term_signal.next().await;
        RUNNING.store(false, Ordering::SeqCst);
    });

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
            else => {
                println!("Shutdown signal received");
                break;
            }
        }
    }

    // Cleanup
    handle.close();
    signal_task.abort();
    println!("Daemon shutting down gracefully");
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    #[cfg(target_os = "macos")]
    env::set_var("OBJC_DISABLE_INITIALIZE_FORK_SAFETY", "YES");

    runtime.block_on(async {
        let matches = cli().get_matches();

        match matches.subcommand() {
            Some(("start", _)) => {
                if is_running() {
                    println!("daemon is already running");
                    return;
                }

                let log_file = match std::fs::File::create("/tmp/active-window-monitor.log") {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("failed to create log file: {:?}", e);
                        return;
                    }
                };
                let err_file = match std::fs::File::create("/tmp/active-window-monitor.err") {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("failed to create error file: {:?}", e);
                        return;
                    }
                };

                let daemonize = Daemonize::new()
                    .pid_file("/tmp/active-window-monitor.pid")
                    .chown_pid_file(true)
                    .working_directory("/tmp")
                    .stdout(log_file)
                    .stderr(err_file);

                println!("daemon started");

                match daemonize.start() {
                    Ok(_) => {
                        println!("daemonized");

                        run_periodic_task().await;
                    }
                    Err(e) => eprintln!("daemonize error: {:?}", e),
                }
            }
            Some(("stop", _)) => {
                if let Ok(pid) = read_pid() {
                    unsafe {
                        libc::kill(pid, libc::SIGTERM);
                    }
                    let _ = std::fs::remove_file("/tmp/active-window-monitor.pid");

                    let mut attempts = 0;
                    while attempts < 10 {
                        if !is_running() {
                            break;
                        }
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        attempts += 1;
                    }

                    let _ = std::fs::remove_file("/tmp/active-window-monitor.log");
                    let _ = std::fs::remove_file("/tmp/active-window-monitor.err");
                    let _ = std::fs::remove_file("/tmp/active-window-monitor.pid");

                    println!("daemon stopped");
                } else {
                    println!("daemon is not running");
                }
            }
            Some(("logs", _)) => {
                println!("--------------------------------");
                match std::fs::read_to_string("/tmp/active-window-monitor.log") {
                    Ok(content) => {
                        content.lines().for_each(|line| println!("{}", line));
                    }
                    Err(_) => {
                        println!("log file doesn't exist");
                    }
                }

                println!("--------------------------------");
                match std::fs::read_to_string("/tmp/active-window-monitor.err") {
                    Ok(content) => {
                        content.lines().for_each(|line| println!("{}", line));
                    }
                    Err(_) => {
                        println!("error file doesn't exist");
                    }
                }
            }
            _ => unreachable!(),
        }
    })
}
