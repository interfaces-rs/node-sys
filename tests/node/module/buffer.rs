use node_sys::buffer;
use wasm_bindgen_test::*;

#[allow(non_snake_case)]
#[wasm_bindgen_test]
fn INSPECT_MAX_BYTES() {
    let _ = buffer::INSPECT_MAX_BYTES;
}

#[wasm_bindgen_test]
fn constants() {
    let constants = &buffer::constants;
    let _ = constants.MAX_LENGTH();
    let _ = constants.MAX_STRING_LENGTH();
}

#[wasm_bindgen_test]
fn k_max_length() {
    let _ = buffer::k_max_length;
}

#[wasm_bindgen_test]
fn transcode() {
    let buffer = crate::helper::buffer::new();
    let encoding = &"utf8".into();
    buffer::transcode(&buffer, encoding, encoding);
}
