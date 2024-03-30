pub mod action {
    use tauri::{AppHandle, Manager};

    pub fn open_settings(app: &AppHandle) {
        if app.get_window("settings").is_none() {
            let _window = tauri::WindowBuilder::new(
                app,
                "settings",
                tauri::WindowUrl::App("settings".into()),
            )
            .title("Home assistant desktop Settings")
            .center()
            .build()
            .unwrap();
        }
    }

    pub fn open_spotlight(app: &AppHandle) {
        if app.get_window("spotlight").is_none() {
            let _window = tauri::WindowBuilder::new(
                app,
                "spotlight",
                tauri::WindowUrl::App("spotlight".into()),
            )
            .title("Home assistant desktop Spotlight")
            .center()
            .build()
            .unwrap();
        }
    }
}

pub mod state {
    use std::path::PathBuf;
    use tokio::sync::Mutex;
    use uuid::Uuid;
    use crate::utils::config::Config;

    pub type AppState = Mutex<State>;

    pub enum Status {
        Connected(Uuid),
        Disconnected,
    }

    pub struct State {
        pub status: Status,
        pub config_path: PathBuf,
        pub config: Config,
    }

    impl State {
        pub fn new(path: PathBuf) -> Self {
            Self { status: Status::Disconnected, config: Config::new(path.to_owned()), config_path: path.to_owned() }
        }
    }
}
