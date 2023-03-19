mod router;
mod keypair;
mod message;
mod connection;
mod webrtc;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Router {
    router: router::Router,
}

#[wasm_bindgen]
impl Router {
    pub fn new() -> Router { /* ... */ }
    pub fn with_key(private_key: &str) -> Result<Router, JsValue> { /* ... */ }
    pub fn get_public_key(&self) -> String { /* ... */ }
    pub fn connect(&mut self, signal: &str) -> Result<(), JsValue> { /* ... */ }
    pub fn send_message(&mut self, message: &str) -> Result<(), JsValue> { /* ... */ }
}
