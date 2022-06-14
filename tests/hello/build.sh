
#RUSTFLAGS="-C link-arg=-zstack-size=8192 -Clinker-plugin-lto" cargo +nightly build --target=wasm32-unknown-unknown -Zbuild-std --no-default-features --release -Zbuild-std-features=panic_immediate_abort
RUSTFLAGS="-C link-arg=-zstack-size=8192 -Clinker-plugin-lto" cargo +nightly build --target=wasm32-wasi -Zbuild-std --no-default-features --release -Zbuild-std-features=panic_immediate_abort || exit 1
eosio-wasm2wast ./target/wasm32-wasi/release/hello.wasm  >./target/hello.wast || exit 1
cargo run --package abi-gen --release || exit 1
