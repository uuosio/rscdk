ALL : test_hello test_abi test_mi

test_hello:
	cd hello;cargo eosiocontract build --release
	run-ipyeos -m pytest -s test.py -k test_hello

test_abi:
	cd testabi;cargo eosiocontract build --release
	run-ipyeos -m pytest -s test.py -k test_abi

test_mi:
	cd testmi;cargo eosiocontract build --release
	run-ipyeos -m pytest -s test.py -k test_mi
