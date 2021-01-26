use crate::interface::ReadWriteStream;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = ReadWriteStream)]
    pub type Socket;

    #[wasm_bindgen(method, getter, js_name = isTTY)]
    pub fn is_tty(this: &Socket) -> bool;
}
