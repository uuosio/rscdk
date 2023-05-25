#include <string.h>
#include <stdint.h>
#include <eosio/eosio.hpp>

using namespace eosio;

extern "C" void say_hello(const char *name, uint32_t size) {
	uint64_t *ptr = new uint64_t(0);
    std::string _name(name, size);
    print_f("hello % %", name, uint64_t(ptr));
    delete ptr;
}
