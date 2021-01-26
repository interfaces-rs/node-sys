use node_sys::path;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn join() {
    path::join({
        let mut val = vec![];
        val.push("foo".into());
        val.push("bar".into());
        val.push("baz".into());
        val.into_boxed_slice()
    });
}
