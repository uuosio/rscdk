#include <stdint.h>
#include <eosio/eosio.hpp>

using namespace eosio;

extern "C" void say_hello(const char *name, uint32_t size) {
    print("hello ", std::string(name, size));
}
