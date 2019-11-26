use js_sys::{Function, Object};
use wasm_bindgen::prelude::*;

#[allow(non_snake_case)]
#[wasm_bindgen]
pub struct CreateHookCallbacks {
    init: Function,
    before: Function,
    after: Function,
    destroy: Function,
    promise_resolve: Function,
}

#[wasm_bindgen]
impl CreateHookCallbacks {
    #[wasm_bindgen(constructor)]
    pub fn new(
        init: &Function,
        before: &Function,
        after: &Function,
        destroy: &Function,
        promise_resolve: &Function,
    ) -> CreateHookCallbacks {
        CreateHookCallbacks {
            init: init.clone(),
            before: before.clone(),
            after: after.clone(),
            destroy: destroy.clone(),
            promise_resolve: promise_resolve.clone(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn init(&self) -> Function {
        self.init.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_init(&mut self, init: Function) {
        self.init = init;
    }

    #[wasm_bindgen(getter)]
    pub fn before(&self) -> Function {
        self.before.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_before(&mut self, before: Function) {
        self.before = before;
    }

    #[wasm_bindgen(getter)]
    pub fn after(&self) -> Function {
        self.after.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_after(&mut self, after: Function) {
        self.after = after;
    }

    #[wasm_bindgen(getter)]
    pub fn destroy(&self) -> Function {
        self.destroy.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_destroy(&mut self, destroy: Function) {
        self.destroy = destroy;
    }

    #[wasm_bindgen(getter)]
    pub fn promise_resolve(&self) -> Function {
        self.promise_resolve.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_promise_resolve(&mut self, promise_resolve: Function) {
        self.promise_resolve = promise_resolve;
    }
}

#[wasm_bindgen(module = "async_hooks")]
extern {
    /// A representation of a collection of functions called throughout the lifetime of async
    /// operations.
    #[wasm_bindgen(extends = Object)]
    pub type AsyncHook;

    /// Registers functions to be called for different lifetime events of each async operation.
    #[wasm_bindgen(js_name = "createHook")]
    pub fn create_hook(callbacks: CreateHookCallbacks) -> AsyncHook;
}