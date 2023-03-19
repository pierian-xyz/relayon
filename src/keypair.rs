// src/keypair.rs

use ed25519_dalek::{Keypair as DalekKeypair, PublicKey, SecretKey, Signature};
use getrandom::getrandom;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct Keypair {
    keypair: DalekKeypair,
}

#[wasm_bindgen]
impl Keypair {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Keypair, JsValue> {
        let mut seed = [0u8; 32];
        getrandom(&mut seed).map_err(|_| JsValue::from_str("Failed to generate random seed"))?;

        let secret_key = SecretKey::from_bytes(&seed).map_err(|_| {
            JsValue::from_str("Failed to create secret key from generated seed")
        })?;
        let public_key = PublicKey::from(&secret_key);

        Ok(Keypair {
            inner: DalekKeypair { secret_key, public_key },
        })
    }

    pub fn from_private_key(private_key: &[u8]) -> Result<Self, JsValue> {
        let secret_key = match SecretKey::from_bytes(private_key) {
            Ok(key) => key,
            Err(_) => return Err(JsValue::from_str("Invalid private key")),
        };

        let public_key = PublicKey::from(&secret_key);
        let keypair = DalekKeypair { secret_key, public_key };

        Ok(Self { keypair })
    }

    pub fn public_key(&self) -> Vec<u8> {
        self.keypair.public.to_bytes().to_vec()
    }

    pub fn private_key(&self) -> Vec<u8> {
        self.keypair.secret.to_bytes().to_vec()
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        self.keypair.sign(message).to_bytes().to_vec()
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<(), JsValue> {
        let signature = match Signature::from_bytes(signature) {
            Ok(sig) => sig,
            Err(_) => return Err(JsValue::from_str("Invalid signature")),
        };

        self.keypair
            .public
            .verify(message, &signature)
            .map_err(|_| JsValue::from_str("Signature verification failed"))
    }
}

