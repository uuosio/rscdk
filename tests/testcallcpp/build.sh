cdt-cpp -c -o add.o add.cpp
cdt-ar rcs libadd.a add.o
rust-contract build
