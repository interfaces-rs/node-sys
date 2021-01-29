use futures_util::StreamExt;
use js_sys_futures::JsStream;
use node_sys::{stream, Buffer};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/node/class/stream/readable.js")]
extern {
    fn readable() -> stream::Readable;
}

#[wasm_bindgen_test]
async fn async_iterator_stream() {
    async fn run() -> Result<(), JsValue> {
        let readable = readable();
        assert!(readable.readable());
        let mut stream = match JsStream::<Buffer>::new(readable.into()) {
            Ok(stream) => stream,
            Err(_err) => panic!("nope"),
        };
        while let Some(_buffer) = stream.next().await {
        }
        Ok(())
    }
    run().await.unwrap();
}
