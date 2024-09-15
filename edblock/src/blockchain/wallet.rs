use rsa::{pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey}, pkcs8::LineEnding, RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256};
use base58::ToBase58;
use ripemd::Ripemd160;
// use rsa::Pkcs1v15Encrypt;

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize, Clone)]
pub struct KeyPairAddress {
    pub priv_key_pem: String,
    pub pub_key_pem: String,
    pub wallet_address: String
}

impl KeyPairAddress {
    pub fn new() -> Self {

        let mut rng = rand::thread_rng();
        let bits = 2048;

        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);
        let priv_key_pem = priv_key.to_pkcs1_pem(LineEnding::LF).expect("Failed to generate private key's pem");
        let pub_key_pem = pub_key.to_pkcs1_pem(LineEnding::LF).expect("Failed to generate public key's pem");
        let wallet_address = Self::generate_wallet_address(pub_key_pem.as_bytes());

        KeyPairAddress {
            priv_key_pem: priv_key_pem.to_string(),
            pub_key_pem,
            wallet_address,
        }
    }

    // Helper function to perform SHA-256
    fn sha256(input: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    // Helper function to perform RIPEMD-160
    fn ripemd160(input: &[u8]) -> Vec<u8> {
        let mut hasher = Ripemd160::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    // Generate a wallet address from the public key
    fn generate_wallet_address(public_key: &[u8]) -> String {
        // Step 1: SHA-256 hash of the public key
        let sha256_result = Self::sha256(public_key);

        // Step 2: RIPEMD-160 hash of the SHA-256 result
        let ripemd160_result = Self::ripemd160(&sha256_result);

        // Step 3: Add version byte (0x00 for mainnet)
        let mut extended_ripemd160 = vec![0x00]; // Version byte
        extended_ripemd160.extend(&ripemd160_result);

        // Step 4: Perform double SHA-256 to generate the checksum
        let checksum = Self::sha256(&Self::sha256(&extended_ripemd160));

        // Step 5: Append the first 4 bytes of the checksum to the extended RIPEMD-160 result
        let mut final_result = extended_ripemd160;
        final_result.extend(&checksum[..4]); // Append first 4 bytes of the checksum

        // Step 6: Base58 encode the result to get the wallet address
        final_result.to_base58()
    }

    // fn generate_key() {
        // Output the result
        // println!("Generated Wallet Address: {}", wallet_address);

        // // Encrypt
        // let data = b"hello world";
        // let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
        // assert_ne!(&data[..], &enc_data[..]);

        // // Decrypt
        // let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
        // assert_eq!(&data[..], &dec_data[..]);
    // }
}