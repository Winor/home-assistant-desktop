use crate::utils::{config::AppStore, hass_api::create_client};
use hass_rs::HassClient;
use hass_rs::HassEntity;
use tauri::State;
use tokio::sync::Mutex;
use uuid::Uuid;

pub type Haconn = Mutex<Option<HassClient>>;

#[tauri::command]
pub async fn hass_connect(
    ha_rs: State<'_, Haconn>,
    state: State<'_, AppStore>,
    server_id: Uuid,
) -> Result<String, String> {
    let config = state.lock().await;
    let server_instance = config.map.get(&server_id);
    if let Some(server_instance) = server_instance {
        let client = create_client(
            server_instance.server.url.clone(),
            server_instance.server.token.clone(),
        )
        .await
        .ok();
        if let Some(client) = client {
            let _set_client = ha_rs.lock().await.insert(client);
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
