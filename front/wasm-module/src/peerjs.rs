// https://peerjs.com/docs/

use std::time::Duration;

use async_std::task::sleep;
use wasm_bindgen::prelude::*;

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
    pub fn _id(this: &Peer) -> JsValue;

    // https://peerjs.com/docs/#peer-id
    #[wasm_bindgen(method, getter, structural)]
    pub fn id(this: &Peer) -> String;
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
            let id = peer._id();
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
}

// impl Peer {
//     // https://peerjs.com/docs/#peerid
//     pub fn id(&self) -> String {
//         let vs = Object::entries(self.dyn_ref::<Object>().unwrap());
//         log(&vs);
//         let id = Reflect::get(self.dyn_ref::<Object>().unwrap(), &"id".into()).unwrap();
//         log(&id);
//         let id = js_sys::Reflect::get(&self, &"_id".into()).unwrap();
//         log(&id);
//         let v = Object::get_prototype_of(self.dyn_ref::<Object>().unwrap());
//         log(&v);
//         todo!()
//     }
// }
