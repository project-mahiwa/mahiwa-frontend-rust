```
cargo init
```

でRustのディレクトリを用意．

```
rustup target add wasm32-unknown-unknown
cargo build
```

## target

```
rustc --print target-list | rg wasm
wasm32-unknown-emscripten
wasm32-unknown-unknown
wasm32-wasi
wasm32-wasi-preview1-threads
wasm64-unknown-unknown
```
