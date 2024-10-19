use serde_json::json;
use x_win::{get_active_window, XWinError};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_active_window_info() -> serde_json::Value {
    match get_active_window() {
        Ok(active_window) => {
            json!({
                "title": active_window.title,
                "url": active_window.url,
                "id": active_window.id,
                "os": active_window.os,
                "info": {
                    "path": active_window.info.path,
                    "name": active_window.info.name,
                    "exec_name": active_window.info.exec_name,
                },
            })
        }
        Err(XWinError) => {
            json!({"error": "Error occurred while getting the active window"})
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri::{
        menu::{Menu, MenuItem},
        tray::TrayIconBuilder,
    };

    tauri::Builder::default()
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_active_window_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
