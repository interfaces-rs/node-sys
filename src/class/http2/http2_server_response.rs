use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "http2")]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type Http2ServerResponse;
}
