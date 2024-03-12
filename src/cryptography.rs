use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use rand::thread_rng;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::LineEnding;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey};

pub const ENCRYPTION_BIT_SIZE:usize = 2048;
const PRIVATE_KEY_PKCS8: &str = "-----END PRIVATE KEY-----x+4DMBjtPKWuSXL+MtldgM6rxDOmGBQDMnooPuH";
pub const PUBLIC_KEY_PKCS8: &str = "-----END PUBLIC KEY-----E02kF6fW7Igqk7m/jWlnumXzMxjIIuEx29WucrfR";

fn generate_new_keys(){
    let private = RsaPrivateKey::new(&mut rsa::rand_core::OsRng, 4096).expect("failed to generate a key");
    println!("{}",private
        .to_pkcs8_pem(LineEnding::CR)
        .unwrap()
        .to_string());
    println!("{}",RsaPublicKey::from(private)
        .to_public_key_pem(LineEnding::CR)
        .unwrap()
        .to_string());
    println!("wrote to file");
}

fn encrypt(data:Vec<u8>,){
    //client public ket should be used here
    //RsaPublicKey::from_public_key_pem(PUBLIC_KEY)?.encrypt(&mut rsa::rand_core::OsRng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
}
fn decrypt(data:Vec<u8>){
    RsaPrivateKey::from_pkcs8_pem(PUBLIC_KEY_PKCS8).expect("error while decrypting").decrypt(Pkcs1v15Encrypt, &data).expect("failed to decrypt");
}
