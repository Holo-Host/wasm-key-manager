const expect				= require('chai').expect;
const crypto				= require('crypto');

const { KeyManager, seedFrom }		= require('../../pkg');

const dnaSha256				= new Uint8Array([
    66, 123, 133, 136, 133,   6, 247, 116,
     4,  59,  43, 206, 131, 168, 123,  44,
    54,  52,   3,  53, 134,  75, 137,  43,
    63,  26, 216, 191,  67, 117,  38, 142
]);

describe("Key Manager", () => {
    it("should create KeyManager instance with random bytes", async () => {
	const seed			= crypto.randomBytes( 32 );
	const keys			= new KeyManager( seed );

	expect( keys.publicKey()	).to.be.a("uint8array");
    });

    it("should derive seed from input", async () => {
	const knownSeed			= new Uint8Array([
            225, 186, 208,  19, 196,  26,  72,  30,
             72,  91, 170, 129, 169, 229,  53, 112,
            216, 149,   4, 192,   1, 114, 148, 173,
             14,  68, 215,  72, 242, 209, 155, 196
	]);

        const seed			= seedFrom(dnaSha256, "example@holo.host", "password");

        expect( seed			).to.be.an("uint8array");
        expect( seed			).to.deep.equal( knownSeed );
    });

    it("should sign and verify using derived seed", async () => {
	const knownSignature		= new Uint8Array([
            155, 142, 163,   4, 233, 104, 154,  74,  85, 194, 232,
             87, 149, 144,  12, 129, 127,  90, 112, 105, 226, 241,
             50, 190, 249,  36,  34,  86,  35, 226, 236, 131,  91,
            156,  19,  86, 196, 124, 134, 133, 242, 152, 240,  65,
            117,  50, 221,  13, 181,  42,  96,  42, 187,  57, 229,
            130,  84,  85, 105, 122, 119,  20, 103,  14
	]);

        const seed			= seedFrom(dnaSha256, "example@holo.host", "password");
        const keys			= new KeyManager( seed );
        const message			= "Hello, world!";

        const signature			= keys.sign( message );

        expect( signature		).to.be.an("uint8array");
        expect( signature		).to.deep.equal( knownSignature );

        const isGenuine			= keys.verify( message, signature );

        expect( isGenuine		).to.be.true;

        const isGenuineStatic		= KeyManager.verifyWithPublicKey( message, signature, keys.publicKey() )

        expect( isGenuineStatic		).to.be.true;
    });
});
