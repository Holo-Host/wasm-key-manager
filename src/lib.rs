use ed25519_dalek::*;
use failure::*;
use wasm_bindgen::prelude::*;

const ARGON2_ADDITIONAL_DATA: &[u8] = b"hclient user ed25519 key v2";

fn into_js_error(err: impl Fail) -> JsValue {
    js_sys::Error::new(&err.to_string()).into()
}

#[wasm_bindgen]
pub struct KeyManager(Keypair);

#[wasm_bindgen]
impl KeyManager {
    // TODO: add hApp hash argument
    #[wasm_bindgen(js_name = deriveSeed)]
    pub fn derive_seed(
        dna_multihash: &str,
        email: &str,
        password: &str,
    ) -> Result<Vec<u8>, JsValue> {
        // This allows to use email addresses shorter than 8 bytes.
        let salt = Sha512::digest(email.as_bytes());
        let mut seed = [0; SECRET_KEY_LENGTH];

        argon2min::Argon2::new(2, 4, 1 << 16, argon2min::Variant::Argon2id)
            .map_err(into_js_error)?
            .hash(
                &mut seed,
                password.as_bytes(),
                &salt,
                dna_multihash.as_bytes(),
                ARGON2_ADDITIONAL_DATA,
            );

        Ok(seed.to_vec())
    }

    #[wasm_bindgen(constructor)]
    pub fn new(seed: &[u8]) -> Result<KeyManager, JsValue> {
        let secret_key = SecretKey::from_bytes(seed).map_err(into_js_error)?;
        let public_key = PublicKey::from(&secret_key);

        Ok(Self(Keypair {
            secret: secret_key,
            public: public_key
        }))
    }

    #[wasm_bindgen]
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let signature = self.0.sign(&message);
        signature.to_bytes()[..].into()
    }
}
