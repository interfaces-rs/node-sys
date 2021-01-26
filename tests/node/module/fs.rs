use node_sys::fs;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn access() {
    let path = &".".into();
    let mode = Default::default();
    let clo = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
    let fun = clo.as_ref().unchecked_ref();
    fs::access(path, mode, fun);
    clo.forget()
}

#[wasm_bindgen_test]
fn access_sync() {
    let path = &".".into();
    let mode = Default::default();
    fs::access_sync(path, mode);
}

#[wasm_bindgen_test]
fn append_file() {
    let path = &crate::helper::file::tmpfile("append_file.test");
    let data = &crate::helper::buffer::new();
    let options = Default::default();
    let clo = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
    let fun = clo.as_ref().unchecked_ref();
    fs::append_file(path, data, options, fun);
    clo.forget();
}
