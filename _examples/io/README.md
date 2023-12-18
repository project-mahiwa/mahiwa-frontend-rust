# arduino_func

Arduino関数の動作確認用Webasseblyを作る

```
rustup target add wasm32-unknown-unknown
cargo build --release
wasm2wat target/wasm32-unknown-unknown/release/helloworld.wasm -o helloworld.wat
cp target/wasm32-unknown-unknown/release/helloworld.wasm helloworld.wasm
```
