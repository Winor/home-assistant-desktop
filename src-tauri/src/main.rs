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
use tauri::Manager;
use tokio::sync::Mutex;
use utils::config::{AppStore, Config};

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let mut path = app.path_resolver().app_config_dir().unwrap();
            let _ = fs::create_dir(&path);
            path.push("config.yaml");
            println!("{:?}", path);
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
        .manage::<Haconn>(Mutex::new(None))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
