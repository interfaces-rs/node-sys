use crate::interface::ReadableStream;
use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen(module = "fs")]
extern {
    #[wasm_bindgen(extends = ReadableStream)]
    #[derive(Clone, Debug)]
    pub type ReadStream;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "bytesRead")]
    pub fn bytes_read(this: &ReadStream) -> usize;

    #[wasm_bindgen(method)]
    pub fn path(this: &ReadStream) -> JsValue; // Buffer | string
}

//******************************//
// Instance Methods (Overloads) //
//******************************//

#[wasm_bindgen]
impl ReadStream {
    pub fn add_listener_with_open(&self, listener: &Function) -> ReadStream {
        self.add_listener("open", listener).unchecked_into()
    }

    pub fn add_listener_with_close(&self, listener: &Function) -> ReadStream {
        self.add_listener("close", listener).unchecked_into()
    }

    pub fn on_with_open(&self, listener: &Function) -> ReadStream {
        self.on("open", listener).unchecked_into()
    }

    pub fn on_with_close(&self, listener: &Function) -> ReadStream {
        self.on("close", listener).unchecked_into()
    }

    pub fn once_with_open(&self, listener: &Function) -> ReadStream {
        self.once("open", listener).unchecked_into()
    }

    pub fn once_with_close(&self, listener: &Function) -> ReadStream {
        self.once("close", listener).unchecked_into()
    }

    pub fn prepend_listener_with_open(&self, listener: &Function) -> ReadStream {
        self.prepend_listener("open", listener).unchecked_into()
    }

    pub fn prepend_listener_with_close(&self, listener: &Function) -> ReadStream {
        self.prepend_listener("close", listener).unchecked_into()
    }

    pub fn prepend_once_listener_with_open(&self, listener: &Function) -> ReadStream {
        self.prepend_once_listener("open", listener).unchecked_into()
    }

    pub fn prepend_once_listener_with_close(&self, listener: &Function) -> ReadStream {
        self.prepend_once_listener("close", listener).unchecked_into()
    }
}
