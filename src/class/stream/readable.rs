use crate::{
    class::{self, EventEmitter},
    interface::PipeOptions,
};
use js_sys::{AsyncIterator, Number};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = EventEmitter, extends = AsyncIterator)]
    #[derive(Clone, Debug)]
    pub type Readable;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn is_paused(this: &Readable) -> bool;

    #[wasm_bindgen(method)]
    pub fn pause(this: &Readable) -> Readable;

    #[wasm_bindgen(method)]
    pub fn pipe(this: &Readable, dest: &class::stream::Writable, opts: PipeOptions) -> bool;

    #[wasm_bindgen(method)]
    pub fn read(this: &Readable, size: Option<Number>) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn resume(this: &Readable) -> bool;

    #[wasm_bindgen(method)]
    pub fn set_encoding(this: &Readable) -> bool;

    #[wasm_bindgen(method)]
    pub fn shift(this: &Readable) -> bool;

    #[wasm_bindgen(method)]
    pub fn unpipe(this: &Readable) -> bool;

    #[wasm_bindgen(method)]
    pub fn unshift(this: &Readable) -> bool;

    #[wasm_bindgen(method)]
    pub fn wrap(this: &Readable) -> bool;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn readable(this: &Readable) -> bool;
}
