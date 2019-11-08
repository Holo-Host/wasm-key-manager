
# Wasm Key Manager

Rust/Wasm key management implementation that uses Ed25519 signing algorithm and Argon2 key
derivation.  The private key remains in Wasm memory with no direct access for Javascript.

![](https://img.shields.io/maintenance/last%20update%202019-11/2019?style=flat-square)
![](https://img.shields.io/badge/dev@latest-0.0.4-orange?style=flat-square)

## Release ![](https://img.shields.io/npm/v/@holo-host/wasm-key-manager/latest?style=flat-square)
Release source - https://github.com/Holo-Host/chaperone/tree/master/key-manager/


## API Reference

[API Reference](https://holo-host.github.io/chaperone/key-manager/docs/KeyManager.html)


## Usage

```js
const { KeyManager } = require('@holo-host/wasm-key-manager');
const crypto = require('crypto');

const seed = crypto.randomBytes( 32 );
// or derive seed
const seed = KeyManager.deriveSeed("some dna hash", "someone@example.com", "Passw0rd!");

const message = "Hello World";
const keys = new KeyManager( seed );

const signature = keys.sign( message );
const verified = keys.verify( message, signature );
```

## Bundle for web

### `bootstrap.js`
```js
import("./index.js")
    .then(m => Object.assign(window, m))
    .catch(e => console.error("Error importing `index.js`:", e));
```

### `index.js`
```js
const { KeyManager } = require("@holo-host/wasm-key-manager");

module.exports = {
    KeyManager,
};
```

### `webpack.config.js`
```js
module.exports = {
    target: "web",

    entry: "./bootstrap.js",

    // Assign 'module.exports' to the window variable
    output: {
        libraryTarget: "window",
    },
};
```

## Contributors

**Development environment as of 2019/11**
- Node.js `12`
- Rust `1.38.0-nightly`

**Project employs**
- Wasm Pack (rust)
- JSDoc

**Setup**

Nix shell will provide packages listed in [./default.nix](./default.nix) `nativeBuildInputs`
```bash
nix-shell ./shell.nix
```

### Build

The source is written in Rust and the wasm is built with `wasm-pack`.

```bash
bash -c './build.sh'
```
or
```bash
make pkg
```

### Testing

```bash
make tests
```

#### Unit tests
There are some unit tests written that test the basic usage.

*Tests that need to be written*
- Failure tests (edge cases)

#### Integration tests
TBD

### Documentation

**Requires** that `npm install` has been run in the parent directory.

```bash
make docs
```
