cmake_minimum_required(VERSION 3.5)

project(cdt_libraries)

# list(APPEND CMAKE_MODULE_PATH ${CDT_BIN})
# include(CDTMacros)

set(EOSIO_WASM_OLD_BEHAVIOR "Off")
find_package(cdt)


set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O3 -Wall")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O3 -Wall -no-missing-ricardian-clause")

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_EXTENSIONS ON)


add_library(say_hello
    say_hello.cpp
)

target_include_directories(say_hello PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}
)
