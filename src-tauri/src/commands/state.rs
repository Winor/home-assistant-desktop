use crate::app::state::{AppState, Status};
use tauri::State;

#[tauri::command]
pub async fn get_status(store: State<'_, AppState>) -> Result<Status, ()> {
    let state = store.lock().await;
    let status = state.status.clone();
    Ok(status)
}