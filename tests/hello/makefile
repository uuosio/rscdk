ALL : ./target/wasm32-wasi/release/hello.wasm ./target/hello.abi

./target/wasm32-wasi/release/hello.wasm : lib.rs
	RUSTFLAGS="-C link-arg=-zstack-size=8192 -Clinker-plugin-lto" cargo +nightly build --target=wasm32-wasi -Zbuild-std --no-default-features --release -Zbuild-std-features=panic_immediate_abort

./target/hello.abi: abigen/main.rs lib.rs
	cargo run --package abi-gen --release

./target/hello.wasm : ./target/wasm32-wasi/release/hello.wasm
	cp ./target/wasm32-wasi/release/hello.wasm ./target/hello.wasm

build:
	RUSTFLAGS="-C link-arg=-zstack-size=8192 -Clinker-plugin-lto" cargo +nightly build --target=wasm32-wasi -Zbuild-std --no-default-features --release -Zbuild-std-features=panic_immediate_abort

abi: abigen/main.rs lib.rs
	cargo run --package abi-gen --release

test: ./target/hello.wasm ./target/hello.abi
	run-ipyeos -m pytest -s test.py -k test_hello
