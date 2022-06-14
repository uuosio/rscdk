import os
import sys
import json
import struct
import pytest

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
    with open('../target/wasm32-wasi/release/hello.wasm', 'rb') as f:
        code = f.read()
    # with open('./target/hello.abi', 'rb') as f:
    #     abi = f.read()
    abi = b''
    chain.deploy_contract('hello', code, abi, 0)

    r = chain.push_action('hello', 'sayhello', {}, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    chain.produce_block()

    r = chain.push_action('hello', 'sayhello', {}, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    chain.produce_block()
