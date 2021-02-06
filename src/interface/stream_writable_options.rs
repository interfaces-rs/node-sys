use crate::class;
use js_sys::{Function, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq)]
    pub type StreamWritableOptions;

    // highWaterMark

    #[wasm_bindgen(method, getter, js_name = highWaterMark)]
    pub fn high_water_mark(this: &StreamWritableOptions) -> f64;

    #[wasm_bindgen(method, setter, js_name = highWaterMark)]
    pub fn set_high_water_mark(this: &StreamWritableOptions, value: f64);

    // decodeStrings

    #[wasm_bindgen(method, getter, js_name = decodeStrings)]
    pub fn decode_string(this: &StreamWritableOptions) -> bool;

    #[wasm_bindgen(method, setter, js_name = decodeStrings)]
    pub fn set_decode_strings(this: &StreamWritableOptions, value: bool);

    // defaultEncoding

    #[wasm_bindgen(method, getter, js_name = defaultEncoding)]
    pub fn default_encoding(this: &StreamWritableOptions) -> String;

    #[wasm_bindgen(method, setter, js_name = defaultEncoding)]
    pub fn set_default_encoding(this: &StreamWritableOptions, value: &str);

    // objectMode

    #[wasm_bindgen(method, getter, js_name = objectMode)]
    pub fn object_mode(this: &StreamWritableOptions) -> bool;

    #[wasm_bindgen(method, setter, js_name = objectMode)]
    pub fn set_object_mode(this: &StreamWritableOptions, value: bool);

    // emitClose

    #[wasm_bindgen(method, getter, js_name = emitClose)]
    pub fn emit_close(this: &StreamWritableOptions) -> bool;

    #[wasm_bindgen(method, setter, js_name = emitClose)]
    pub fn set_emit_close(this: &StreamWritableOptions, value: bool);

    // write

    #[wasm_bindgen(method, getter, js_name = write)]
    pub fn write(this: &StreamWritableOptions) -> Function;

    #[wasm_bindgen(method, setter, js_name = write)]
    pub fn set_write(this: &StreamWritableOptions, value: &Function);

    // writev

    #[wasm_bindgen(method, getter, js_name = writev)]
    pub fn writev(this: &StreamWritableOptions) -> Function;

    #[wasm_bindgen(method, setter, js_name = writev)]
    pub fn set_writev(this: &StreamWritableOptions, value: &Function);

    // destroy

    #[wasm_bindgen(method, getter, js_name = destroy)]
    pub fn destroy(this: &StreamWritableOptions) -> Function;

    #[wasm_bindgen(method, setter, js_name = destroy)]
    pub fn set_destroy(this: &StreamWritableOptions, value: &Function);

    // final

    #[wasm_bindgen(method, getter, js_name = final)]
    pub fn final_(this: &StreamWritableOptions) -> Function;

    #[wasm_bindgen(method, setter, js_name = final)]
    pub fn set_final(this: &StreamWritableOptions, value: &Function);

    // construct

    #[wasm_bindgen(method, getter, js_name = construct)]
    pub fn construct(this: &StreamWritableOptions) -> Function;

    #[wasm_bindgen(method, setter, js_name = construct)]
    pub fn set_construct(this: &StreamWritableOptions, value: &Function);

    // autoDestroy

    #[wasm_bindgen(method, getter, js_name = autoDestroy)]
    pub fn auto_destroy(this: &StreamWritableOptions) -> bool;

    #[wasm_bindgen(method, setter, js_name = autoDestroy)]
    pub fn set_auto_destroy(this: &StreamWritableOptions, value: bool);

    // abortSignal

    #[wasm_bindgen(method, getter, js_name = abortSignal)]
    pub fn abort_signal(this: &StreamWritableOptions) -> class::AbortSignal;

    #[wasm_bindgen(method, setter, js_name = abortSignal)]
    pub fn set_abort_signal(this: &StreamWritableOptions, value: &class::AbortSignal);
}
