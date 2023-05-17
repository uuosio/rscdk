mkdir -p say_hello/build
pushd say_hello/build
cmake -DCMAKE_TOOLCHAIN_FILE=`cdt-get-dir`/CDTWasmToolchain.cmake ..
make
popd
rust-contract build
