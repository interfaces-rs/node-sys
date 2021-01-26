use crate::class::Immediate;
use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "timers")]
extern {
    #[wasm_bindgen(variadic, js_name = setImmediate)]
    pub fn set_immediate(callback: &Function, args: Box<[JsValue]>) -> Immediate;
}
