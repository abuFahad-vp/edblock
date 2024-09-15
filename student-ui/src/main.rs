// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use edblock::{blockchain::blockchain_core::Certificate, generate_key};

pub mod api;

#[tauri::command(rename_all = "snake_case")]
async fn upload_certificate(trans_url: String, course_id: String, course_name: String, uni_addr: String) -> Result<(), ()>{
  let status = api::sent_transaction(trans_url, course_id, course_name, uni_addr).await;
  if status {
    Ok(())
  }else {
    Err(())
  }
}

#[tauri::command(rename_all = "snake_case")]
async fn get_address() -> String {
  generate_key::generate_key("stu").wallet_address
}

#[tauri::command(rename_all = "snake_case")]
async fn get_certificates(url: String) -> Result<Vec<Certificate>, String> {
  api::check_certificate(&url).await.map_err(|err| err.to_string())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![upload_certificate, get_certificates, get_address])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}