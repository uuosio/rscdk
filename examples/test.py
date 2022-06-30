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

@chain_test
def test_1counter():
    deploy_contract('counter')

    args = {}
    r = chain.push_action('hello', 'inc', args)
    logger.info('+++++++create elapsed: %s', r['elapsed'])
    chain.produce_block()

    r = chain.push_action('hello', 'inc', args)
    logger.info('+++++++create elapsed: %s', r['elapsed'])
    chain.produce_block()

@chain_test
def test_2counter():
    deploy_contract('counter2')

    args = {}
    r = chain.push_action('hello', 'inc', args)
    logger.info('+++++++create elapsed: %s', r['elapsed'])
    chain.produce_block()

    r = chain.push_action('hello', 'inc', args)
    logger.info('+++++++create elapsed: %s', r['elapsed'])
    chain.produce_block()

@chain_test
def test_token():
    deploy_contract('token')
    create = {
        "issuer": "hello",
        "maximum_supply": "100.0000 EOS",
    }
    r = chain.push_action('hello', 'create', create)
    logger.info('+++++++create elapsed: %s', r['elapsed'])
    chain.produce_block()

    r = chain.get_table_rows(True, 'hello', 'EOS', 'stat', "", "", 1)
    logger.info(r)
    assert r['rows'][0]['issuer'] == 'hello'
    assert r['rows'][0]['max_supply'] == '100.0000 EOS'
    assert r['rows'][0]['supply'] == '0.0000 EOS'

    try:
        r = chain.push_action('hello', 'create', create)
    except Exception as e:
        error_msg = e.args[0]['action_traces'][0]['except']['stack'][0]['data']['s']
        assert error_msg == 'token with symbol already exists'
        # logger.info(json.dumps(e.args[0], indent='    '))

    #test issue

    issue = {'to': 'hello', 'quantity': '1.0000 EOS', 'memo': 'issue to alice'}
    r = chain.push_action('hello', 'issue', issue)
    logger.info('+++++++issue elapsed: %s', r['elapsed'])
    chain.produce_block()

    r = chain.get_table_rows(True, 'hello', 'EOS', 'stat', "", "", 1)
    logger.info(r)
    assert r['rows'][0]['issuer'] == 'hello'
    assert r['rows'][0]['max_supply'] == '100.0000 EOS'
    assert r['rows'][0]['supply'] == '1.0000 EOS'

    r = chain.get_table_rows(True, 'hello', 'hello', 'accounts', "", "", 1)
    logger.info(r)
    assert r['rows'][0]['balance'] == '1.0000 EOS'

    try:
        issue = {'to': 'eosio', 'quantity': '1.0000 EOS', 'memo': 'issue to alice'}
        chain.push_action('hello', 'issue', issue)
    except Exception as e:
        error_msg = e.args[0]['action_traces'][0]['except']['stack'][0]['data']['s']
        assert error_msg == 'tokens can only be issued to issuer account'

    #test transfer
    transfer = {'from': 'hello', 'to': 'alice', 'quantity': '1.0000 EOS', 'memo': 'transfer from alice'}
    r = chain.push_action('hello', 'transfer', transfer)
    logger.info('+++++++transfer elapsed: %s', r['elapsed'])

    chain.produce_block()

    r = chain.get_table_rows(True, 'hello', 'hello', 'accounts', "", "", 1)
    logger.info(r)
    assert r['rows'][0]['balance'] == '0.0000 EOS'

    r = chain.get_table_rows(True, 'hello', 'alice', 'accounts', "", "", 1)
    logger.info(r)
    assert r['rows'][0]['balance'] == '1.0000 EOS'

    # transfer back
    transfer = {'from': 'alice', 'to': 'hello', 'quantity': '1.0000 EOS', 'memo': 'transfer back'}
    r = chain.push_action('hello', 'transfer', transfer, {'alice': 'active'})
    logger.info('+++++++transfer elapsed: %s', r['elapsed'])
    chain.produce_block()

    #quantity chain.Asset, memo
    retire = {'quantity': '1.0000 EOS', 'memo': 'retire 1.0000 EOS'}
    r = chain.push_action('hello', 'retire', retire)
    logger.info('+++++++retire elapsed: %s', r['elapsed'])

    r = chain.get_table_rows(True, 'hello', 'hello', 'accounts', "", "", 1)
    assert r['rows'][0]['balance'] == '0.0000 EOS'

    r = chain.get_table_rows(True, 'hello', 'EOS', 'stat', "", "", 1)
    logger.info(r)
    assert r['rows'][0]['supply'] == '0.0000 EOS'


    r = chain.get_table_rows(True, 'hello', 'helloworld11', 'accounts', "", "", 1)
    assert len(r['rows']) == 0

    #owner chain.Name, symbol chain.Symbol, ram_payer chain.Name
    #test open
    open_action = {'owner': 'helloworld11', 'symbol': '4,EOS', 'ram_payer': 'hello'}
    r = chain.push_action('hello', 'open', open_action)
    logger.info('+++++++open elapsed: %s', r['elapsed'])

    r = chain.get_table_rows(True, 'hello', 'helloworld11', 'accounts', "", "", 1)
    assert r['rows'][0]['balance'] == '0.0000 EOS'

    #test close
    close_action = {'owner': 'helloworld11', 'symbol': '4,EOS'}
    r = chain.push_action('hello', 'close', close_action, {'helloworld11': 'active'})
    logger.info('+++++++close elapsed: %s', r['elapsed'])
    chain.produce_block()

    r = chain.get_table_rows(True, 'hello', 'helloworld11', 'accounts', "", "", 1)
    assert len(r['rows']) == 0

