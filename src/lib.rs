//! Raw bindings to the Node.js API for projects using wasm-bindgen

#![deny(clippy::all)]
// #![deny(missing_docs)] // FIXME: wasm-bindgen macros break this

pub(crate) mod class;
pub mod globals;
pub(crate) mod interface;
pub(crate) mod module;

pub use class::{stream, Buffer, EventEmitter, Immediate, Timeout, Wasi};
pub use interface::*;
pub use module::*;
