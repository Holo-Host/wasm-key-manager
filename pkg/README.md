
# Overview

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
