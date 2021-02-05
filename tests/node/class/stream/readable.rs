use node_sys::stream;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/node/class/stream/readable.js")]
extern {
    fn readable_buffers() -> stream::Readable;
    fn readable_strings() -> stream::Readable;
}

mod async_iterator {
    use futures_util::io::AsyncReadExt;
    use js_sys::AsyncIterator;
    use js_sys_futures::JsAsyncRead;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn readable_buffers() {
        async fn run() -> Result<(), JsValue> {
            let readable = super::readable_buffers();
            let iterable = Into::<AsyncIterator>::into(readable);

            let mut reader = JsAsyncRead::new(iterable)?;
            let mut out = [0u8; 2];

            let amt = reader.read(&mut out).await.unwrap();
            let val = [1];
            assert_eq!(amt, val.len());
            assert_eq!(&out[.. amt], val);

            let amt = reader.read(&mut out).await.unwrap();
            let val = [2, 3];
            assert_eq!(amt, val.len());
            assert_eq!(&out[.. amt], val);

            let amt = reader.read(&mut out).await.unwrap();
            let val = [4, 5];
            assert_eq!(amt, val.len());
            assert_eq!(&out[.. amt], val);

            let amt = reader.read(&mut out).await.unwrap();
            let val = [6];
            assert_eq!(amt, val.len());
            assert_eq!(&out[.. amt], val);

            let amt = reader.read(&mut out).await.unwrap();
            let val = [];
            assert_eq!(amt, val.len());
            assert_eq!(&out[.. amt], val);

            Ok(())
        }
        run().await.unwrap();
    }

    #[wasm_bindgen_test]
    async fn readable_strings() {
        async fn run() -> Result<(), JsValue> {
            let readable = super::readable_strings();
            let iterable = Into::<AsyncIterator>::into(readable);

            let mut reader = JsAsyncRead::new(iterable)?;
            let mut out = [0u8; 3];

            let amt = reader.read(&mut out).await.unwrap();
            let val = "foo";
            assert_eq!(amt, val.len());
            assert_eq!(&out[.. amt], val.as_bytes());

            let amt = reader.read(&mut out).await.unwrap();
            let val = "bar";
            assert_eq!(amt, val.len());
            assert_eq!(&out[.. amt], val.as_bytes());

            let amt = reader.read(&mut out).await.unwrap();
            let val = "baz";
            assert_eq!(amt, val.len());
            assert_eq!(&out[.. amt], val.as_bytes());

            let amt = reader.read(&mut out).await.unwrap();
            let val = "";
            assert_eq!(amt, val.len());
            assert_eq!(&out[.. amt], val.as_bytes());

            Ok(())
        }
        run().await.unwrap();
    }
}
