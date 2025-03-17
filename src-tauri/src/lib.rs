use std::{fs::File, io::{Read, Write}, path::Path};
use remotefs::{RemoteFs, fs::UnixPex};
use remotefs_smb::{SmbFs, SmbOptions, SmbCredentials};
use serde::{Deserialize, Serialize};
use sqlite::ffi::sqlite3;
use tauri::utils::config::CapabilityEntry;
mod config;
mod error;
use crate::error::{Error, ErrorType};
use crate::config::load_config;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![load_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
