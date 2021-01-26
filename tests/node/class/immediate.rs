use wasm_bindgen_test::*;

#[wasm_bindgen_test]
pub fn has_ref() {
    let immediate = crate::helper::immediate::new();
    immediate.has_ref();
}

#[wasm_bindgen_test]
pub fn ref_() {
    let immediate = crate::helper::immediate::new();
    immediate.ref_();
}

#[wasm_bindgen_test]
pub fn unref() {
    let immediate = crate::helper::immediate::new();
    immediate.unref();
}
