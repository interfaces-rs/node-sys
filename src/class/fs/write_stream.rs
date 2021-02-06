use crate::{stream, CreateWriteStreamOptions};
use js_sys::{Function, JsString};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::Url;

#[wasm_bindgen(module = "fs")]
extern {
    #[wasm_bindgen(js_name = createWriteStream)]
    pub fn create_write_stream_with_string(path: &JsString, options: Option<&CreateWriteStreamOptions>) -> WriteStream;

    #[wasm_bindgen(js_name = createWriteStream)]
    pub fn create_write_stream_with_url(url: &Url, options: Option<&CreateWriteStreamOptions>) -> WriteStream;

    #[wasm_bindgen(js_name = unlinkSync)]
    pub fn unlink_sync_path(path: &JsString);

    #[wasm_bindgen(js_name = unlinkSync)]
    pub fn unlink_sync_url(url: &Url);
}

#[wasm_bindgen(module = "fs")]
extern {
    #[wasm_bindgen(extends = stream::Writable)]
    #[derive(Clone, Debug)]
    pub type WriteStream;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, getter, js_name = bytesWritten)]
    pub fn bytes_written(this: &WriteStream) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn path(this: &WriteStream) -> JsValue; // Buffer | string

    #[wasm_bindgen(method, getter)]
    pub fn pending(this: &WriteStream) -> bool;
}

//******************************//
// Instance Methods (Overloads) //
//******************************//

#[wasm_bindgen]
impl WriteStream {
    pub fn add_listener_with_open(&self, listener: &Function) -> WriteStream {
        self.add_listener("open", listener).unchecked_into()
    }

    pub fn add_listener_with_close(&self, listener: &Function) -> WriteStream {
        self.add_listener("close", listener).unchecked_into()
    }

    pub fn on_with_open(&self, listener: &Function) -> WriteStream {
        self.on("open", listener).unchecked_into()
    }

    pub fn on_with_close(&self, listener: &Function) -> WriteStream {
        self.on("close", listener).unchecked_into()
    }

    pub fn once_with_open(&self, listener: &Function) -> WriteStream {
        self.once("open", listener).unchecked_into()
    }

    pub fn once_with_close(&self, listener: &Function) -> WriteStream {
        self.once("close", listener).unchecked_into()
    }

    pub fn prepend_listener_with_open(&self, listener: &Function) -> WriteStream {
        self.prepend_listener("open", listener).unchecked_into()
    }

    pub fn prepend_listener_with_close(&self, listener: &Function) -> WriteStream {
        self.prepend_listener("close", listener).unchecked_into()
    }

    pub fn prepend_once_listener_with_open(&self, listener: &Function) -> WriteStream {
        self.prepend_once_listener("open", listener).unchecked_into()
    }

    pub fn prepend_once_listener_with_close(&self, listener: &Function) -> WriteStream {
        self.prepend_once_listener("close", listener).unchecked_into()
    }
}
