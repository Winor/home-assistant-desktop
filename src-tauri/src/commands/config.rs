use std::{borrow::BorrowMut, collections::BTreeMap};

use tauri::State;
use uuid::Uuid;

use crate::{app::state::AppState, utils::config::ServerParams};

#[tauri::command]
pub async fn add_instance(store: State<'_, AppState>, server: ServerParams) -> Result<(), ()> {
    let mut state = store.lock().await;
    let config = state.config.borrow_mut();

    config.add(server, None);
    config.write().await;
    println!("{:?}", config);
    Ok(())
}

#[tauri::command]
pub async fn get_list(
    store: State<'_, AppState>,
) -> Result<BTreeMap<Uuid, std::string::String>, ()> {
    let mut state = store.lock().await;
    let config = state.config.borrow_mut();
    config.read().await;
    Ok(config.list())
}

#[tauri::command]
pub async fn remove_instance(store: State<'_, AppState>, id: Uuid) -> Result<(), ()> {
    let mut state = store.lock().await;
    let config = state.config.borrow_mut();

    config.remove(id);
    config.write().await;
    Ok(())
}
