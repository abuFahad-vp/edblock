use std::{fs::File, io::{ErrorKind, Read, Write}};
use crate::blockchain::wallet::KeyPairAddress;

fn get_key_content_else_generate(prefix: &str, file_name: &str) -> Result<String, std::io::Error> {
    let file_path_name = format!("{}_{file_name}", prefix);
    let file = File::open(&file_path_name);
    if let Ok(mut file) = file {
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("Failed to read the contents");
        return Ok(buf);
    }
    println!("Failed to open the {file_name}. Generating a new {file_name}");
    let key_pair = KeyPairAddress::new();
    let mut file = File::create(&file_path_name).expect("Failed to create {file_name}");
    match file_name {
        "pub_key.pem" => {
            file.write_all(key_pair.pub_key_pem.as_bytes())?;
            Ok(key_pair.pub_key_pem)
        },

        "priv_key.pem" => {
            file.write_all(key_pair.priv_key_pem.as_bytes())?;
            Ok(key_pair.priv_key_pem)
        },

        "address.txt" => {
            file.write_all(key_pair.wallet_address.as_bytes())?;
            Ok(key_pair.wallet_address)
        },
        _ => Err(std::io::Error::new(ErrorKind::NotFound, "the specified file not found"))
    }
}

pub fn generate_key(prefix: &str) -> KeyPairAddress {

    let pub_key  =  get_key_content_else_generate(prefix,"pub_key.pem").unwrap();
    let priv_key =  get_key_content_else_generate(prefix,"priv_key.pem").unwrap();
    let address  =  get_key_content_else_generate(prefix,"address.txt").unwrap();

    KeyPairAddress {
        pub_key_pem: pub_key, 
        priv_key_pem: priv_key,
        wallet_address: address
    }
}