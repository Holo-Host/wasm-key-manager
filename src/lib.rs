use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello() -> Result<String, JsValue> {
    Ok("Hello, world!".to_string())
}
