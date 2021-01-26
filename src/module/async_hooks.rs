use crate::interface::CreateHookCallbacks;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "async_hooks")]
extern {
    #[wasm_bindgen(extends = Object)]
    pub type AsyncHook;

    #[wasm_bindgen(js_name = createHook)]
    pub fn create_hook(callbacks: CreateHookCallbacks) -> AsyncHook;
}
