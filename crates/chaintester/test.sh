#!/bin/bash
for i in {1..100}
do
   ./target/debug/tester || exit 1
   # RUST_BACKTRACE=1 cargo run --bin tester || exit 1
done
