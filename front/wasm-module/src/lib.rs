mod utils;

use wasm_bindgen::prelude::*;

mod components;
mod peerjs;

use peerjs::Peer;
use statue::initialize_elements;

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
    initialize_elements!(
        html: "../index.html", elements: {
            let peer_id_display = Single("#peer-id-display");
            let copy_peer_id_icon = Single("#copy-peer-id");
            let add_by_peer_id_icon = Single("#add-by-peer-id");
            let open_menu_icon = Single("#open-menu");
            let open_contacts_icon = Single("#open-contacts");
            let make_bet_icon = Single("#make-bet");
            let arbitrate_icon = Single("#arbitrate");
        },
        opts: {
            window_ret_ty: Some(T),
            document_ret_ty: None,
        }
    );

    let peer_id_display = components::PeerIdDisplay::new(peer_id_display);
    let copy_peer_id_icon = components::CopyPeerIdIcon::new(copy_peer_id_icon);
    let add_by_peer_id_icon = components::AddByPeerIdIcon::new(add_by_peer_id_icon);
    let open_menu_icon = components::OpenMenuIcon::new(open_menu_icon);
    let open_contacts_icon = components::OpenContactsIcon::new(open_contacts_icon);
    let make_bet_icon = components::MakeBetIcon::new(make_bet_icon);
    let arbitrate_icon = components::ArbitrateIcon::new(arbitrate_icon);

    let peer = Peer::new().await;
    let peer_id = peer.id();

    peer_id_display.display(&peer_id);
    copy_peer_id_icon.add_click_handler(peer_id);
    add_by_peer_id_icon.add_click_handler();
    open_menu_icon.add_click_handler();
    open_contacts_icon.add_click_handler();
    make_bet_icon.add_click_handler();
    arbitrate_icon.add_click_handler();
}
