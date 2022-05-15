#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::path::PathBuf;
use tauri::{App, api::path::resource_dir, Env};

static mut RESOURCE_DIR: Option<PathBuf> = None;

fn main() {
  tauri::Builder::default()
    .setup(|app| Ok(setup(app)))
    .invoke_handler(tauri::generate_handler![ping])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn setup(app: &App) {
    let resource_dir = resource_dir(app.package_info(), &Env::default()).unwrap();
    unsafe {
        RESOURCE_DIR = Some(resource_dir.clone());
    }
    unsafe {
        println!("{}", RESOURCE_DIR.clone().unwrap().to_str().unwrap());
    }
}

#[tauri::command]
fn ping(message: &str) -> Result<String, String> {
    unsafe {
        let resource_dir = RESOURCE_DIR.clone().unwrap();
        return Ok(format!("resource_dir path: {}", resource_dir.to_str().unwrap()));
    }
}
