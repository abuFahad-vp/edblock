use edblock::generate_key::generate_key;
use edblock::utils::{get_certificates, get_time};
use rand::rngs::OsRng;
use reqwest::Client;
use edblock::blockchain::blockchain_core::Certificate;
use edblock::blockchain::blockchain_rest::rest_api::Msg;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1v15::SigningKey;
use rsa::signature::RandomizedSigner;
use rsa::RsaPrivateKey;
use uuid::Uuid;

pub async fn sent_transaction(tran_url: String, course_id: String, course_name: String, uni_wallet_addr: String) -> bool {
    let client = Client::new();
    let key_pair = generate_key("stu");
    let stud_pub_key = key_pair.pub_key_pem.clone();
    let stud_wallet_addr = key_pair.wallet_address.clone();

    let data = format!("{}{}{}{}{}", course_id, course_name, stud_pub_key, stud_wallet_addr, uni_wallet_addr);

    let priv_key = RsaPrivateKey::from_pkcs1_pem(&key_pair.priv_key_pem).expect("Failed to parse the private key pem file");

    let signing_key = SigningKey::<rsa::sha2::Sha256>::new(priv_key);
    let stud_sign = signing_key.sign_with_rng(&mut OsRng, data.as_bytes()).to_string();
    // println!("{}", stud_sign);

    let certificate_id = Uuid::new_v4().to_string();
    let certificate = Certificate {
        timestamp: get_time(),
        certificate_id,
        course_id,
        course_name,
        stud_pub_key,
        stud_wallet_addr,
        uni_wallet_addr,
        stud_sign,
        uni_pub_key: String::new(),
        status: String::new(),
        uni_sign: String::new(),
    };

    let response = client.post(format!("{}/transaction", tran_url))
        .json(&certificate)
        .send()
        .await;

    if let Ok(response) = response {
        if response.status().is_success() {
            let response_data: Msg = response.json().await.unwrap();
            println!("{:?}", response_data);
            return true
        }
    }
    return false
}

pub async fn check_certificate(url: &str) -> Result<Vec<Certificate>, reqwest::Error> {
    let address = generate_key("stu").wallet_address;
    get_certificates(url, &address).await
}