/// DataStore trait — swap any implementation in lib.rs without touching the rest of the app.
///
/// Current implementations:
///   LocalJsonStore  — JSON file on disk (default)
///
/// Planned (stub) implementations:
///   GoogleDriveStore — stores data.json in Google Drive AppData folder
///   DropboxStore     — stores data.json in Dropbox /Apps/Places/
///   SqliteStore      — local SQLite database (for offline-first with future sync)
///
/// Migration between stores is handled by the MigrationManager (see migrate route).
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ── Domain types (mirror src/lib/types.ts) ────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceItem {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String, // "file" | "url" | "text"
    pub label: String,
    pub value: String,
    pub notes: String,
    pub created: i64, // unix ms
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: String,
    pub items: Vec<PlaceItem>,
    pub created: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppData {
    pub projects: Vec<Project>,
    pub version: u32,
}

impl AppData {
    pub fn versioned() -> Self {
        AppData { projects: Vec::new(), version: 1 }
    }
}

// ── StoreMeta — info about a configured store ─────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreMeta {
    pub kind: String,      // "local_json" | "google_drive" | "dropbox" | "sqlite"
    pub location: String,  // Human-readable path or account description
}

// ── DataStore trait ───────────────────────────────────────────────────────────

pub trait DataStore: Send + Sync {
    fn load(&self) -> Result<AppData, String>;
    fn save(&self, data: &AppData) -> Result<(), String>;
    fn meta(&self) -> StoreMeta;
}

// ── LocalJsonStore ────────────────────────────────────────────────────────────

pub struct LocalJsonStore {
    path: PathBuf,
}

impl LocalJsonStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl DataStore for LocalJsonStore {
    fn load(&self) -> Result<AppData, String> {
        if !self.path.exists() {
            return Ok(AppData::versioned());
        }
        let raw = std::fs::read_to_string(&self.path).map_err(|e| e.to_string())?;
        serde_json::from_str(&raw).map_err(|e| e.to_string())
    }

    fn save(&self, data: &AppData) -> Result<(), String> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
        std::fs::write(&self.path, json).map_err(|e| e.to_string())
    }

    fn meta(&self) -> StoreMeta {
        StoreMeta {
            kind: "local_json".into(),
            location: self.path.display().to_string(),
        }
    }
}

// ── GoogleDriveStore (stub) ───────────────────────────────────────────────────
//
// Stores `places-data.json` in the Google Drive hidden AppData folder via
// the Drive REST API (https://www.googleapis.com/drive/v3/files).
// Requires OAuth2 credentials stored in secure keychain.
//
// pub struct GoogleDriveStore {
//     pub account_email: String,
//     pub access_token: String,  // refreshed automatically
// }
//
// impl DataStore for GoogleDriveStore {
//     fn load(&self) -> Result<AppData, String> { todo!() }
//     fn save(&self, data: &AppData) -> Result<(), String> { todo!() }
//     fn meta(&self) -> StoreMeta {
//         StoreMeta { kind: "google_drive".into(), location: self.account_email.clone() }
//     }
// }

// ── DropboxStore (stub) ───────────────────────────────────────────────────────
//
// Stores `/Apps/Places/data.json` in Dropbox via the Dropbox API v2.
// Requires OAuth2 credentials stored in secure keychain.
//
// pub struct DropboxStore {
//     pub account_email: String,
//     pub access_token: String,
// }
//
// impl DataStore for DropboxStore {
//     fn load(&self) -> Result<AppData, String> { todo!() }
//     fn save(&self, data: &AppData) -> Result<(), String> { todo!() }
//     fn meta(&self) -> StoreMeta {
//         StoreMeta { kind: "dropbox".into(), location: self.account_email.clone() }
//     }
// }

// ── SqliteStore (stub) ────────────────────────────────────────────────────────
//
// Local SQLite database — better for large datasets, full-text search, etc.
//
// pub struct SqliteStore { pub path: PathBuf }
//
// impl DataStore for SqliteStore {
//     fn load(&self) -> Result<AppData, String> { todo!() }
//     fn save(&self, data: &AppData) -> Result<(), String> { todo!() }
//     fn meta(&self) -> StoreMeta {
//         StoreMeta { kind: "sqlite".into(), location: self.path.display().to_string() }
//     }
// }
