
RUSTFLAGS="-C link-arg=-zstack-size=8192 -Clinker-plugin-lto" cargo +nightly build --target=wasm32-wasi -Zbuild-std --no-default-features --release -Zbuild-std-features=panic_immediate_abort || exit 1
