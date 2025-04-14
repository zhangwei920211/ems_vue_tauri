mod https;

use https::async_routes::get_async_routes;
use https::refresh_token::refresh_token;
use https::users::login;

#[allow(dead_code)]
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn create_app() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_async_routes,
            login,
            refresh_token
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
