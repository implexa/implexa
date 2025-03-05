// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager};
use std::sync::Mutex;

// Define a simple state struct for our application
struct AppState {
    counter: Mutex<i32>,
}

// Define a command to increment the counter
#[tauri::command]
fn increment_counter(state: tauri::State<AppState>) -> Result<i32, String> {
    let mut counter = state.counter.lock().map_err(|_| "Failed to lock counter".to_string())?;
    *counter += 1;
    Ok(*counter)
}

// Define a command to get the current counter value
#[tauri::command]
fn get_counter(state: tauri::State<AppState>) -> Result<i32, String> {
    let counter = state.counter.lock().map_err(|_| "Failed to lock counter".to_string())?;
    Ok(*counter)
}

fn main() {
    // Initialize logging
    env_logger::init();

    // Build the Tauri application
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            // Initialize the application state
            app.manage(AppState {
                counter: Mutex::new(0),
            });
            
            // Log that the application has started
            log::info!("Implexa application started");
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            increment_counter,
            get_counter
        ])
        .run(context)
        .expect("Error while running Implexa application");
}