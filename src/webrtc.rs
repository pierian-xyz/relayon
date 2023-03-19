// src/webrtc.rs

use crate::message::Message;
use crate::router::Router;
use futures::channel::mpsc;
use js_sys::Promise;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{MessageEvent, RtcDataChannel, RtcDataChannelInit, RtcPeerConnection, RtcSdpType};

pub struct WebRTC {
    router: Rc<RefCell<Router>>,
    peer_connection: RtcPeerConnection,
    data_channel: RtcDataChannel,
    on_message_receiver: mpsc::UnboundedReceiver<Message>,
}

impl WebRTC {
    pub async fn new(router: Rc<RefCell<Router>>) -> Result<Self, JsValue> {
        let peer_connection = create_peer_connection()?;
        let data_channel = create_data_channel(&peer_connection)?;

        let (on_message_sender, on_message_receiver) = mpsc::unbounded();

        let webrtc = WebRTC {
            router,
            peer_connection,
            data_channel,
            on_message_receiver,
        };

        webrtc.set_data_channel_callbacks(on_message_sender);
        webrtc.set_peer_connection_callbacks();

        Ok(webrtc)
    }

    fn set_data_channel_callbacks(&self, sender: mpsc::UnboundedSender<Message>) {
        let data_channel = self.data_channel.clone();
        let on_message = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Some(message) = event.data().dyn_into::<js_sys::JsString>().ok() {
                let message = Message::from_json(&message.into()).unwrap();
                sender.unbounded_send(message).unwrap();
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        data_channel.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();
    }

    fn set_peer_connection_callbacks(&self) {
        // Implement callbacks for RtcPeerConnection, such as onicecandidate, onnegotiationneeded, etc.
    }
}

async fn create_offer(peer_connection: &RtcPeerConnection) -> Result<JsValue, JsValue> {
    let offer = JsFuture::from(peer_connection.create_offer()).await?;
    let sdp = offer
        .dyn_into::<web_sys::RtcSessionDescriptionInit>()
        .unwrap();

    JsFuture::from(peer_connection.set_local_description(&sdp)).await?;
    Ok(sdp.into())
}

async fn set_remote_description(
    peer_connection: &RtcPeerConnection,
    sdp: &web_sys::RtcSessionDescriptionInit,
) -> Result<(), JsValue> {
    JsFuture::from(peer_connection.set_remote_description(sdp)).await?;
    Ok(())
}

fn create_peer_connection() -> Result<RtcPeerConnection, JsValue> {
    let config = web_sys::RtcConfiguration::new();
    RtcPeerConnection::new_with_configuration(&config)
}

fn create_data_channel(peer_connection: &RtcPeerConnection) -> Result<RtcDataChannel, JsValue> {
    let data_channel_init = RtcDataChannelInit::new();
    peer_connection.create_data_channel_with_data_channel_dict("p2p", &data_channel_init)
}
