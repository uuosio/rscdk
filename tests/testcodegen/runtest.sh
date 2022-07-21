#python3 -m pip install pytest-xdist
ipyeos -m pytest -n 2 -s -x test.py
#ipyeos -m pytest -n 2 -s -x test.py -k test_bad_struct_name

