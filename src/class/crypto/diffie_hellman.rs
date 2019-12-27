use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type DiffieHellman;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn compute_secret(
        this: &DiffieHellman,
        other_public_key: &JsValue,
        input_encoding: Option<&str>,
        output_encoding: Option<&str>,
    ) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn generate_keys(this: &DiffieHellman, encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn get_generator(this: &DiffieHellman, encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn get_prime(this: &DiffieHellman, encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn get_private_key(this: &DiffieHellman, encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn get_public_key(this: &DiffieHellman, encoding: Option<&str>) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn set_private_key(this: &DiffieHellman, private_key: &JsValue, encoding: Option<&str>);

    #[wasm_bindgen(method)]
    pub fn set_public_key(this: &DiffieHellman, public_key: &JsValue, encoding: Option<&str>);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn verify_error(this: &DiffieHellman) -> u32;
}
