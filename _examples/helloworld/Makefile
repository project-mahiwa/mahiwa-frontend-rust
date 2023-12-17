r:
	cargo build --release
	cp target/wasm32-unknown-unknown/release/helloworld.wasm /opt/usuyuki/.cache/mahiwa/user.wasm
	(cd /opt/usuyuki/.cache/mahiwa && xxd -i user.wasm > user.h)
	cp /opt/usuyuki/.cache/mahiwa/user.h /opt/usuyuki/mahiwa_space/mahiwa-backend/src/wasm/user.h
