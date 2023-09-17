use web_sys::HtmlInputElement;

use crate::peerjs::PeerId;

pub(crate) struct PeerIdDisplay(HtmlInputElement);

impl PeerIdDisplay {
    pub(crate) fn new(e: HtmlInputElement) -> Self {
        Self(e)
    }

    pub(crate) fn display(&self, peer_id: &PeerId) {
        let s: &str = peer_id.as_ref();
        self.0.set_value(s);
    }
}
