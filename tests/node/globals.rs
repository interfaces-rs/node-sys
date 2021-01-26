use node_sys::globals;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn dirname() {
    let _ = globals::__dirname.clone();
}

#[wasm_bindgen_test]
fn filename() {
    let _ = globals::__filename.clone();
}

#[wasm_bindgen_test]
fn exports() {
    let _ = globals::exports.clone();
}

#[wasm_bindgen_test]
fn global() {
    let _ = globals::global.clone();
}

#[wasm_bindgen_test]
fn module() {
    let module = globals::module.clone();
    let _ = module.children();
    let _ = module.exports();
    let _ = module.filename();
    let _ = module.id();
    let _ = module.loaded();
    let _ = module.parent();
    let _ = module.paths();
    let _ = module.require(&"module".into());
}

#[wasm_bindgen_test]
fn queue_microtask() {
    let clo = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
    let fun = clo.as_ref().unchecked_ref();
    globals::queue_microtask(&fun);
    clo.forget();
}
