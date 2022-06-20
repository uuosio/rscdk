ALL : ./target/counter.wasm ./target/counter.abi

./target/wasm32-wasi/release/counter.wasm : lib.rs
	RUSTFLAGS="-C link-arg=-zstack-size=8192 -Clinker-plugin-lto" cargo +nightly build --target=wasm32-wasi -Zbuild-std --no-default-features --release -Zbuild-std-features=panic_immediate_abort

./target/counter.abi: abigen/main.rs lib.rs
	cargo run --package abi-gen --release

./target/counter.wasm : ./target/wasm32-wasi/release/counter.wasm
	cp ./target/wasm32-wasi/release/counter.wasm ./target/counter.wasm

build:
	RUSTFLAGS="-C link-arg=-zstack-size=8192 -Clinker-plugin-lto" cargo +nightly build --target=wasm32-wasi -Zbuild-std --no-default-features --release -Zbuild-std-features=panic_immediate_abort

abi: abigen/main.rs lib.rs
	cargo run --package abi-gen --release

test: ./target/counter.wasm ./target/counter.abi
	run-ipyeos -m pytest -s test.py -k test_counter
