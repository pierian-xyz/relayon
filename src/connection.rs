// src/connection.rs

use crate::message::Message;
use crate::webrtc::WebRTC;
use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Connection {
    webrtc: WebRTC,
    pub remote_public_key: Vec<u8>,
    send_message_sender: UnboundedSender<Message>,
    receive_message_receiver: UnboundedReceiver<Message>,
}

impl Connection {
    pub async fn new(router: Rc<RefCell<Router>>, remote_public_key: Vec<u8>) -> Result<Self, JsValue> {
        let webrtc = WebRTC::new(router).await?;
        let (send_message_sender, send_message_receiver) = mpsc::unbounded();
        let (receive_message_sender, receive_message_receiver) = mpsc::unbounded();

        let connection = Connection {
            webrtc,
            remote_public_key,
            send_message_sender,
            receive_message_receiver,
        };

        connection.spawn_message_handling(send_message_receiver, receive_message_sender);

        Ok(connection)
    }

    pub fn send_message(&self, message: Message) -> Result<(), JsValue> {
        self.send_message_sender.unbounded_send(message).map_err(|_| {
            JsValue::from_str("Failed to send message to the connection's message handling task")
        })
    }

    fn spawn_message_handling(
        &self,
        mut send_message_receiver: UnboundedReceiver<Message>,
        receive_message_sender: UnboundedSender<Message>,
    ) {
        let webrtc = self.webrtc.clone();

        spawn_local(async move {
            while let Some(message) = send_message_receiver.next().await {
                // Send the message over the WebRTC connection
                let json = message.to_json().unwrap();
                let _ = webrtc.data_channel.send_with_str(&json);
            }
        });

        self.webrtc
            .set_on_message_callback(Box::new(move |message: Message| {
                let _ = receive_message_sender.unbounded_send(message);
            }));
    }
}
