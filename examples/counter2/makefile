ALL : ./target/counter2.wasm ./target/counter2.abi

./target/wasm32-wasi/release/counter2.wasm : lib.rs
	RUSTFLAGS="-C link-arg=-zstack-size=8192 -Clinker-plugin-lto" cargo +nightly build --target=wasm32-wasi -Zbuild-std --no-default-features --release -Zbuild-std-features=panic_immediate_abort

./target/counter2.abi: abigen/main.rs lib.rs
	cargo run --package abi-gen --release

./target/counter2.wasm : ./target/wasm32-wasi/release/counter2.wasm
	cp ./target/wasm32-wasi/release/counter2.wasm ./target/counter2.wasm

build:
	RUSTFLAGS="-C link-arg=-zstack-size=8192 -Clinker-plugin-lto" cargo +nightly build --target=wasm32-wasi -Zbuild-std --no-default-features --release -Zbuild-std-features=panic_immediate_abort

abi: abigen/main.rs lib.rs
	cargo run --package abi-gen --release

test: ./target/counter2.wasm ./target/counter2.abi
	run-ipyeos -m pytest -s test.py -k test_counter
