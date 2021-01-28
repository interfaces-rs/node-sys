use node_sys::path;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn join() {
    path::join({
        let val = vec!["foo".into(), "bar".into(), "baz".into()];
        val.into_boxed_slice()
    });
}
