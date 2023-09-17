use wasm_extra::add_event_listener_with_callback;
use web_sys::{Event, HtmlElement};

use crate::{alert, peerjs::PeerId};

pub(crate) struct CopyPeerIdIcon(HtmlElement);

impl CopyPeerIdIcon {
    pub(crate) fn new(e: HtmlElement) -> Self {
        Self(e)
    }

    pub(crate) fn add_clipboard_overwrite_click_handler(&self, _peer_id: PeerId) {
        let Self(e) = self;
        add_event_listener_with_callback!(e, "click", {}, move |_event: Event| {
            alert("Not implemented yet");
        })
    }
}
