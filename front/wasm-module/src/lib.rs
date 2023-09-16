mod utils;

use wasm_bindgen::prelude::*;

mod peerjs;

use peerjs::Peer;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(v: &JsValue);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_str(s: &str);
}

#[wasm_bindgen(start)]
async fn main() {
    utils::set_panic_hook();
    let peer = Peer::new().await;
    log(&peer);
    log_str(&peer.id());
}
