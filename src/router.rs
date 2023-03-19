// src/router.rs

use crate::connection::Connection;
use crate::keypair::Keypair;
use crate::message::{Message, Recipients};
use crate::webrtc::WebRTC;
use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub struct Router {
    keypair: Keypair,
    connections: HashMap<Vec<u8>, Connection>,
    on_message_sender: UnboundedSender<Message>,
    on_message_receiver: UnboundedReceiver<Message>,
}

impl Router {
    pub fn new(keypair: Keypair) -> Self {
        let (on_message_sender, on_message_receiver) = mpsc::unbounded();

        Router {
            keypair,
            connections: HashMap::new(),
            on_message_sender,
            on_message_receiver,
        }
    }

    pub fn public_key(&self) -> Vec<u8> {
        self.keypair.public_key().to_vec()
    }

    pub fn add_connection(&mut self, connection: Connection) {
        let public_key = connection.remote_public_key.clone();
        self.connections.insert(public_key, connection);
    }

    pub async fn create_connection(
        &mut self,
        router: Rc<RefCell<Router>>,
        remote_public_key: Vec<u8>,
    ) -> Result<(), JsValue> {
        let connection = Connection::new(router, remote_public_key).await?;
        self.add_connection(connection);
        Ok(())
    }

    pub fn route_message(&mut self, message: Message) -> Result<(), JsValue> {
        match message.recipients {
            Recipients::Wildcard => {
                let signed_message = self.sign_message(message)?;
                for (_, connection) in self.connections.iter() {
                    connection.send_message(signed_message.clone())?;
                }
            }
            Recipients::Specific(public_keys) => {
                let mut undelivered_keys = Vec::new();
                let signed_message = self.sign_message(message)?;
                for public_key in public_keys {
                    if let Some(connection) = self.connections.get(&public_key) {
                        connection.send_message(signed_message.clone())?;
                    } else {
                        undelivered_keys.push(public_key);
                    }
                }
                if !undelivered_keys.is_empty() {
                    let updated_message = self.update_recipients(signed_message, undelivered_keys)?;
                    for (_, connection) in self.connections.iter() {
                        connection.send_message(updated_message.clone())?;
                    }
                }
            }
        }

        Ok(())
    }

    fn sign_message(&self, message: Message) -> Result<Message, JsValue> {
        let mut signed_message = message.clone();
        signed_message.sign(&self.keypair)?;
        Ok(signed_message)
    }

    fn update_recipients(
        &self,
        message: Message,
        undelivered_keys: Vec<Vec<u8>>,
    ) -> Result<Message, JsValue> {
        let mut updated_message = message.clone();
        updated_message.recipients = Recipients::Specific(undelivered_keys.into_iter().collect());
        updated_message.sign(&self.keypair)?;
        Ok(updated_message)
    }

    pub fn spawn_message_listener(&mut self) {
        let mut on_message_receiver = self.on_message_receiver.clone();
        spawn_local(async move {
            while let Some(message) = on_message_receiver.next().await {
                // Handle incoming messages from connections (e.g., process message or forward it)
            }
        });
    }
}

