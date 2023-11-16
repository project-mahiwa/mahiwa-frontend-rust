```
cargo init
```

でRustのディレクトリを用意．

```
rustup target add wasm32-unknown-unknown
cargo build --release
wasm2wat target/wasm32-unknown-unknown/release/helloworld.wasm -o helloworld.wat
cp target/wasm32-unknown-unknown/release/helloworld.wasm helloworld.wasm
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
