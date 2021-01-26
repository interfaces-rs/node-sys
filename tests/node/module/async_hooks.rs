use node_sys::async_hooks;
use wasm_bindgen_test::*;

mod helper {
    use node_sys::CreateHookCallbacks;
    use wasm_bindgen::{prelude::*, JsCast};

    pub fn create_hook_callbacks() -> CreateHookCallbacks {
        let init = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        let before = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        let after = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        let destroy = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        let promise_resolve = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
        let callbacks = CreateHookCallbacks::new(
            init.as_ref().unchecked_ref(),
            before.as_ref().unchecked_ref(),
            after.as_ref().unchecked_ref(),
            destroy.as_ref().unchecked_ref(),
            promise_resolve.as_ref().unchecked_ref(),
        );
        init.forget();
        before.forget();
        after.forget();
        destroy.forget();
        promise_resolve.forget();
        callbacks
    }
}

#[wasm_bindgen_test]
fn create_hook_callbacks_new() {
    helper::create_hook_callbacks();
}

#[wasm_bindgen_test]
fn create_hook() {
    let callbacks = helper::create_hook_callbacks();
    async_hooks::create_hook(callbacks);
}
