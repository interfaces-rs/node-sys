use node_sys::{events, events::EventEmitter, process};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn once() {
    let emitter = EventEmitter::new();
    let promise = events::once(&emitter, "event");
    let clo = Closure::wrap(Box::new(move || {
        emitter.emit("event", Box::new([42.into()]));
    }) as Box<dyn Fn()>);
    process.next_tick(clo.as_ref().unchecked_ref(), Box::new([0.into()]));
    clo.forget();
    JsFuture::from(promise).await.unwrap_throw();
}
