use wasm_extra::add_event_listener_with_callback;
use web_sys::{Event, HtmlDialogElement, HtmlElement};

pub(crate) struct OpenContactsIcon(HtmlElement);

impl OpenContactsIcon {
    pub(crate) fn new(e: HtmlElement) -> Self {
        Self(e)
    }

    pub(crate) fn add_click_handler(&self, contacts_modal: HtmlDialogElement) {
        let Self(e) = self;
        add_event_listener_with_callback!(e, "click", {}, move |_event: Event| {
            contacts_modal.show_modal().unwrap();
        })
    }
}
