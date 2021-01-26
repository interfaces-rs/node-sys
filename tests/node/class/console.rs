use node_sys::{console, process, ConsoleConstructorOptions};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
pub fn new() {
    let stdout = process.stdout();
    let stderr = process.stderr();
    let options = ConsoleConstructorOptions::new(stdout, stderr);
    console::Console::new(options);
}
