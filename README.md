# relayon
A message relay

## Protocol

Wasm application that creates a p2p network of message routers with the routers running on other systems.

Every router has a public and private key. The user can initialize the router by either providing the private key or the application will generate a new private key and public key pair for it and returning the pair to the user. The public key for the router also acts as its ID.

After starting the router connects to other instances of the router using WebRTC. Upon connecting the router instances exchange their public keys. The router maintains a map of the other router's public key to the connection.

The message contains -
• Unique message ID
• Sender identified by the sender's public key
• Recipients can be either a wildcard (*) or a list of one or more recipient public keys.
• Timestamp of the message creation
• The message type is an integer
• The message body is a JSON and is processed by the intended recipient based on the message type
• The message body contains at least two fields, instruction and content. It may have other fields depending on the message type.

1. A router Receives a message from the javascript or from other routers.
2. If the recipient is a wildcard, it signs and forwards the message to all the other connected routers.
3. If it has recipients listed and then it checks if the public key of any of the recipients matches the keys in the connection map. If it finds the match, it sends the message to those routers. It then checks if any more recipients another kee undelivered, if there are, it updates the recipient list removing the  delivered keys.

## Project Structure

The project is structured as a Rust library with a wasm-bindgen-types crate for the typescript definitions.

### Overview

p2p_wasm_router/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── router.rs
│   ├── keypair.rs
│   ├── message.rs
│   ├── connection.rs
│   ├── webrtc.rs
│   └── utils.rs
└── wasm-bindgen-types/
    ├── types.d.ts
    └── types_bg.wasm


### The key files

*router.rs*: Implements the Router struct and its methods.
*keypair.rs*: Implements the Keypair struct and related functions for generating and managing keypairs.
*message.rs*: Implements the Message struct and related functions for creating, signing, and verifying messages.
*connection.rs*: Implements the Connection struct and related functions for managing WebRTC connections.
*webrtc.rs*: Implements the WebRTC signaling and data channels.
*utils.rs*: Implements utility functions for encoding and decoding messages, timestamps, etc.

## Building

### Prerequisites

Rust and wasm-pack

### Build

You can compile this project Rust code to WASM using `wasm-pack` or `cargo wasm` and use it with JavaScript in your web application.

