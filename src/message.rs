// src/message.rs

use crate::keypair::Keypair;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub message_id: String,
    pub sender: Vec<u8>,
    pub recipients: Recipients,
    pub timestamp: u64,
    pub message_type: i32,
    pub body: JsonValue,
    pub signature: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum Recipients {
    Wildcard,
    Specific(HashSet<Vec<u8>>),
}

impl Message {
    pub fn new(
        message_id: &str,
        sender: &[u8],
        recipients: Recipients,
        timestamp: u64,
        message_type: i32,
        body: &JsonValue,
    ) -> Self {
        Message {
            message_id: message_id.to_string(),
            sender: sender.to_vec(),
            recipients,
            timestamp,
            message_type,
            body: body.clone(),
            signature: None,
        }
    }

    pub fn from_json(json_str: &str) -> Result<Self, JsValue> {
        serde_json::from_str(json_str).map_err(|_| JsValue::from_str("Invalid JSON"))
    }

    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(self).map_err(|_| JsValue::from_str("JSON serialization failed"))
    }

    pub fn sign(&mut self, keypair: &Keypair) -> Result<(), JsValue> {
        self.signature = None;
        let json = self.to_json()?;
        let signature = keypair.sign(json.as_bytes());
        self.signature = Some(signature);
        Ok(())
    }

    pub fn verify(&self, public_key: &[u8]) -> Result<(), JsValue> {
        let signature = match self.signature.as_ref() {
            Some(sig) => sig,
            None => return Err(JsValue::from_str("Message is not signed")),
        };

        let message_copy = Message {
            signature: None,
            ..self.clone()
        };

        let json = message_copy.to_json()?;
        let keypair = Keypair::from_private_key(public_key)?;
        keypair.verify(json.as_bytes(), signature)
    }
}
