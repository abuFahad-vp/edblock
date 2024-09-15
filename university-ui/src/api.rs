use edblock::generate_key::generate_key;
use edblock::utils::{get_certificates, get_time};
use reqwest::Client;
use edblock::blockchain::blockchain_core::Certificate;
use edblock::blockchain::blockchain_rest::rest_api::Msg;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1v15::SigningKey;
use rsa::rand_core::OsRng;
use rsa::signature::RandomizedSigner;
use rsa::RsaPrivateKey;

pub async fn check_certificate(url: &str) -> Result<Vec<Certificate>, reqwest::Error> {
    let address = generate_key("uni").wallet_address;
    let certificates: Vec<Certificate> = get_certificates(url, &address).await?.into_iter().filter(|elem| {
        elem.status != "completed"
    }).collect();
    Ok(certificates)
}

pub async fn sign_certificate(url: &str, cert: &Certificate) -> bool {

    let client = Client::new();
    let key_pair = generate_key("uni");
    let status = "completed";
    let uni_priv_key = RsaPrivateKey::from_pkcs1_pem(&key_pair.priv_key_pem).expect("Failed to parse the private key pem file");
    let data = format!("{}{}{}{}{}{}{}{}",
        cert.course_id,cert.course_name, cert.stud_pub_key, cert.stud_wallet_addr, cert.uni_wallet_addr, 
        cert.stud_sign, &key_pair.pub_key_pem, status);

    let signing_key = SigningKey::<rsa::sha2::Sha256>::new(uni_priv_key);
    let uni_sign = signing_key.sign_with_rng(&mut OsRng, data.as_bytes()).to_string();

    let certificate = Certificate {
        timestamp: get_time(),
        certificate_id: cert.certificate_id.clone(),
        course_id: cert.course_id.clone(),
        course_name: cert.course_name.clone(),
        stud_pub_key: cert.stud_pub_key.clone(),
        stud_wallet_addr: cert.stud_wallet_addr.clone(),
        uni_wallet_addr: cert.uni_wallet_addr.clone(),
        stud_sign: cert.stud_sign.clone(),
        uni_pub_key: key_pair.pub_key_pem.clone(),
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