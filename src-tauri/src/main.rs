// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use rust_game_of_life::{Cell, Universe};

#[tauri::command]
fn tick_universe(universe: tauri::State<'_, GameUniverse>) -> Vec<Cell> {
    universe.0.lock().unwrap().tick()
}

struct GameUniverse(Mutex<Universe>);

fn main() {
    let universe = Mutex::from(Universe::new(64, 64));

    tauri::Builder::default()
        .manage(GameUniverse(universe))
        .invoke_handler(tauri::generate_handler![tick_universe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
