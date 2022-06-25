ALL : test_hello test_abi test_mi test_mod \
	test_inline test_asset test_optional test_variant test_name

test_hello:
	cd hello;cargo eosiocontract build --release
	run-ipyeos -m pytest -s test.py -k test_hello

test_abi:
	cd testabi;cargo eosiocontract build --release
	run-ipyeos -m pytest -s test.py -k test_abi

test_mi:
	cd testmi;cargo eosiocontract build --release
	run-ipyeos -m pytest -s test.py -k test_mi

test_mod:
	cd testmod;cargo eosiocontract build --release
	run-ipyeos -m pytest -s test.py -k test_mi

test_inline:
	cd testinlineaction;cargo eosiocontract build --release
	run-ipyeos -m pytest -s test.py -k test_inlineaction

test_asset:
	cd testasset;cargo eosiocontract build --release
	run-ipyeos -m pytest -s test.py -k test_asset

test_optional:
	cd testoptional;cargo eosiocontract build --release
	run-ipyeos -m pytest -s test.py -k test_optional

test_variant:
	cd testvariant;cargo eosiocontract build --release
	run-ipyeos -m pytest -s test.py -k test_variant

test_name:
	cd testname;cargo eosiocontract build --release
	run-ipyeos -m pytest -s test.py -k test_name
