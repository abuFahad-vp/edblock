use std::{collections::HashMap, io::Write};
use chrono::Utc;
use reqwest::Client;
use std::{fs::File, io::{ErrorKind, Read}};
use crate::blockchain::wallet::{self, KeyPairAddress};
use bincode::{serialize, deserialize};
use secp256k1::{PublicKey, Secp256k1, SecretKey, Message};
use sha2::{Sha256, Digest};


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

pub fn generate_key(prefix: &str) -> Result<KeyPairAddress, std::io::Error> {
    let file_path_name = format!("{}_keys.bin", prefix);
    let file = File::open(&file_path_name);
    if let Ok(mut file) = file {

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let (priv_key_bytes, pub_key_bytes, addr_bytes): (Vec<u8>, Vec<u8>, Vec<u8>) = 
            deserialize(&buffer).map_err(|_| ErrorKind::InvalidData)?;
        
        let priv_key = SecretKey::from_slice(&priv_key_bytes).map_err(|_| {
            ErrorKind::InvalidData
        })?;

        let pub_key = PublicKey::from_slice(&pub_key_bytes).map_err(|_| {
            ErrorKind::InvalidData
        })?;

        let address = String::from_utf8(addr_bytes).map_err(|_| {
            ErrorKind::InvalidData
        })?;
        Ok(KeyPairAddress{
            priv_key,
            pub_key,
            address
        })
    } else {
        let key_pair = wallet::KeyPairAddress::new();

        let priv_key_bytes = key_pair.priv_key[..].to_vec();
        let pub_key_bytes = key_pair.pub_key.serialize().to_vec();
        let address_bytes = key_pair.address.as_bytes().to_vec();

        let data = (priv_key_bytes, pub_key_bytes, address_bytes);

        let mut file = File::create(&file_path_name)?;

        let encoded: Vec<u8> = serialize(&data).map_err(|_| {
            ErrorKind::InvalidData
        })?;
        file.write_all(&encoded)?;
        Ok(key_pair)
    }
}

pub fn sign_message(secp: &Secp256k1<secp256k1::All>, secret_key: &SecretKey, message: &str) -> Result<secp256k1::ecdsa::Signature, Box<dyn std::error::Error>> {
    let message = create_message_hash(message);
    let signature = secp.sign_ecdsa(&message, secret_key);
    Ok(signature)
}

pub fn verify_signature(secp: &Secp256k1<secp256k1::All>, public_key: &PublicKey, message: &str, signature: &secp256k1::ecdsa::Signature) -> Result<bool, Box<dyn std::error::Error>> {
    let message = create_message_hash(message);
    Ok(secp.verify_ecdsa(&message, signature, public_key).is_ok())
}

fn create_message_hash(message: &str) -> Message {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let result = hasher.finalize();
    Message::from_digest_slice(&result).expect("32 bytes")
}