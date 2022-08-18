import os
import sys
import json
import struct
import pytest
import subprocess
import shlex
import shutil
import tempfile
import threading

test_dir = os.path.dirname(__file__)
sys.path.append(os.path.join(test_dir, '..'))

from ipyeos import log
from ipyeos import chaintester
from ipyeos.chaintester import ChainTester

chaintester.chain_config['contracts_console'] = True

logger = log.get_logger(__name__)

def update_auth(chain, account):
    a = {
        "account": account,
        "permission": "active",
        "parent": "owner",
        "auth": {
            "threshold": 1,
            "keys": [
                {
                    "key": 'EOS6AjF6hvF7GSuSd4sCgfPKq5uWaXvGM2aQtEUCwmEHygQaqxBSV',
                    "weight": 1
                }
            ],
            "accounts": [{"permission":{"actor":account,"permission": 'eosio.code'}, "weight":1}],
            "waits": []
        }
    }
    chain.push_action('eosio', 'updateauth', a, {account:'active'})

def init_chain():
    chain = chaintester.ChainTester()
    update_auth(chain, 'hello')
    update_auth(chain, 'alice')
    return chain

chain = None
def chain_test(fn):
    def call(*args, **vargs):
        global chain
        chain = init_chain()
        ret = fn(*args, **vargs)
        chain.free()
        chain = None
        return ret
    return call

class NewChain():
    def __init__(self):
        self.chain = None

    def __enter__(self):
        self.chain = init_chain()
        return self.chain

    def __exit__(self, type, value, traceback):
        self.chain.free()

@chain_test
def test_hello():
    print('+++++++test_hello', file=sys.stderr)
    # assert False, 'oops!'
    return
    with open('./target/wasm32-wasi/release/hello.wasm', 'rb') as f:
        code = f.read()
    with open('./target/hello.abi', 'rb') as f:
        abi = f.read()
    chain.deploy_contract('hello', code, abi)

    args = {
        'name': 'rust'
    }
    r = chain.push_action('hello', 'sayhello', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    chain.produce_block()

    r = chain.push_action('hello', 'sayhello', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    chain.produce_block()

toml = '''
[package]
name = "test"
version = "0.1.0"
authors = [""]
edition = "2021"

[dependencies]
rust_chain= { version = "0.1.0", path = "%s", default-features = false }

[lib]
name = "test"
path = "lib.rs"
crate-type = ["cdylib", "rlib"]

[profile.release]
panic = "abort"
lto = true

[features]
default = ["std"]
std = [
    "rust_chain/std",
]

# [workspace]
# members = ["abigen"]
'''

def run_test(code, error_message):
    temp_dir = tempfile.mkdtemp()
    try:
        test_dir = os.path.dirname(__file__)
        with open(os.path.join(temp_dir, 'Cargo.toml'), 'w') as f:
            f.write(toml%(f'{test_dir}/../../crates/rust-chain',))

        with open(os.path.join(temp_dir, 'lib.rs'), 'w') as f:
            f.write(code)

        os.environ['RUSTFLAGS'] = "-C link-arg=-zstack-size=8192 -Clinker-plugin-lto"
        os.chdir(temp_dir)
        cmd = f'cargo +nightly build --target=wasm32-wasi -Zbuild-std --no-default-features --release -Zbuild-std-features=panic_immediate_abort'
        cmd = shlex.split(cmd)
        # process = subprocess.run(cmd)
        process = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        output = process.stderr.decode()
        print(output, file=sys.stderr)
        assert error_message in output, 'bad output'
    finally:
        shutil.rmtree(temp_dir)

def test_bad_struct_name():
    return
    code = '''
#![cfg_attr(not(feature = "std"), no_std)]
use rust_chain as chain;
#[chain::contract]
mod hello {
    struct AAA_ {
        value: u64,
    }
}
'''
    error_message = '''
error: structs with `_` in name are not supported by contract
 --> lib.rs:6:5
  |
6 | /     struct AAA_ {
7 | |         value: u64,
8 | |     }
  | |_____^
'''
    run_test(code, error_message)
