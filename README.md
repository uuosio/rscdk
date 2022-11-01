![build](https://github.com/uuosio/rscdk/actions/workflows/pr-any.yml/badge.svg?event=push)
[![codecov](https://codecov.io/gh/uuosio/rscdk/branch/main/graph/badge.svg?token=GG76LHX28E)](https://codecov.io/gh/uuosio/rscdk)


Rust Smart Contracts Development Kit

## Getting Started

- [try rust smart contracts](https://colab.research.google.com/github/uuosio/rscdk/blob/master/quickstart/quick-start.ipynb)
- [try examples](https://github.com/uuosio/rscdk/tree/main/examples)
- [read the rscdk book](https://uuosio.github.io/rscdk-book)

## Getting involved

- [submit an issue](https://github.com/uuosio/rscdk/issues)
- [create a new pull request](https://github.com/uuosio/rscdk/pulls)
- [help improve the RSCDK book](https://github.com/uuosio/rscdk-book)

## Debugging

Install `ipyeos` first for debugging.

```bash
python3 -m pip install ipyeos
```

In order to update to a new version, use the following command:

```bash
python3 -m pip install -U ipyeos
```

Then run the debugging server:

```bash
eosdebugger
```

On Windows, you need to use a docker image to run a debugging server.

```bash
docker pull ghcr.io/uuosio/ipyeos:latest
```

Run the debugging server on the Windows platform:

```bash
docker run -it --rm -p 9090:9090 -p 9092:9092 -t ghcr.io/uuosio/ipyeos
```

Build contract

```bash
rust-contract build
```

Start debugging:

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
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off"
cargo +nightly test
grcov . -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
```

You will need to start `eosdebugger` first if you didn't do that. `cargo +nightly test` command depends on that to run.

![Code Coverage](https://github.com/uuosio/rscdk/blob/main/images/code-coverage.png)
