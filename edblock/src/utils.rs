use std::{collections::HashMap, io::Write};

use chrono::Utc;
use reqwest::Client;

use crate::blockchain::blockchain_core::{Certificate, Transaction};

// read a string from the console by displaying a prompt
pub fn get_value(prompt: &str) -> String {
    print!("{prompt}");
    let mut value = String::new();
    std::io::stdout().flush().expect("Failed to flush the stdout.");
    std::io::stdin().read_line(&mut value).expect("Failed to read the miner address");
    value.trim().to_string()
}

pub fn get_time() -> String {

    let now_time = Utc::now();
    now_time.format("%d-%m-%Y %H:%M:%S").to_string()
}

pub async fn get_certificates(url: &str, addr: &str) -> Result<Vec<Certificate>, reqwest::Error> {
    let client = Client::new();
    let request_url = format!("{}/get_transaction/{}",url, addr);
    let response = client.get(request_url)
        .send()
        .await?
        .json::<Vec<Transaction>>()
        .await?;

    let mut unique_map: HashMap<String, Certificate> = HashMap::new();
    for item in response.clone().into_iter() {
        let id = &item.certificate.certificate_id;
        if let Some(existing) = unique_map.get(id) {
            if existing.status != "completed" && item.certificate.status == "completed" {
                unique_map.insert(id.clone(), item.certificate);
            }
        } else {
            unique_map.insert(id.clone(), item.certificate);
        }
    }
    let mut certificates: Vec<Certificate> = unique_map.into_values().collect();
    certificates.sort_by(|a,b| {
        a.certificate_id.cmp(&b.certificate_id)
    });
    Ok(certificates)
}