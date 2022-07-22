![build](https://github.com/uuosio/rscdk/actions/workflows/pr-any.yml/badge.svg?event=push)

Rust Smart Contracts Development Kit

## Getting Started

- [Try Rust Contracts in One Minute](https://colab.research.google.com/github/uuosio/rscdk/blob/master/quickstart/quick-start.ipynb)
- [Try Examples](https://github.com/uuosio/rscdk/tree/main/examples)
- [Read the RSCDK Book](https://uuosio.github.io/rscdk-book)

## Getting involved

- [Submit an Issue](https://github.com/uuosio/rscdk/issues)
- [Create a New Pull Request](https://github.com/uuosio/rscdk/pulls)
- [Help Improving the RSCDK Book](https://github.com/uuosio/rscdk-book)

## Debugging
![Debugging](https://github.com/uuosio/rscdk/blob/main/images/debugging.gif)

## Code Coverage Analysis

First, install grcon

```bash
cargo install grcov
```

Second, install llvm-tools

```bash
rustup component add llvm-tools-preview
```


Generate code coverage report in html
```bash
# rm -rf ./target

export CARGO_INCREMENTAL=0
export RUSTDOCFLAGS="-Cpanic=abort"
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off 
-Zpanic_abort_tests -Cpanic=abort"

cargo +nightly test
grcov . -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
```

You will need to start `eos-debugger` first if you don't do that. `cargo +nightly test` command depends on that to run.

![Code Coverage](https://github.com/uuosio/rscdk/blob/main/images/code-coverage.png)
