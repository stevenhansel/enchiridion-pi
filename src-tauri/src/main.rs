#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod announcement;
pub mod queue;
pub mod api;
pub mod util;
pub mod config;

pub mod app;
pub mod commands;
pub mod events;

fn main() {
    app::run()
}
