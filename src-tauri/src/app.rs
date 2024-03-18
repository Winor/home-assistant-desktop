pub mod action {
    use tauri::{AppHandle, Manager};

    pub fn open_settings(app: &AppHandle) {
        if app.get_window("settings").is_none() {
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
    }
}
