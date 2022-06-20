ALL : ./target/token.wasm ./target/token.abi

./target/wasm32-wasi/release/token.wasm : lib.rs
	RUSTFLAGS="-C link-arg=-zstack-size=8192 -Clinker-plugin-lto" cargo +nightly build --target=wasm32-wasi -Zbuild-std --no-default-features --release -Zbuild-std-features=panic_immediate_abort

./target/token.abi: abigen/main.rs lib.rs
	cargo run --package abi-gen --release

./target/token.wasm : ./target/wasm32-wasi/release/token.wasm
	cp ./target/wasm32-wasi/release/token.wasm ./target/token.wasm

build:
	RUSTFLAGS="-C link-arg=-zstack-size=8192 -Clinker-plugin-lto" cargo +nightly build --target=wasm32-wasi -Zbuild-std --no-default-features --release -Zbuild-std-features=panic_immediate_abort

abi: abigen/main.rs lib.rs
	cargo run --package abi-gen --release

test: ./target/token.wasm ./target/token.abi
	run-ipyeos -m pytest -s test.py -k test_token
