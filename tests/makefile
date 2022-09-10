ALL : test_all

build_all:
	rust-contract build-contract

build_hello:
	cd hello;rust-contract build-contract

build_abi:
	cd testabi;rust-contract build-contract

build_mod:
	cd testmod;rust-contract build-contract 

build_inline:
	cd testinlineaction;rust-contract build-contract 

build_asset:
	cd testasset;rust-contract build-contract 

build_optional:
	cd testoptional;rust-contract build-contract 

build_variant:
	cd testvariant;rust-contract build-contract 

build_crypto:
	cd testcrypto;rust-contract build-contract 

build_serializer:
	cd testserializer;rust-contract build-contract 

build_intrinsics:
	cd testintrinsics;rust-contract build-contract 

build_notify:
	cd testnotify/receiver;rust-contract build 
	cd testnotify/sender;rust-contract build

all_builds = build_hello build_abi build_mod \
	build_inline build_asset build_optional build_variant \
	build_notify build_crypto build_serializer build_intrinsics

test_all: $(all_builds)
	rust-contract build
	cargo test -- --test-threads=1
