r:
	cargo build --release
	cp target/wasm32-unknown-unknown/release/arduino_func.wasm /opt/usuyuki/.cache/mahiwa/user.wasm
	wasm2wat /opt/usuyuki/.cache/mahiwa/user.wasm -o /opt/usuyuki/.cache/mahiwa/user.wat
	(cd /opt/usuyuki/.cache/mahiwa && xxd -i user.wasm > user.h)
	cp /opt/usuyuki/.cache/mahiwa/user.h /opt/usuyuki/mahiwa_space/mahiwa-backend/src/wasm/user.h
