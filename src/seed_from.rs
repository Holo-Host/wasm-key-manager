use crate::util::*;
use ed25519_dalek::*;
use wasm_bindgen::prelude::*;

const ARGON2_ADDITIONAL_DATA: &[u8] = b"holo chaperone web user ed25519 key v1";

/// @description Derive seed from DNA SHA-256 digest bytes, email, and password
///
/// @example
/// const dnaSha256 = new Uint8Array([
///     66, 123, 133, 136, 133,   6, 247, 116,
///      4,  59,  43, 206, 131, 168, 123,  44,
///     54,  52,   3,  53, 134,  75, 137,  43,
///     63,  26, 216, 191,  67, 117,  38, 142
/// ]);
///
/// seedFrom( dnaSha256, "example@holo.host", "password" ); // Uint8Array [ ... ]
#[wasm_bindgen(js_name = seedFrom)]
pub fn seed_from(
    dna_sha256: &[u8],
    email: &str,
    password: &str,
) -> Fallible<Vec<u8>> {
    console_error_panic_hook::set_once();

    // This allows to use email addresses shorter than 8 bytes.
    let salt = Sha512::digest(email.as_bytes());
    let mut seed = [0; SECRET_KEY_LENGTH];

    argon2min::Argon2::new(2, 4, 1 << 16, argon2min::Variant::Argon2id)
        .map_err(into_js_error)?
        .hash(
            &mut seed,
            password.as_bytes(),
            &salt,
            dna_sha256,
            ARGON2_ADDITIONAL_DATA,
        );

    Ok(seed.to_vec())
}
