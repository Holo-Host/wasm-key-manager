
const expect				= require('chai').expect;
const crypto				= require('crypto');

const { KeyManager }			= require('../../pkg/');

describe("Key Manager", () => {

    it("should create KeyManager instance with random bytes", async () => {
	const seed			= crypto.randomBytes( 32 );
	const keys			= new KeyManager( seed );

	expect( keys.agentId()		).to.be.a("string");
    });

    it("should derive seed from input", async () => {
	const known_seed		= new Uint8Array([
	    45,  12, 221,  54, 152, 210,  92,  23,
	    47,  44, 246, 126, 237, 194, 205, 129,
	    200, 206,  41,  68, 122, 108, 233, 200,
	    52, 133,  16,  47, 203, 102,  21, 201
	]);
	
	const seed			= KeyManager.deriveSeed("some dna hash", "someone@example.com", "Passw0rd!");

	expect( seed			).to.be.an("uint8array");
	expect( seed			).to.deep.equal( known_seed );
    });

    it("should sign and verify using derived seed", async () => {
	const known_signature		= new Uint8Array([
	    40, 120, 215, 244,  19, 247,  36,  47, 242,   5, 137,
	    113, 128,  51, 127, 173,  56,  75,  32,   0,  22, 183,
	    251, 100,  98,  51,  31,  98,  74, 218,  22, 174, 183,
	    41,  56, 194, 169,  63, 106, 105,  41, 240,  84, 113,
	    68, 199, 110, 206, 135, 192,  99,  69,  28, 154, 211,
	    11, 133,  82, 220,  45,  49, 243, 204,  14
	]);
	
	const seed			= KeyManager.deriveSeed("some dna hash", "someone@example.com", "Passw0rd!");
	const keys			= new KeyManager( seed );
	const message			= "Hello World";

	const signature			= keys.sign( message );

	expect( signature		).to.be.an("uint8array");
	expect( signature		).to.deep.equal( known_signature );
	
	const verified			= keys.verify( message, signature );
	
	expect( verified		).to.be.true;
    });
    
});
