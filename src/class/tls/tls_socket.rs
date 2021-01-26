use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tls")]
extern {
    #[wasm_bindgen(js_name = TLSSocket, extends = Object)]
    #[derive(Clone, Debug)]
    pub type TlsSocket;
}
