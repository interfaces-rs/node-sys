use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type CreateWriteStreamOptions;

    // autoClose

    #[wasm_bindgen(method, getter, js_name = autoClose)]
    pub fn auto_close(this: &CreateWriteStreamOptions) -> bool;

    #[wasm_bindgen(method, setter, js_name = autoClose)]
    pub fn set_auto_close(this: &CreateWriteStreamOptions, value: bool);

    // emitClose

    #[wasm_bindgen(method, getter, js_name = emitClose)]
    pub fn emit_close(this: &CreateWriteStreamOptions) -> bool;

    #[wasm_bindgen(method, setter, js_name = emitClose)]
    pub fn set_emit_close(this: &CreateWriteStreamOptions, value: bool);

    // encoding

    #[wasm_bindgen(method, getter, js_name = encoding)]
    pub fn encoding(this: &CreateWriteStreamOptions) -> String;

    #[wasm_bindgen(method, setter, js_name = encoding)]
    pub fn set_encoding(this: &CreateWriteStreamOptions, value: &str);

    // fd

    #[wasm_bindgen(method, getter, js_name = fd)]
    pub fn fd(this: &CreateWriteStreamOptions) -> JsValue;

    #[wasm_bindgen(method, setter, js_name = fd)]
    pub fn set_fd(this: &CreateWriteStreamOptions, value: &JsValue);

    // flags

    #[wasm_bindgen(method, getter, js_name = flags)]
    pub fn flags(this: &CreateWriteStreamOptions) -> String;

    #[wasm_bindgen(method, setter, js_name = flags)]
    pub fn set_flags(this: &CreateWriteStreamOptions, value: &str);

    // fs

    #[wasm_bindgen(method, getter, js_name = fs)]
    pub fn fs(this: &CreateWriteStreamOptions) -> JsValue;

    #[wasm_bindgen(method, setter, js_name = fs)]
    pub fn set_fs(this: &CreateWriteStreamOptions, value: &JsValue);

    // mode

    #[wasm_bindgen(method, getter, js_name = mode)]
    pub fn mode(this: &CreateWriteStreamOptions) -> u32;

    #[wasm_bindgen(method, setter, js_name = mode)]
    pub fn set_mode(this: &CreateWriteStreamOptions, value: u32);

    // start

    #[wasm_bindgen(method, getter, js_name = start)]
    pub fn start(this: &CreateWriteStreamOptions) -> u32;

    #[wasm_bindgen(method, setter, js_name = start)]
    pub fn set_start(this: &CreateWriteStreamOptions, value: u32);
}
