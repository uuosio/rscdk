ALL : test_all

build_all:
	rust-contract build

build_hello:
	cd hello;rust-contract build

build_abi:
	cd testabi;rust-contract build 

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

build_notify:
	cd testnotify/receiver;rust-contract build 
	cd testnotify/sender;rust-contract build 

test_hello: build_hello
	ipyeos -m pytest -s test.py -k test_hello

test_abi: build_abi
	ipyeos -m pytest -s test.py -k test_abi

test_mi: build_all
	ipyeos -m pytest -s test.py -k test_mi

test_mi2: build_all
	ipyeos -m pytest -s test.py -k test_2mi

test_mod: build_mod
	ipyeos -m pytest -s test.py -k test_mod

test_inline: build_inline
	ipyeos -m pytest -s test.py -k test_inlineaction

test_asset: build_asset
	ipyeos -m pytest -s test.py -k test_asset

test_optional: build_optional
	ipyeos -m pytest -s test.py -k test_optional

test_variant: build_variant
	ipyeos -m pytest -s test.py -k test_variant

test_name: build_all
	ipyeos -m pytest -s test.py -k test_name

test_transaction: build_all
	ipyeos -m pytest -s test.py -k test_transaction

test_binaryextension: build_all
	ipyeos -m pytest -s test.py -k test_binaryextension

test_notify: build_notify
	ipyeos -m pytest -s test.py -k test_notify

test_destructor: build_all
	ipyeos -m pytest -s test.py -k test_destructor

all_builds = build_hello build_abi build_mod \
	build_inline build_asset build_optional build_variant \
	build_notify

test_all: $(all_builds)
	rust-contract build
	ipyeos -m pytest -s -x test.py
