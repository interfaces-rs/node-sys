use crate::interface::{FormatInputPathObject, ParsedPath};
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "path")]
extern {
    pub static delimiter: JsString;

    pub static sep: JsString;

    pub fn basename(path: &JsString, ext: Option<JsString>) -> JsString;

    pub fn dirname(path: &JsString) -> JsString;

    pub fn extname(path: &JsString) -> JsString;

    pub fn format(object: FormatInputPathObject) -> JsString;

    pub fn is_absolute(path: &JsString) -> bool;

    #[wasm_bindgen(variadic)]
    pub fn join(paths: &Array) -> JsString;

    pub fn normalize(path: &JsString) -> JsString;

    pub fn parse(string: &JsString) -> ParsedPath;

    pub fn relative(from: &JsString, to: &JsString) -> JsString;

    #[wasm_bindgen(variadic)]
    pub fn resolve(path_segments: &Array) -> JsString;

// FIXME: path.posix

// FIXME: path.win32
}
