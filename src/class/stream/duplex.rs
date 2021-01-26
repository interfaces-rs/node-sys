use crate::class;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = class::stream::Readable, extends = class::stream::Writable)]
    #[derive(Clone, Debug)]
    pub type Duplex;
}
