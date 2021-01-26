use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = WebAssembly, extends = Object)]
    #[derive(Clone, Debug)]
    pub type WebAssembly;
}
