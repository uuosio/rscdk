ALL : build_all test_all

build_hello:
	cd hello;rust-contract build

build_abi:
	cd testabi;rust-contract build 

build_mi:
	cd testmi;rust-contract build 

build_mi2:
	cd testmi2;rust-contract build 

build_mod:
	cd testmod;rust-contract build 

build_inline:
	cd testinlineaction;rust-contract build 

build_asset:
	cd testasset;rust-contract build 

build_optional:
	cd testoptional;rust-contract build 

build_variant:
	cd testvariant;rust-contract build 

build_name:
	cd testname;rust-contract build 

build_transaction:
	cd testtransaction;rust-contract build 

test_hello: build_hello
	run-ipyeos -m pytest -s test.py -k test_hello

test_abi: build_abi
	run-ipyeos -m pytest -s test.py -k test_abi

test_mi: build_mi
	run-ipyeos -m pytest -s test.py -k test_mi

test_mi2: build_mi2
	run-ipyeos -m pytest -s test.py -k test_2mi

test_mod: build_mod
	run-ipyeos -m pytest -s test.py -k test_mod

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

test_transaction: build_transaction
	run-ipyeos -m pytest -s test.py -k test_transaction

build_all: build_hello build_abi build_mi build_mi2 build_mod \
	build_inline build_asset build_optional build_variant build_name test_transaction

test_all: build_all
	run-ipyeos -m pytest -s -x test.py
