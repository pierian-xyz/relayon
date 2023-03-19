// src/utils.rs

use std::time::{SystemTime, UNIX_EPOCH};
use wasm_bindgen::prelude::*;

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn to_hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

pub fn from_hex_string(hex_str: &str) -> Result<Vec<u8>, JsValue> {
    if hex_str.len() % 2 != 0 {
        return Err(JsValue::from_str("Invalid hex string length"));
    }

    let result = (0..hex_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16))
        .collect::<Result<Vec<u8>, _>>();

    match result {
        Ok(bytes) => Ok(bytes),
        Err(_) => Err(JsValue::from_str("Invalid hex string")),
    }
}
