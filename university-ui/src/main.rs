// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;

use edblock::blockchain::blockchain_core::Certificate;
use edblock::utils::generate_key;

#[tauri::command(rename_all = "snake_case")]
async fn get_address() -> String {
  generate_key("uni").unwrap().address
}

#[tauri::command(rename_all = "snake_case")]
async fn get_not_completed_certificates(url: String) -> Result<Vec<Certificate>, String> {
  api::check_certificate(&url).await.map_err(|err| err.to_string())
}

#[tauri::command(rename_all = "snake_case")]
async fn approve_certificate(url: String, certificate: Certificate) -> Result<(), ()>{
  if api::sign_certificate(&url, &certificate).await {
    Ok(())
  } else {
    Err(())
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_not_completed_certificates, approve_certificate, get_address])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}