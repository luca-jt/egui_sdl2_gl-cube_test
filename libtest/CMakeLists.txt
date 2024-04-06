cmake_minimum_required(VERSION 3.17)
set(CMAKE_CXX_STANDARD 20)

project(test_cpp_lib)

add_library(test STATIC test.h test.cpp)
install(TARGETS test DESTINATION .)
