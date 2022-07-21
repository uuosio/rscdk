ALL : test_all

build_counter:
	cd counter;rust-contract build

build_counter2:
	cd counter2;rust-contract build

build_token:
	cd token;rust-contract build

build_inlineaction:
	cd inlineaction;rust-contract build

build_notify:
	cd notify/sender;rust-contract build
	cd notify/receiver;rust-contract build

build_globalstates:
	cd globalstates;rust-contract build

build_secondaryindex:
	cd secondaryindex;rust-contract build

build_helloworld:
	cd helloworld;rust-contract build

build_dbi64:
	cd dbi64;rust-contract build


test_counter:
	cd counter;rust-contract build
	ipyeos -m pytest -s test.py -k test_1counter

test_counter2:
	cd counter2;rust-contract build
	ipyeos -m pytest -s test.py -k test_2counter

test_token:
	cd token;rust-contract build
	ipyeos -m pytest -s test.py -k test_token

test_inlineaction:
	cd inlineaction;rust-contract build
	ipyeos -m pytest -s test.py -k test_inlineaction

test_notify:
	cd notify/sender;rust-contract build
	cd notify/receiver;rust-contract build
	ipyeos -m pytest -s test.py -k test_notify

test_globalstates:
	cd globalstates;rust-contract build
	ipyeos -m pytest -s test.py -k test_globalstates

test_secondaryindex:
	cd secondaryindex;rust-contract build
	ipyeos -m pytest -s test.py -k test_secondaryindex

test_helloworld:
	cd helloworld;rust-contract build
	ipyeos -m pytest -s test.py -k test_helloworld

test_dbi64:
	cd dbi64;rust-contract build
	ipyeos -m pytest -s test.py -k test_dbi64

all_builds = build_counter build_counter2 build_token build_inlineaction \
	build_notify build_globalstates build_secondaryindex build_helloworld build_dbi64

test_all: $(all_builds)
	ipyeos -m pytest -s -x test.py
