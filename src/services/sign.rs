use aes_gcm::aead::consts::U12;
use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use lazy_static::lazy_static;
use sha3::digest::generic_array::GenericArray;
use sha3::{Digest, Sha3_256};
use std::fmt::{Debug, Display, Formatter};

fn create_cipher() -> Aes256Gcm {
    let key_str =
        std::env::var("RITW_SIGNKEY").expect("environment variable RITW_SIGNKEY is required");
    let hash = Sha3_256::digest(&key_str);
    let key = Key::<Aes256Gcm>::from_slice(&hash);

    aes_gcm::Aes256Gcm::new(key)
}

lazy_static! {
    static ref CIPHER: Aes256Gcm = create_cipher();
}

pub struct SignError;

impl Debug for SignError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ritw::models::Error")
    }
}

impl Display for SignError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ritw::models::Error")
    }
}

impl std::error::Error for SignError {}

pub struct Sign;


impl Sign {
    pub fn encrypt(value: &[u8]) -> Result<Vec<u8>, SignError> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext = CIPHER
            .encrypt(&nonce, value)
            .map_err(|_| SignError)?;

        let mut buffer: Vec<u8> = Vec::with_capacity(nonce.len() + ciphertext.len());
        let (head, body) = buffer.split_at_mut(nonce.len());
        head.copy_from_slice(&nonce);
        body.copy_from_slice(&ciphertext);

        Ok(buffer)
    }

    pub fn decrypt(value: &[u8]) -> Result<Vec<u8>, SignError> {
        if value.len() < 12 {
            return Err(SignError);
        }

        let (head, body) = value.split_at(12);

        let nonce: GenericArray<u8, U12> = *Nonce::from_slice(head);

        match CIPHER.decrypt(&nonce, body) {
            Ok(buffer) => Ok(buffer),
            Err(_) => Err(SignError),
        }
    }
}
