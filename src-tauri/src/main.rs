// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// mod plugins;
mod plugins;
mod utils;

use std::fs;

use plugins::{
    config::{add_instance, get_list, remove_instance},
    ha_client::{hass_connect, hass_states, Haconn},
};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu};
use tokio::sync::Mutex;
use utils::config::{AppStore, Config};

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .system_tray(
            SystemTray::new().with_menu(
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new("settings", "Settings"))
                    .add_item(CustomMenuItem::new("exit", "Exit")),
            ),
        )
        .setup(|app| {
            let mut path = app.path_resolver().app_config_dir().unwrap();
            let _ = fs::create_dir(&path);
            path.push("config.yaml");
            app.manage::<AppStore>(Mutex::new(Config::new(path)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            hass_connect,
            hass_states,
            add_instance,
            get_list,
            remove_instance
        ])
        .on_system_tray_event(|app, event| match event {
            tauri::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "settings" => {
                    let _window = tauri::WindowBuilder::new(
                        app,
                        "settings",
                        tauri::WindowUrl::App("index.html".into()),
                    )
                    .title("Home assistant desktop Settings")
                    .center()
                    .build()
                    .unwrap();
                }
                "exit" => {
                    app.exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .manage::<Haconn>(Mutex::new(None))
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
