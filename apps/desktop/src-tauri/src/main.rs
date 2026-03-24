#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use git_core::{get_status, FileStatus};

// #[tauri::command] exposes this function to the frontend via invoke()
// Return type must be serializable — we map our GitError to a String so Tauri can send it as JSON
#[tauri::command]
fn git_status(repo_path: String) -> Result<Vec<FileStatus>, String> {
    get_status(&repo_path).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        // Register commands here so the frontend can call them
        .invoke_handler(tauri::generate_handler![git_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
