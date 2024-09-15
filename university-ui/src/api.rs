use std::str::FromStr;

use edblock::utils::{self, generate_key, get_certificates, get_time, verify_signature};
use reqwest::Client;
use edblock::blockchain::blockchain_core::Certificate;
use edblock::blockchain::blockchain_rest::Msg;
use std::io::ErrorKind;
use secp256k1::{ecdsa::Signature, PublicKey, Secp256k1};

pub async fn check_certificate(url: &str) -> Result<Vec<Certificate>, std::io::Error> {
    let address = generate_key("uni")?.address;

    let certificates: Vec<Certificate> = get_certificates(url, &address).await
        .map_err(|_| ErrorKind::InvalidData)?
        .into_iter()
        .filter_map(|c| {
            // Create data string
            let data = format!("{}{}{}{}{}", c.course_id, c.course_name, c.stud_pub_key, c.stud_wallet_addr, c.uni_wallet_addr);

            // Set up secp256k1 context
            let secp = Secp256k1::new();

            // Parse public key and signature
            let stud_pub_key = PublicKey::from_str(&c.stud_pub_key).ok()?;
            let signature = Signature::from_str(&c.stud_sign).ok()?;

            // Verify the signature
            match verify_signature(&secp, &stud_pub_key, &data, &signature) {
                Ok(true) if c.status != "completed" => Some(c),  // Only keep the valid and incomplete certificates
                _ => None,  // Filter out invalid or completed certificates
            }
        })
        .collect();

    Ok(certificates)
}

pub async fn sign_certificate(url: &str, cert: &Certificate) -> bool {

    let client = Client::new();
    let key_pair = generate_key("uni").unwrap();
    let status = "completed";
    let data = format!("{}{}{}{}{}{}{}{}",
        cert.course_id,cert.course_name, cert.stud_pub_key, cert.stud_wallet_addr, cert.uni_wallet_addr, 
        cert.stud_sign, &key_pair.pub_key.to_string(), status);

    let secp = Secp256k1::new();
    let uni_sign = utils::sign_message(&secp, &key_pair.priv_key, &data).unwrap().to_string();

    let certificate = Certificate {
        timestamp: get_time(),
        certificate_id: cert.certificate_id.clone(),
        course_id: cert.course_id.clone(),
        course_name: cert.course_name.clone(),
        stud_pub_key: cert.stud_pub_key.clone(),
        stud_wallet_addr: cert.stud_wallet_addr.clone(),
        uni_wallet_addr: cert.uni_wallet_addr.clone(),
        stud_sign: cert.stud_sign.clone(),
        uni_pub_key: key_pair.pub_key.to_string(),
        status: status.to_string(),
        uni_sign,
    };

    let response = client.post(format!("{}/transaction", url))
        .json(&certificate)
        .send()
        .await.unwrap();

    if response.status().is_success() {
        let response_data: Msg = response.json().await.unwrap();
        if response_data.status == 200 {
            println!("{:?}", response_data);
            return true;
        }
    }
    return false;
}