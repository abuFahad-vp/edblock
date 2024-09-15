use edblock::utils::{self, generate_key, get_certificates, get_time};
use reqwest::Client;
use edblock::blockchain::blockchain_core::Certificate;
use edblock::blockchain::blockchain_rest::Msg;
use secp256k1::Secp256k1;

pub async fn check_certificate(url: &str) -> Result<Vec<Certificate>, reqwest::Error> {
    let address = generate_key("uni").unwrap().address;
    let certificates: Vec<Certificate> = get_certificates(url, &address).await?.into_iter().filter(|elem| {
        elem.status != "completed"
    }).collect();
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