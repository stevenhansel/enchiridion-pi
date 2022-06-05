#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs, path::PathBuf, time::Duration};
use tauri::{api::path::resource_dir, async_runtime, App, Env, Manager};
use tokio::time;

static mut RESOURCE_DIR: Option<PathBuf> = None;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            setup(app);

            async_runtime::spawn(async move {
                let mut interval = time::interval(Duration::from_millis(1000));
                loop {
                    handle
                        .emit_all("listen_media_update", "payload")
                        .expect("Error when emitting");
                    interval.tick().await;
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_images])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &App) {
    let resource_dir = resource_dir(app.package_info(), &Env::default()).unwrap();
    unsafe {
        RESOURCE_DIR = Some(resource_dir.clone());
        println!("{}", resource_dir.to_str().unwrap())
    }
}

#[tauri::command]
fn get_images() -> Result<Vec<String>, String> {
    unsafe {
        let resource_dir = RESOURCE_DIR
            .clone()
            .expect("Resource Directory is not available");
        let images_dir = &resource_dir.join("images");

        if !images_dir.exists() {
            fs::create_dir_all(images_dir).expect("Error when creating image directory");
        }

        let images = fs::read_dir(images_dir).expect("Error when reading directory");
        let res = images
            .filter_map(|entry| {
                entry
                    .ok()
                    .and_then(|e| e.path().to_str().map(|s| ["asset:///", s].concat()))
            })
            .collect::<Vec<String>>();

        return Ok(res);
    }
}
