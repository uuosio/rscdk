ALL : test_all

build_notify:
	cd testnotify/receiver;rust-contract build 
	cd testnotify/sender;rust-contract build

test_all: build_notify
	rust-contract build
	cargo test -- --test-threads=1
