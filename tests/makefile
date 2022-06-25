ALL : build_all test_all

build_hello:
	cd hello;cargo eosiocontract build --release

build_abi:
	cd testabi;cargo eosiocontract build --release

build_mi:
	cd testmi;cargo eosiocontract build --release

build_mod:
	cd testmod;cargo eosiocontract build --release

build_inline:
	cd testinlineaction;cargo eosiocontract build --release

build_asset:
	cd testasset;cargo eosiocontract build --release

build_optional:
	cd testoptional;cargo eosiocontract build --release

build_variant:
	cd testvariant;cargo eosiocontract build --release

build_name:
	cd testname;cargo eosiocontract build --release


test_hello: build_hello
	run-ipyeos -m pytest -s test.py -k test_hello

test_abi: build_abi
	run-ipyeos -m pytest -s test.py -k test_abi

test_mi: build_mi
	run-ipyeos -m pytest -s test.py -k test_mi

test_mod: build_mod
	run-ipyeos -m pytest -s test.py -k test_mi

test_inline: build_inline
	run-ipyeos -m pytest -s test.py -k test_inlineaction

test_asset: build_asset
	run-ipyeos -m pytest -s test.py -k test_asset

test_optional: build_optional
	run-ipyeos -m pytest -s test.py -k test_optional

test_variant: build_variant
	run-ipyeos -m pytest -s test.py -k test_variant

test_name: build_name
	run-ipyeos -m pytest -s test.py -k test_name

build_all: build_hello build_abi build_mi build_mod \
	build_inline build_asset build_optional build_variant build_name

test_all: build_all
	run-ipyeos -m pytest -s test.py
