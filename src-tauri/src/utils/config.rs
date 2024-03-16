use std::{collections::BTreeMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::sync::Mutex;

use tauri::Url;
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub url: Url,
    pub token: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub default: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerInstance {
    pub display_name: String,
    pub server: ServerConfig,
    pub app: Option<AppConfig>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerParams {
    pub protocol: String,
    pub address: String,
    pub port: i32,
    pub token: String,
}

pub type AppStore = Mutex<Config>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub map: BTreeMap<Uuid, ServerInstance>,
    path: PathBuf,
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        Config {
            map: BTreeMap::new(),
            path: path,
        }
    }

    pub async fn write(&self) {
        let contents = serde_yaml::to_string(&self.map).unwrap();
        let _ = fs::write(&self.path, contents).await;
    }

    pub async fn read(&mut self) {
        let file = fs::read(&self.path).await.unwrap();
        let contents: BTreeMap<Uuid, ServerInstance> = serde_yaml::from_slice(&file).unwrap();
        self.map = contents;
    }

    pub fn add(&mut self, server: ServerParams, app: Option<AppConfig>) {
        let url_string = format!("{}{}:{}", server.protocol, server.address, server.port);
        let url = Url::parse(&url_string).unwrap();
        self.map.insert(
            Uuid::new_v4(),
            ServerInstance {
                display_name: format!("{}:{}", &server.address, &server.port),
                server: ServerConfig {
                    url: url,
                    token: server.token,
                },
                app: app,
            },
        );
    }

    pub fn remove(&mut self, id: Uuid) {
        self.map.remove_entry(&id);
    }

    pub fn list(&self) -> BTreeMap<Uuid, String> {
        let mut list: BTreeMap<Uuid, String> = BTreeMap::new();
        for (key, value) in self.map.iter() {
            list.insert(*key, (value.display_name).to_string());
        }
        list
    }

    // pub fn find(&self, id: Uuid) -> Option<&ServerInstance> {
    //     self.map.get(&id)
    // }
}
