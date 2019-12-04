
# Contributing

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

## Build

The source is written in Rust and the wasm is built with `wasm-pack`.

```bash
bash -c './build.sh'
```
or
```bash
make pkg
```

## Testing

```bash
make tests
```

### Unit tests
There are some unit tests written that test the basic usage.

*Tests that need to be written*
- Failure tests (edge cases)

### Integration tests
TBD

## Documentation

**Requires** that `npm install` has been run in the parent directory.

```bash
make docs
```

## Publishing a new release

*Preview*
```bash
make preview-package
# npm pack --dry-run ./pkg
# npm notice 
# npm notice 📦  @holo-host/wasm-key-manager@0.0.4
# npm notice === Tarball Contents === 
# npm notice 373B    wasm_key_manager_bg.js  
# npm notice 5.0kB   wasm_key_manager_node.js
# npm notice 5.5kB   wasm_key_manager.js     
# npm notice 547B    package.json            
# npm notice 2.2kB   README.md               
# npm notice 693B    wasm_key_manager.d.ts   
# npm notice 237.2kB wasm_key_manager_bg.wasm
# npm notice === Tarball Details === 
# npm notice name:          @holo-host/wasm-key-manager             
# npm notice version:       0.0.4                                   
# npm notice filename:      holo-host-wasm-key-manager-0.0.4.tgz    
# npm notice package size:  97.0 kB                                 
# npm notice unpacked size: 251.4 kB                                
# npm notice shasum:        05ce209c885e502bc7ab65b70cc96eb940abce6d
# npm notice integrity:     sha512-b6ndB1lHXLPqu[...]SMJvVn49T3EwQ==
# npm notice total files:   7                                       
# npm notice 
# holo-host-wasm-key-manager-0.0.4.tgz
```

*Publish*
> **HINT:** You must be a member of https://www.npmjs.com/org/holo-host to publish
```bash
make publish-package
```
