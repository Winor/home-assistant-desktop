// use crate::utils::oauth2::create_client;
// use oauth2::url::Url;

// #[tauri::command]
// pub fn oauth(window: tauri::Window, url: String) -> Option<Url> {
//     let mut redirect = window.url().clone();
//     redirect.set_path("/login/callback");
//     let res = create_client(url, redirect.to_string()).ok();
//     let _ = window.eval(&format!("window.location.replace('{}')", res.clone()?));
//     res
// }
