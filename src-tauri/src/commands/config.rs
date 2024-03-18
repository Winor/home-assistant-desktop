use std::{borrow::BorrowMut, collections::BTreeMap};

use tauri::State;
use uuid::Uuid;

use crate::utils::config::{AppStore, ServerParams};

#[tauri::command]
pub async fn add_instance(state: State<'_, AppStore>, server: ServerParams) -> Result<(), ()> {
    let mut config = state.lock().await;
    let config = config.borrow_mut();

    config.add(server, None);
    config.write().await;
    println!("{:?}", config);
    Ok(())
}

#[tauri::command]
pub async fn get_list(
    state: State<'_, AppStore>,
) -> Result<BTreeMap<Uuid, std::string::String>, ()> {
    let mut config = state.lock().await;
    config.read().await;
    Ok(config.list())
}

#[tauri::command]
pub async fn remove_instance(state: State<'_, AppStore>, id: Uuid) -> Result<(), ()> {
    let mut config = state.lock().await;

    config.remove(id);
    config.write().await;
    Ok(())
}
