use edblock::utils::{self, get_certificates, get_time, generate_key};
use reqwest::Client;
use edblock::blockchain::blockchain_core::Certificate;
use edblock::blockchain::blockchain_rest::Msg;
use uuid::Uuid;
use secp256k1::Secp256k1;

pub async fn sent_transaction(tran_url: String, course_id: String, course_name: String, uni_wallet_addr: String) -> bool {
    let client = Client::new();
    let key_pair = generate_key("stu").unwrap();
    let stud_pub_key = key_pair.pub_key.to_string();
    let stud_wallet_addr = key_pair.address;

    let data = format!("{}{}{}{}{}", course_id, course_name, stud_pub_key, stud_wallet_addr, uni_wallet_addr);

    let secp = Secp256k1::new();
    let stud_sign = utils::sign_message(&secp, &key_pair.priv_key, &data)
        .unwrap()
        .to_string();
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
    let address = generate_key("stu").unwrap().address;
    get_certificates(url, &address).await
}