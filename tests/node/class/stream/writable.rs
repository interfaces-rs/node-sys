use futures_util::io::AsyncWriteExt;
use node_sys::{fs, stream};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn async_write() {
    async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let path = &crate::helper::file::tmpfile("async_write.test");
        let options = Default::default();
        let stream = fs::create_write_stream_with_string(path, options);
        let writable: stream::Writable = stream.into();

        let mut writer = stream::JsAsyncWrite::from(writable);

        let buf = &[1u8, 2, 3];
        let amt = writer.write(buf).await?;
        assert_eq!(amt, 3);

        let buf = &[4u8, 5];
        let amt = writer.write(buf).await?;
        assert_eq!(amt, 2);

        fs::unlink_sync_path(path);

        Ok(())
    }

    run().await.unwrap()
}
