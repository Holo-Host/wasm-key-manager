use crate::util::*;
use hcid::HcidEncoding;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

lazy_static! {
    pub static ref HCS0_CODEC: hcid::HcidEncoding =
        HcidEncoding::with_kind("hcs0").expect("Couldn't init hcs0 hcid codec.");
}

/// @ignore
#[wasm_bindgen]
pub fn from_hcs0(public_key_hcid: &str) -> Result<Vec<u8>, JsValue> {
    HCS0_CODEC
        .decode(&public_key_hcid)
        .map_err(into_js_error)
}

/// @ignore
#[wasm_bindgen]
pub fn to_hcs0(public_key_bytes: &[u8]) -> Result<String, JsValue> {
    HCS0_CODEC
        .encode(&public_key_bytes)
        .map_err(into_js_error)
}
