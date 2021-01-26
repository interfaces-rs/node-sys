use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Function, js_name = HRTime)]
    pub type HrTime;
}
