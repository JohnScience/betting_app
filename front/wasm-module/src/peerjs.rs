// https://peerjs.com/docs/

use std::time::Duration;

use async_std::task::sleep;
use wasm_bindgen::prelude::*;

pub struct PeerId(String);

#[wasm_bindgen]
extern "C" {
    // https://peerjs.com/docs/#peer
    pub type Peer;

    // After calling this method, the peer is not initialized immediately.
    #[wasm_bindgen(constructor)]
    pub fn _new() -> Peer;

    // https://peerjs.com/docs/#peer-id
    // Since the id is not available immediately, this method returns either null
    // or String.
    #[wasm_bindgen(method, getter, structural, js_name = id)]
    pub fn id_as_jsvalue(this: &Peer) -> JsValue;

    // https://peerjs.com/docs/#peer-id
    #[wasm_bindgen(method, getter, structural, js_name = id)]
    pub fn id_as_string(this: &Peer) -> String;
}

impl Peer {
    pub async fn new() -> Peer {
        const INITIAL_PAUSE_IN_MS: u64 = 100;
        const INTERVAL_IN_MS: u64 = 50;
        const TIMEOUT_IN_MS: u64 = 5_000;

        const MAX_RETRIES: u64 = TIMEOUT_IN_MS / INTERVAL_IN_MS;

        let peer = Peer::_new();
        sleep(Duration::from_millis(INITIAL_PAUSE_IN_MS)).await;
        let mut tries = 0;
        loop {
            let id = peer.id_as_jsvalue();
            if !id.is_null() {
                break;
            }
            tries += 1;
            if tries > MAX_RETRIES {
                panic!("Couldn't get peer id in {}ms", TIMEOUT_IN_MS);
            }
            sleep(Duration::from_millis(INTERVAL_IN_MS)).await;
        }
        peer
    }

    pub fn id(&self) -> PeerId {
        PeerId(self.id_as_string())
    }
}

impl PeerId {
    pub fn to_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for PeerId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
