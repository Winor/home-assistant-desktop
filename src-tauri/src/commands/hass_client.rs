use crate::app::state::AppState;
use crate::utils::hass_api::create_client;
use hass_rs::HassClient;
use hass_rs::HassEntity;
use tauri::State;
use tokio::sync::Mutex;
use uuid::Uuid;

pub type Haconn = Mutex<Option<HassClient>>;

#[tauri::command]
pub async fn hass_connect(
    hass_client: State<'_, Haconn>,
    store: State<'_, AppState>,
    server_id: Uuid,
) -> Result<String, String> {
    let state = store.lock().await;
    let server_instance = state.config.map.get(&server_id);
    if let Some(server_instance) = server_instance {
        let client = create_client(
            server_instance.server.url.clone(),
            server_instance.server.token.clone(),
        )
        .await
        .ok();
        if let Some(client) = client {
            let _set_client = hass_client.lock().await.insert(client);
            return Ok("connected".to_string());
        }
    }
    Err("error".to_string())
}

#[tauri::command]
pub async fn hass_states(state: State<'_, Haconn>) -> Result<Vec<HassEntity>, ()> {
    let mut client = state.lock().await;
    let client = client.as_mut();

    if let Some(client) = client {
        let states = client.get_states().await.ok();
        if let Some(states) = states {
            return Ok(states);
        }
    }

    Err(())
}
