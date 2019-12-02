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
    /// @static
    /// @async
    /// @function deriveSeed
    /// @memberof KeyManager
    ///
    /// @description Derive seed from email and password
    ///
    /// @example
    /// const seed = KeyManger.deriveSeed( 'somebody@example.com', 'Pa55w0rd!' );
    /// const keys = new KeyManger( seed );
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

    /// @description Create a Ed25519 key manager
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

    /// @instance
    /// @memberof KeyManager
    ///
    /// @description Get public key bytes
    ///
    /// @example
    /// const keys = new KeyManager();
    /// const publicKey = keys.publicKey();
    #[wasm_bindgen(js_name = publicKey)]
    pub fn public_key(&self) -> Vec<u8> {
        self.0.public.to_bytes()[..].into()
    }

    /// @instance
    /// @async
    /// @function sign
    /// @memberof KeyManager
    ///
    /// @description Sign a message using private key
    ///
    /// @example
    /// let signature = await Keys.sign( message );
    #[wasm_bindgen]
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let signature = self.0.sign(message);
        signature.to_bytes()[..].into()
    }

    /// @instance
    /// @async
    /// @function verify
    /// @memberof KeyManager
    ///
    /// @description Verify a signed message against given public key
    ///
    /// @example
    /// let genuine = await Keys.verify( signature, message, Keys.sign.public );
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

#[wasm_bindgen]
pub fn from_hcs0(public_key_hcid: &str) -> Result<Vec<u8>, JsValue> {
    HCS0_CODEC
        .decode(&public_key_hcid)
        .map_err(into_js_error)
}

#[wasm_bindgen]
pub fn to_hcs0(public_key_bytes: &[u8]) -> Result<String, JsValue> {
    HCS0_CODEC
        .encode(&public_key_bytes)
        .map_err(into_js_error)
}
