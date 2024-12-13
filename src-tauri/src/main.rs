// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


#[tauri::command]
fn addTwo(num1: i32, num2: i32) -> i32{
  let retNum: i32 = num1 + num2;
  return retNum;
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![addTwo])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}