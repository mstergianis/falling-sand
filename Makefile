.PHONY = falling-sand

falling-sand:
	cargo build

web:
	cargo build --target wasm32-unknown-emscripten
	cp target/wasm32-unknown-emscripten/debug/falling-sand.d target/wasm32-unknown-emscripten/debug/falling-sand.js target/wasm32-unknown-emscripten/debug/falling_sand.wasm ./static/
