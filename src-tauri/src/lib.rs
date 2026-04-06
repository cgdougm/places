mod commands;
mod store;

use tauri::Manager;

use commands::StoreState;
use store::{LocalJsonStore};
use std::sync::{Arc, Mutex};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            // Resolve data directory: $APPDATA/places/data.json (Windows)
            // or ~/.local/share/places/data.json (Linux)
            let data_dir = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| {
                    dirs::data_local_dir()
                        .unwrap_or_else(|| std::path::PathBuf::from("."))
                        .join("places")
                });
            let data_path = data_dir.join("data.json");
            let store: StoreState = Arc::new(Mutex::new(Box::new(LocalJsonStore::new(data_path))));
            app.manage(store);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::load_app_data,
            commands::save_app_data,
            commands::get_store_location,
            commands::list_dir,
            commands::read_text_file,
            commands::get_home_dir,
            commands::open_path,
            commands::open_url,
            commands::reveal_path,
            commands::read_binary_file,
            commands::read_clipboard,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
