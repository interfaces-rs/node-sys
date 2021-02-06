use crate::class;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "events")]
extern {
    #[wasm_bindgen(extends = class::EventEmitter)]
    #[derive(Clone, Debug)]
    pub type AbortSignal;
}
