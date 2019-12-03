use ed25519_dalek::*;
use failure::*;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use hcid::HcidEncoding;

// wee_alloc shaves off ~4KB off WASM file size.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const ARGON2_ADDITIONAL_DATA: &[u8] = b"holo chaperone web user ed25519 key v1";

fn into_js_error(err: impl Fail) -> JsValue {
    js_sys::Error::new(&err.to_string()).into()
}

#[wasm_bindgen]
pub struct KeyManager(Keypair);

#[wasm_bindgen]
impl KeyManager {
    /// @description Derive seed from DNA SHA-256 digest bytes, email, and password
    ///
    /// @example
    /// const dnaSha256 = new Uint8Array([
    ///     66, 123, 133, 136, 133,   6, 247, 116,
    ///      4,  59,  43, 206, 131, 168, 123,  44,
    ///     54,  52,   3,  53, 134,  75, 137,  43,
    ///     63,  26, 216, 191,  67, 117,  38, 142
    /// ]);
    /// const seed = KeyManger.deriveSeed( dnaSha256, 'example@holo.host', 'password' );
    ///
    /// console.log( new KeyManager( seed ));
    #[wasm_bindgen(js_name = deriveSeed)]
    pub fn derive_seed(
        dna_sha256: &[u8],
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
                dna_sha256,
                ARGON2_ADDITIONAL_DATA,
            );

        Ok(seed.to_vec())
    }

    /// @description Verify signed message with provided public key
    ///
    /// @example
    /// let isGenuine = await keys.verifyWithPublicKey( message, signature, publicKey );
    #[wasm_bindgen(js_name = verifyWithPublicKey)]
    pub fn verify_with_public_key(message: &[u8], signature_bytes: &[u8], public_key_bytes: &[u8]) -> Result<bool, JsValue> {
        let public_key = PublicKey::from_bytes(public_key_bytes).map_err(into_js_error)?;
        let signature = Signature::from_bytes(signature_bytes).map_err(into_js_error)?;
        Ok(public_key.verify(message, &signature).is_ok())
    }

    /// @description Create an Ed25519 key manager out of seed
    /// @see KeyManager.deriveSeed
    #[wasm_bindgen(constructor)]
    pub fn new(seed: &[u8]) -> Result<KeyManager, JsValue> {
        console_error_panic_hook::set_once();

        let secret_key = SecretKey::from_bytes(seed).map_err(into_js_error)?;
        let public_key = PublicKey::from(&secret_key);

        Ok(Self(Keypair {
            secret: secret_key,
            public: public_key,
        }))
    }

    /// @description Get public key bytes
    ///
    /// @example
    /// const keys = new KeyManager();
    /// const publicKey = keys.publicKey();
    #[wasm_bindgen(js_name = publicKey)]
    pub fn public_key(&self) -> Vec<u8> {
        self.0.public.to_bytes()[..].into()
    }

    /// @description Sign message and return signature bytes
    ///
    /// @example
    /// let signature = await keys.sign( message );
    #[wasm_bindgen]
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let signature = self.0.sign(message);
        signature.to_bytes()[..].into()
    }

    /// @description Verify signed message against manager's public key
    ///
    /// @example
    /// let isGenuine = await keys.verify( message, signature );
    #[wasm_bindgen]
    pub fn verify(&self, message: &[u8], signature_bytes: &[u8]) -> Result<bool, JsValue> {
        let signature = Signature::from_bytes(signature_bytes).map_err(into_js_error)?;
        Ok(self.0.verify(message, &signature).is_ok())
    }
}

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
