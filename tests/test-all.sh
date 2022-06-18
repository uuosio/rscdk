# pushd testcodegen
# run-ipyeos -m pytest -n 2 -s -x test.py || exit 1
# popd

pushd hello
make test
popd