@chain_test
def test_inlineaction():
    deploy_contract('inlineaction')

    args = {'name': 'bob'}
    r = chain.push_action('hello', 'sayhello', args)
    logger.info('+++++++create elapsed: %s', r['elapsed'])
    chain.produce_block()

@chain_test
def test_notify():
    # info = chain.get_account('helloworld11')
    # logger.info(info)
    with open('./notify/sender/target/sender.wasm', 'rb') as f:
        code = f.read()
    with open('./notify/sender/target/sender.abi', 'rb') as f:
        abi = f.read()
    chain.deploy_contract('alice', code, abi, 0)

    with open('./notify/receiver/target/receiver.wasm', 'rb') as f:
        code = f.read()
    with open('./notify/receiver/target/receiver.abi', 'rb') as f:
        abi = f.read()
    chain.deploy_contract('hello', code, abi, 0)

    args = {
        "name": "rust",
    }

    r = chain.push_action('alice', 'test', args, {'alice': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    chain.produce_block()

@chain_test
def test_globalstates():
    deploy_contract('globalstates')
    args = {}
    r = chain.push_action('hello', 'inc', args)
    logger.info('+++++++create elapsed: %s', r['elapsed'])
    chain.produce_block()


    r = chain.push_action('hello', 'inc', args)
    logger.info('+++++++create elapsed: %s', r['elapsed'])
    chain.produce_block()

@chain_test
def test_secondaryindex():
    deploy_contract('secondaryindex')
    args = {
        'key': 1,
        'value': 11,
    }
    r = chain.push_action('hello', 'test1', args)
    logger.info('+++++++create elapsed: %s', r['elapsed'])
    chain.produce_block()

    args = {
        'key': 2,
        'value': 22,
    }
    r = chain.push_action('hello', 'test1', args)
    logger.info('+++++++create elapsed: %s', r['elapsed'])
    chain.produce_block()

    args = {
        'value': 22,
    }
    r = chain.push_action('hello', 'test2', args)
    logger.info('+++++++create elapsed: %s', r['elapsed'])
    chain.produce_block()

    args = {
        'value': 23,
    }
    r = chain.push_action('hello', 'test2', args)
    logger.info('+++++++create elapsed: %s', r['elapsed'])
    chain.produce_block()

@chain_test
def test_helloworld():
    deploy_contract('helloworld')
    args = {}
    r = chain.push_action('hello', 'sayhello', args)
    logger.info('+++++++create elapsed: %s', r['elapsed'])
    chain.produce_block()
