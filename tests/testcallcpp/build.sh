cdt-cpp -c -o say_hello.o say_hello.cpp
cdt-ar rcs libsay_hello.a say_hello.o
rust-contract build
