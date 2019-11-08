
/**
 * @typedef {object} KeyPair
 *
 * @description KeyPair struct
 *
 * @property {Uint8Array} public	- Public key
 * @property {Uint8Array} secret	- Secret key
 */

/**
 * @class KeyManager
 *
 * @description Create and manage 25519 keys for use with 'hcs0' Holochain encoding
 *
 * @param {Buffer} seed			- Buffer to be used as seed for creating keys (optional)
 *
 * @prop {KeyPair} signing		- Signing keypair
 * @prop {KeyPair} encryption		- Encryption keypair
 *
 * @return {KeyManager} instance of KeyManager class
 */
class KeyManager {
    
    constructor ( seed ) {
    }

    /**
     * @static
     * @async
     * @function deriveSeed
     * @memberof KeyManager
     *
     * @description Derive seed from email and password
     *
     * @param {string} email		- User email
     * @param {string} password		- User password
     *
     * @return {KeyManager} instance of KeyManager class
     *
     * @example
     * const seed = KeyManger.deriveSeed( "somebody@example.com", "Pa55w0rd!" );
     * const keys = new KeyManger( seed );
     */
    static async deriveSeed ( email, password ) {
    }

    /**
     * @instance
     * @async
     * @function sign
     * @memberof KeyManager
     *
     * @description Sign a message using private signing key
     *
     * @return {Uint8Array} signed message
     *
     * @example
     * let signature = await Keys.sign( message );
     */
    async sign () {
    }
    
    /**
     * @instance
     * @async
     * @function verify
     * @memberof KeyManager
     *
     * @description Verify a signed message against given key
     *
     * @return {Boolean} verification result
     *
     * @example
     * let genuine = await Keys.verify( signature, message, Keys.sign.public );
     */
    async verify () {
    }
    
    /**
     * @instance
     * @async
     * @function encrypt
     * @memberof KeyManager
     *
     * @description Encrypt a message using private encryption key
     *
     * @return {Uint8Array} encrypted message
     *
     * @example
     * let encrypted = await Keys.encrypt( message );
     */
    async encrypt () {
    }
    
    /**
     * @instance
     * @async
     * @function decrypt
     * @memberof KeyManager
     *
     * @description Decrypt a message using private encryption key
     *
     * @return {String} decrypted message
     *
     * @example
     * let message = await Keys.decrypt( encrypted );
     */
    async decrypt () {
    }
    
}


module.exports = {
    KeyManager,
};
