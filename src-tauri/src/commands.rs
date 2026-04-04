use crate::store::{AppData, DataStore};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::State;

pub type StoreState = Arc<Mutex<Box<dyn DataStore>>>;

// ── Data persistence ──────────────────────────────────────────────────────────

#[tauri::command]
pub fn load_app_data(store: State<StoreState>) -> Result<String, String> {
    let s = store.lock().map_err(|e| e.to_string())?;
    let data = s.load()?;
    serde_json::to_string(&data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_app_data(json: String, store: State<StoreState>) -> Result<(), String> {
    let data: AppData = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    let s = store.lock().map_err(|e| e.to_string())?;
    s.save(&data)
}

#[tauri::command]
pub fn get_store_location(store: State<StoreState>) -> Result<String, String> {
    let s = store.lock().map_err(|e| e.to_string())?;
    Ok(s.location())
}

// ── File system ───────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: String, // ISO 8601
}

#[tauri::command]
pub async fn list_dir(path: String) -> Result<Vec<FileEntry>, String> {
    use chrono::{DateTime, Utc};
    use std::time::SystemTime;

    let mut entries = tokio::fs::read_dir(&path)
        .await
        .map_err(|e| e.to_string())?;

    let mut result: Vec<FileEntry> = Vec::new();

    while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
        let name = entry.file_name().to_string_lossy().to_string();
        // Skip hidden / system entries
        if name.starts_with('.') || name == "$RECYCLE.BIN" || name == "System Volume Information" {
            continue;
        }

        let meta = entry.metadata().await.map_err(|e| e.to_string())?;
        let modified = meta
            .modified()
            .ok()
            .and_then(|t| {
                let dt: DateTime<Utc> = t.into();
                Some(dt.to_rfc3339())
            })
            .unwrap_or_default();

        result.push(FileEntry {
            name,
            path: entry.path().to_string_lossy().to_string(),
            is_dir: meta.is_dir(),
            size: if meta.is_file() { meta.len() } else { 0 },
            modified,
        });
    }

    // Directories first, then alphabetical
    result.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.to_lowercase().cmp(&b.name.to_lowercase())));

    Ok(result)
}

#[tauri::command]
pub async fn read_text_file(path: String) -> Result<String, String> {
    // Limit reads to 2 MB to avoid freezing the UI
    const MAX_BYTES: u64 = 2 * 1024 * 1024;
    let meta = tokio::fs::metadata(&path).await.map_err(|e| e.to_string())?;
    if meta.len() > MAX_BYTES {
        return Err(format!("File too large to preview ({:.1} MB)", meta.len() as f64 / 1_048_576.0));
    }
    tokio::fs::read_to_string(&path).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_home_dir() -> Result<String, String> {
    dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| "Could not determine home directory".to_string())
}

// ── Open / reveal ─────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn open_path(path: String) -> Result<(), String> {
    opener::open(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_url(url: String) -> Result<(), String> {
    opener::open(&url).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reveal_path(path: String) -> Result<(), String> {
    // On Windows: explorer /select,<path>
    // On Linux:   xdg-open <parent dir>
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .args(["/select,", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        use std::path::Path;
        let parent = Path::new(&path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone());
        std::process::Command::new("xdg-open")
            .arg(&parent)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

// ── Clipboard ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn read_clipboard(app: tauri::AppHandle) -> Result<String, String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    app.clipboard()
        .read_text()
        .map_err(|e| e.to_string())
}
