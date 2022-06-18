#python3 -m pip install pytest-xdist
run-ipyeos -m pytest -n 2 -s -x test.py
#run-ipyeos -m pytest -n 2 -s -x test.py -k test_bad_struct_name

