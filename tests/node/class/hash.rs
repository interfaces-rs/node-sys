use node_sys::crypto;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
pub fn digest() {
    let algorithm = "md5";
    let options = Default::default();
    let hash = crypto::create_hash(algorithm, options);
    let encoding = Some("hex");
    hash.digest(encoding);
}

#[wasm_bindgen_test]
pub fn update() {
    let algorithm = "md5";
    let options = Default::default();
    let hash = crypto::create_hash(algorithm, options);
    let text = &"text".into();
    let encoding = Some("utf8");
    hash.update(text, encoding);
}
