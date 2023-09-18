use wasm_extra::add_event_listener_with_callback;
use web_sys::{Event, HtmlElement};

use crate::alert;

pub(crate) struct MakeBetIcon(HtmlElement);

impl MakeBetIcon {
    pub(crate) fn new(e: HtmlElement) -> Self {
        Self(e)
    }

    pub(crate) fn add_click_handler(&self) {
        let Self(e) = self;
        add_event_listener_with_callback!(e, "click", {}, move |_event: Event| {
            // TODO: Implement this
            alert("Not implemented yet");
        })
    }
}
