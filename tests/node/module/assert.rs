use js_sys::{Object, Promise, Reflect};
use node_sys::assert;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn assertion_error_options_new() {
    let message = Default::default();
    let actual = 0u32.into();
    let expected = 1u32.into();
    let operator = "strictEqual".into();
    let options = assert::AssertionErrorOptions::new(message, actual, expected, operator);
    assert_eq!(options.actual(), 0);
    assert_eq!(options.expected(), 1);
    assert_eq!(options.operator(), "strictEqual");
}

#[wasm_bindgen_test]
fn assertion_error_new() {
    let message = Default::default();
    let actual = 0u32.into();
    let expected = 1u32.into();
    let operator = "strictEqual".into();
    let options = assert::AssertionErrorOptions::new(message, actual, expected, operator);
    let error = assert::AssertionError::new(options);
    let message = error.message();
    assert_eq!(message, "Expected values to be strictly equal:\n\n0 !== 1\n");
}

#[wasm_bindgen_test]
fn deep_strict_equal() {
    let fst = {
        let this = Object::new();
        Reflect::set(&this, &"a".into(), &1u32.into()).unwrap_throw();
        this
    };
    let snd = {
        let this = Object::new();
        Reflect::set(&this, &"a".into(), &"1".into()).unwrap_throw();
        this
    };
    let message = Default::default();
    if let Err(_err) = assert::deep_strict_equal(&fst, &snd, message) {
        // #[should_panic]
    }
}

#[wasm_bindgen_test]
async fn does_not_reject_function() {
    let clo = Closure::wrap(Box::new(|| Promise::resolve(&JsValue::UNDEFINED)) as Box<dyn Fn() -> Promise>);
    let fun = clo.as_ref().unchecked_ref();
    let promise = assert::does_not_reject_function(&fun, None, None);
    clo.forget();
    JsFuture::from(promise).await.unwrap_throw();
}

#[wasm_bindgen_test]
async fn does_not_reject_promise() {
    JsFuture::from(assert::does_not_reject_promise(
        &Promise::resolve(&JsValue::UNDEFINED),
        None,
        None,
    ))
    .await
    .unwrap_throw();
}

#[wasm_bindgen_test]
fn does_not_throw() {
    let clo = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
    let fun = clo.as_ref().unchecked_ref();
    assert::does_not_throw(&fun, None, None).unwrap_throw();
}

#[wasm_bindgen_test]
fn fail() {
    if let Err(_err) = assert::fail(None) {}
}

#[wasm_bindgen_test]
fn if_error_fail() {
    if let Err(_err) = assert::if_error(&0.into()) {}
}

#[wasm_bindgen_test]
fn if_error_pass() {
    assert::if_error(&JsValue::NULL).unwrap_throw();
}

#[wasm_bindgen_test]
fn not_deep_strict_equal() {
    let fst = {
        let this = Object::new();
        Reflect::set(&this, &"a".into(), &1u32.into()).unwrap_throw();
        this
    };
    let snd = {
        let this = Object::new();
        Reflect::set(&this, &"a".into(), &"1".into()).unwrap_throw();
        this
    };
    let message = Default::default();
    assert::not_deep_strict_equal(&fst, &snd, message).unwrap_throw();
}

#[wasm_bindgen_test]
fn not_strict_equal() {
    assert::not_strict_equal(&0.into(), &1.into(), None).unwrap_throw();
}

#[wasm_bindgen_test]
fn ok_fail() {
    if let Err(_err) = assert::ok(&JsValue::NULL, None) {}
}

#[wasm_bindgen_test]
fn ok_pass() {
    assert::ok(&1.into(), None).unwrap_throw();
}

#[wasm_bindgen_test]
async fn rejects_function() {
    let clo = Closure::wrap(Box::new(|| Promise::reject(&JsValue::UNDEFINED)) as Box<dyn Fn() -> Promise>);
    let fun = clo.as_ref().unchecked_ref();
    let promise = assert::rejects_function(&fun, None, None);
    clo.forget();
    JsFuture::from(promise).await.unwrap_throw();
}

#[wasm_bindgen_test]
async fn rejects_promise() {
    JsFuture::from(assert::rejects_promise(
        &Promise::reject(&JsValue::UNDEFINED),
        None,
        None,
    ))
    .await
    .unwrap_throw();
}

#[wasm_bindgen_test]
fn strict_equal() {
    assert::strict_equal(&0.into(), &0.into(), None).unwrap_throw();
}

#[wasm_bindgen_test]
fn throws() {
    use js_sys::JsString;
    let clo = Closure::wrap(Box::new(|| {
        let val = JsValue::UNDEFINED;
        let str = val.unchecked_into::<JsString>(); // coerce undefined into String
        str.length();
    }) as Box<dyn Fn()>);
    let fun = clo.as_ref().unchecked_ref();
    assert::throws(&fun, None, None).unwrap_throw();
}
