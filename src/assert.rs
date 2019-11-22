use js_sys::{Error, JsString};
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[wasm_bindgen]
pub struct AssertionErrorOptions {
    message: Option<JsString>,
    actual: JsValue,
    expected: JsValue,
    operator: JsString,
    // stack_start_fn: JsValue, // FIXME: Function
}

#[wasm_bindgen]
impl AssertionErrorOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        message: Option<JsString>,
        actual: JsValue,
        expected: JsValue,
        operator: JsString,
    ) -> Result<AssertionErrorOptions, JsValue> {
        Ok(AssertionErrorOptions {
            message,
            actual,
            expected,
            operator,
        })
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> Option<JsString> {
        self.message.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_message(&mut self, message: Option<JsString>) {
        self.message = message;
    }

    #[wasm_bindgen(getter)]
    pub fn actual(&self) -> JsValue {
        self.actual.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_actual(&mut self, actual: JsValue) {
        self.actual = actual
    }

    #[wasm_bindgen(getter)]
    pub fn expected(&self) -> JsValue {
        self.expected.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_expected(&mut self, expected: JsValue) {
        self.expected = expected
    }

    #[wasm_bindgen(getter)]
    pub fn operator(&self) -> JsString {
        self.operator.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_operator(&mut self, operator: JsString) {
        self.operator = operator
    }
}
