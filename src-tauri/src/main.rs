#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod api;
pub mod consumer;
pub mod domain;
pub mod queue;
pub mod repositories;
pub mod services;
pub mod settings;

pub mod app;
pub mod commands;
pub mod events;

fn main() {
    app::run()
}
