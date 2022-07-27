#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
pub mod queue;
pub mod consumer;
pub mod device;
pub mod commands;
pub mod app;

fn main() {
    app::run()
}
