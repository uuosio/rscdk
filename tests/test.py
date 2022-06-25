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

test_dir = os.path.dirname(__file__)
def deploy_contract(package_name):
    with open(f'{test_dir}/target/{package_name}/{package_name}.wasm', 'rb') as f:
        code = f.read()
    with open(f'{test_dir}/target/{package_name}/{package_name}.abi', 'rb') as f:
        abi = f.read()
    chain.deploy_contract('hello', code, abi)

def check_error_message(ex, err_msg):
    assert ex.value.args[0]['except']['stack'][0]['data']['s'] == err_msg

def run_test(action, args, err_msg=None, permission = None):
    if not permission:
        permission = {'hello': 'active'}

    if err_msg:
        with pytest.raises(Exception) as exc_info:
            r = chain.push_action('hello', action, args, permission)
            chain.produce_block()
        check_error_message(exc_info, err_msg)
    else:
        r = chain.push_action('hello', action, args, permission)
        chain.produce_block()

@chain_test
def test_hello():
    deploy_contract('hello')
    args = {
        'name': 'rust'
    }
    r = chain.push_action('hello', 'sayhello', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    chain.produce_block()

    r = chain.push_action('hello', 'sayhello', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    chain.produce_block()

@chain_test
def test_abi():
    deploy_contract('testabi')

    args = {
        "a1": True,
        "a2": -1,
        "a3": 0xff,
        "a4": -1,
        "a5": 0xffff,
        "a6": -1,
        "a7": 0xffffffff,
        "a8": -1,
        "a9": 0xffffffffffffffff,
        "a10": -1, # max 0x7fffffffffffffffffffffffffffffff
        "a11": "0xffffffffffffffffffffffffffffffff",
        # "a12": -1,
        "a13": 0xffffffff,
        "a14": 1.1,
        "a15": 2.2,
        "a16": "0xffffffffffffffffffffffffffffffff",
        "a17": '2021-09-03T04:13:21',
        "a18": '2021-09-03T04:13:21',
        "a19": {'slot': 193723200},
        "a20": "eosio",
        "a21": b"hello".hex(),
        "a22": "hello",
        "a23": 'bb' + 'aa'*19, #Checksum160, //checksum160,
        "a24": 'bb' + 'aa'*31, #Checksum256, //checksum256,
        "a25": 'bb' + 'aa'*63, #Checksum512, //checksum512,
        "a26": 'EOS5HoPaVaPivnVHsCvpoKZMmB6gcWGV5b3vF7S6pfsgFACzufMDy', # //public_key,
        "a27": 'SIG_K1_KbSF8BCNVA95KzR1qLmdn4VnxRoLVFQ1fZ8VV5gVdW1hLfGBdcwEc93hF7FBkWZip1tq2Ps27UZxceaR3hYwAjKL7j59q8', # //signature,
        "a28": '4,EOS',# a27 chain.Symbol, //symbol,
        "a29": 'EOS', # a28 chain.SymbolCode, //symbol_code,
        "a30": '1.0000 EOS', # a29 chain.Asset,
        "a31": ['1.0000 EOS', 'eosio.token'],
        # "a31": {'quantity': '1.0000 EOS', 'contract': 'eosio.token'}
    }

    r = chain.push_action('hello', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    chain.produce_block()

@chain_test
def test_mi():
    deploy_contract('testmi')

    args = {}
    r = chain.push_action('hello', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    chain.produce_block()

    r = chain.push_action('hello', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    chain.produce_block()

@chain_test
def test_mod():
    deploy_contract('testmod')
    args = {
        'name': 'rust'
    }
    r = chain.push_action('hello', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    chain.produce_block()

@chain_test
def test_inlineaction():
    deploy_contract('testinlineaction')
    args = {
        "name": "rust",
    }

    r = chain.push_action('hello', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    chain.produce_block()

@chain_test
def test_asset():
    deploy_contract('testasset')

    MAX_AMOUNT = (1 << 62) - 1

    bad_mini_amount = -MAX_AMOUNT - 1
    bad_max_amount = MAX_AMOUNT  + 1

    test_cases = [
        #test basic
        {
            'action': 'test',
            'args': {'a': '1.1234 EOS'},
            'err_msg': None,
        },
        #test Asset.unpack
        {
            'action': 'test2',
            'args': int.to_bytes(bad_max_amount, 8, 'little') + b'\x04EOS\x00\x00\x00\x00',
            'err_msg': "Asset.unpack: bad asset amount",
        },
        #test Asset.unpack
        {
            'action': 'test2',
            'args': int.to_bytes(bad_mini_amount & 0xffffffffffffffff, 8, 'little') + b'\x04EOS\x00\x00\x00\x00',
            'err_msg': "Asset.unpack: bad asset amount",
        },
        #test Asset.unpack
        {
            'action': 'test2',
            'args': int.to_bytes(MAX_AMOUNT, 8, 'little') + b'\x04EOS\x00\x00\x00E',
            'err_msg': "Symbol.unpack: bad symbol value",
        },
        #test Asset.from_string
        {
            'action': 'test3',
            'args': {'error_asset': "1123A.0 EOS"},
            'err_msg': "Asset.from_string: bad amount",
        },
        #test Asset.from_string
        {
            'action': 'test3',
            'args': {'error_asset': "11234.A EOS"},
            'err_msg': "Asset.from_string: bad amount",
        },
    ]
    for test in test_cases:
        run_test(**test)

@chain_test
def test_optional():
    deploy_contract('testoptional')
    args = dict(
        a1 = None,
        a2 = {"a2": {"a1": 123}},
        a3 =  {"a2": {"a1": 456}},
        a4 =  {"a2": {"a1": None}},
    )
    run_test('test', args)

@chain_test
def test_variant():
    deploy_contract('testvariant')

    args = dict(
        v = ['uint64', 10],
    )

    r = chain.push_action('hello', 'test', args, {'hello': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    chain.produce_block()
