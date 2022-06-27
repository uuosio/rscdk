ALL : test_counter test_counter2 test_token

test_counter:
	cd counter;cargo eosiocontract build 
	run-ipyeos -m pytest -s test.py -k test_1counter

test_counter2:
	cd counter2;cargo eosiocontract build 
	run-ipyeos -m pytest -s test.py -k test_2counter

test_token:
	cd token;cargo eosiocontract build 
	run-ipyeos -m pytest -s test.py -k test_token
