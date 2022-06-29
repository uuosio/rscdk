ALL : test_counter test_counter2 test_token test_inlineaction

test_counter:
	cd counter;rust-contract build 
	run-ipyeos -m pytest -s test.py -k test_1counter

test_counter2:
	cd counter2;rust-contract build 
	run-ipyeos -m pytest -s test.py -k test_2counter

test_token:
	cd token;rust-contract build 
	run-ipyeos -m pytest -s test.py -k test_token

test_inlineaction:
	cd inlineaction;rust-contract build 
	run-ipyeos -m pytest -s test.py -k test_inlineaction
