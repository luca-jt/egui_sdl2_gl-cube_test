cmake_minimum_required(VERSION 3.17)
set(CMAKE_CXX_STANDARD 20)

#if (WIN32)
#    set(CMAKE_CXX_COMPILER "g++.exe")
#endif (WIN32)

project(test_cpp_lib)

add_subdirectory(lib2)

add_library(test STATIC test.h test.cpp)
install(TARGETS test DESTINATION .)

target_include_directories(test PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(test PROPERTIES LINKER_LANGUAGE CXX)
target_link_libraries(test sub)
