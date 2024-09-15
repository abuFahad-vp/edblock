// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::str::FromStr;

use edblock::utils::{get_certificates, verify_signature};
use secp256k1::{ecdsa::Signature, PublicKey, Secp256k1};
use edblock::blockchain::blockchain_core::Certificate;

#[tauri::command(rename_all = "snake_case")]
async fn get_status(url: String, certificate_id: String, student_address: String, university_address: String) -> Result<Certificate,()> {
    let certificates = get_certificates(&url, &student_address).await.map_err(|_| ())?;

    for c in certificates {
        if c.status == "completed" && c.certificate_id == certificate_id && c.uni_wallet_addr == university_address {
            // Verifying student signature
            let stud_data = format!(
                "{}{}{}{}{}",
                c.course_id, c.course_name, c.stud_pub_key, c.stud_wallet_addr, c.uni_wallet_addr
            );
            let secp = Secp256k1::new();

            // Using ? operator to handle unwraps safely
            let stud_pub_key = PublicKey::from_str(&c.stud_pub_key).map_err(|_| ())?;
            let stud_signature = Signature::from_str(&c.stud_sign).map_err(|_| ())?;
            let is_okay_stud = verify_signature(&secp, &stud_pub_key, &stud_data, &stud_signature).map_err(|_| ())?;

            // Verifying university signature
            let uni_data = format!(
                "{}{}{}{}{}{}{}{}",
                c.course_id, c.course_name, c.stud_pub_key, c.stud_wallet_addr, c.uni_wallet_addr, 
                c.stud_sign, &c.uni_pub_key, c.status
            );
            let uni_pub_key = PublicKey::from_str(&c.uni_pub_key).map_err(|_| ())?;
            let uni_signature = Signature::from_str(&c.uni_sign).map_err(|_| ())?;
            let is_okay_uni = verify_signature(&secp, &uni_pub_key, &uni_data, &uni_signature).map_err(|_| ())?;

            if is_okay_stud && is_okay_uni {
                return Ok(c);
            }
        }
    }
    Err(())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_status])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}