use crate::{EventEmitter, StreamWritableOptions};
use futures_core::Future;
use futures_util::io::AsyncWrite;
use js_sys::{Function, JsString, Promise, Uint8Array};
use js_sys_futures::*;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug)]
    pub type Writable;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    fn new(options: Option<&StreamWritableOptions>) -> Writable;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    fn end_callback(this: &Writable, cb: Option<&Function>);

    #[wasm_bindgen(method, js_name = end)]
    fn end_with_data_callback(this: &Writable, buffer: &Uint8Array, cb: Option<&Function>);

    #[wasm_bindgen(method, js_name = end)]
    fn end_with_string_callback(this: &Writable, string: &JsString, encoding: Option<&JsString>, cb: Option<&Function>);

    #[wasm_bindgen(method, js_name = write)]
    fn write_with_data_callback(this: &Writable, buffer: &Uint8Array, cb: Option<&Function>) -> bool;

    #[wasm_bindgen(method, js_name = write)]
    fn write_with_string_callback(
        this: &Writable,
        string: &JsString,
        encoding: Option<&JsString>,
        cb: Option<&Function>,
    ) -> bool;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    fn writeable(this: &Writable) -> bool;
}

mod promise {
    use super::Writable;
    use js_sys::{JsString, Promise, Uint8Array};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "src/class/stream/writable.js")]
    extern {
        #[wasm_bindgen(js_name = endPromise)]
        pub(super) fn end(this: &Writable) -> Promise;

        #[wasm_bindgen(js_name = endWithDataPromise)]
        pub(super) fn end_with_data(this: &Writable, buffer: &Uint8Array) -> Promise;

        #[wasm_bindgen(js_name = endWithStringPromise)]
        pub(super) fn end_with_string(this: &Writable, string: &JsString, encoding: Option<&JsString>) -> Promise;

        #[wasm_bindgen(js_name = writeWithDataPromise)]
        pub(super) fn write_with_data(this: &Writable, buffer: &Uint8Array) -> Promise;

        #[wasm_bindgen(js_name = writeWithStringPromise)]
        pub(super) fn write_with_string(this: &Writable, string: &JsString, encoding: Option<&JsString>) -> Promise;
    }
}

impl Writable {
    pub fn end(&self) -> Promise {
        promise::end(self)
    }

    pub fn end_with_data(&self, buffer: &Uint8Array) -> Promise {
        promise::end_with_data(self, buffer)
    }

    pub fn end_with_string(&self, string: &JsString, encoding: Option<&JsString>) -> Promise {
        promise::end_with_string(self, string, encoding)
    }

    pub fn write_with_data(&self, buffer: &Uint8Array) -> Promise {
        promise::write_with_data(self, buffer)
    }

    pub fn write_with_string(&self, string: &JsString, encoding: Option<&JsString>) -> Promise {
        promise::write_with_string(self, string, encoding)
    }
}

pub struct JsAsyncWrite {
    inner: Writable,
    close: Option<JsFuture>,
    write: Option<JsFuture>,
}

impl JsAsyncWrite {
    pub fn new(inner: Writable) -> Self {
        let close = Default::default();
        let write = Default::default();
        Self { inner, close, write }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Result<Poll<std::io::Result<()>>, JsValue> {
        let this = self.get_mut();
        if let Some(future) = &mut this.close {
            match Pin::new(future).poll(cx)? {
                Poll::Ready(_ready) => Ok(Poll::Ready(Ok(()))),
                Poll::Pending => Ok(Poll::Pending),
            }
        } else {
            this.close = Some(JsFuture::from(this.inner.end()));
            Ok(Poll::Pending)
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Result<Poll<std::io::Result<()>>, JsValue> {
        let this = self.get_mut();
        if let Some(future) = &mut this.write {
            match Pin::new(future).poll(cx)? {
                Poll::Ready(_ready) => Ok(Poll::Ready(Ok(()))),
                Poll::Pending => Ok(Poll::Pending),
            }
        } else {
            Ok(Poll::Ready(Ok(())))
        }
    }

    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Result<Poll<std::io::Result<usize>>, JsValue> {
        let this = self.get_mut();
        if let Some(future) = &mut this.write {
            match Pin::new(future).poll(cx)? {
                Poll::Ready(_ready) => {
                    this.write = Some(JsFuture::from(this.inner.write_with_data(&Uint8Array::from(buf))));
                    Ok(Poll::Ready(Ok(buf.len())))
                },
                Poll::Pending => Ok(Poll::Pending),
            }
        } else {
            this.write = Some(JsFuture::from(this.inner.write_with_data(&Uint8Array::from(buf))));
            Ok(Poll::Ready(Ok(buf.len())))
        }
    }
}

impl From<Writable> for JsAsyncWrite {
    fn from(inner: Writable) -> Self {
        Self::new(inner)
    }
}

#[derive(Clone, Debug)]
struct AsyncWritableError(JsValue);

unsafe impl Send for AsyncWritableError {
}
unsafe impl Sync for AsyncWritableError {
}

impl std::fmt::Display for AsyncWritableError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self.0)
    }
}

impl std::error::Error for AsyncWritableError {
}

impl AsyncWrite for JsAsyncWrite {
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<std::io::Result<()>> {
        match JsAsyncWrite::poll_close(self, cx) {
            Ok(success) => success,
            Err(error) => {
                let kind = std::io::ErrorKind::Other;
                let error = AsyncWritableError(error);
                Poll::Ready(Err(std::io::Error::new(kind, error)))
            },
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<std::io::Result<()>> {
        match JsAsyncWrite::poll_flush(self, cx) {
            Ok(success) => success,
            Err(error) => {
                let kind = std::io::ErrorKind::Other;
                let error = AsyncWritableError(error);
                Poll::Ready(Err(std::io::Error::new(kind, error)))
            },
        }
    }

    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<std::io::Result<usize>> {
        match JsAsyncWrite::poll_write(self, cx, buf) {
            Ok(success) => success,
            Err(error) => {
                let kind = std::io::ErrorKind::Other;
                let error = AsyncWritableError(error);
                Poll::Ready(Err(std::io::Error::new(kind, error)))
            },
        }
    }
}
