use argon2::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use rand::thread_rng;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};


pub struct Hash{
    hash: String,
}

impl Hash{
    fn check_hash(&self,password:String,salt:&str)->bool{
        let argon2 = Argon2::default();
        argon2.hash_password(password.as_bytes(),&SaltString::generate(rsa::rand_core::OsRng))?.to_string()==self.hash
    }
    fn from_hash(h:String)->Hash{
        Hash{
            hash:h,
        }
    }
    fn new(password:&str,salt:&str)->Hash{
        let argon2 = Argon2::default();
        Hash{
            hash: argon2.hash_password(password.as_bytes(),salt)?.to_string(),
        }
    }
}

fn encrypt(data:Vec<u8>,){
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rsa::rand_core::OsRng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    pub_key.encrypt(&mut rsa::rand_core::OsRng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
}
fn decrypt(data:Vec<u8>){
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rsa::rand_core::OsRng, bits).expect("failed to generate a key");
    priv_key.decrypt(Pkcs1v15Encrypt, &data).expect("failed to decrypt");
}