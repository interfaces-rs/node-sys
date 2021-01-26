use crate::class;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = class::stream::Duplex)]
    pub type Socket;

    #[wasm_bindgen(method, getter, js_name = isTTY)]
    pub fn is_tty(this: &Socket) -> bool;
}
