use rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};
use base58::ToBase58;
use ripemd::Ripemd160;
// use rsa::Pkcs1v15Encrypt;

#[derive(Debug, Clone)]
pub struct KeyPairAddress {
    pub priv_key: SecretKey,
    pub pub_key: PublicKey,
    pub address: String
}

impl KeyPairAddress {
    pub fn new() -> Self {

        let secp = Secp256k1::new();
        let mut rng = OsRng::default();
        let (sk, pk) = secp.generate_keypair(&mut rng);

        let address = Self::generate_wallet_address(&pk.to_string().as_bytes());

        KeyPairAddress {
            priv_key: sk,
            pub_key: pk,
            address,
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
}