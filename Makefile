build-web:
	wasm-pack build ./crates/lava_core --release --out-dir ./pkg/web --target web

build-bundler:
	wasm-pack build --package lava_core --release --out-dir ./pkg/bundler --target bundler

build-nodejs:
	wasm-pack build --release --out-dir ./pkg/node --target nodejs
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen ./target/wasm32-unknown-unknown/release/bsv_wasm.wasm --out-dir pkg/node --target nodejs --weak-refs
	wasm-opt -O4 --dce ./pkg/node/bsv_wasm_bg.wasm -o ./pkg/node/bsv_wasm_bg.wasm