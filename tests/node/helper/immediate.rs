use node_sys::{timers, Immediate};
use wasm_bindgen::{prelude::*, JsCast};

pub fn new() -> Immediate {
    let clo = Closure::wrap(Box::new(|| {}) as Box<dyn Fn()>);
    let fun = clo.as_ref().unchecked_ref();
    let immediate = timers::set_immediate(fun, Box::new([]));
    clo.forget();
    immediate
}
