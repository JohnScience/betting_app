mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-module!");
}

#[wasm_bindgen(start)]
fn main() {
    utils::set_panic_hook();
    greet();
}
