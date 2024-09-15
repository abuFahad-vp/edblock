// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use edblock::utils::get_certificates;

#[tauri::command(rename_all = "snake_case")]
async fn get_status(url: String, certificate_id: String, student_address: String, university_address: String) -> Result<(),()> {
    if let Ok(certificates) = get_certificates(&url, &student_address).await {
      for cert in certificates {
        if cert.status == "completed" && cert.certificate_id == certificate_id && cert.uni_wallet_addr == university_address {
          return Ok(())
        }
      }
    }
    return Err(())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_status])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}