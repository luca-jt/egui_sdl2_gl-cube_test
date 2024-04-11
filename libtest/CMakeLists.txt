cmake_minimum_required(VERSION 3.17)
set(CMAKE_CXX_STANDARD 20)

project(test_cpp_lib)

add_subdirectory(SDL EXCLUDE_FROM_ALL)

add_library(test STATIC test.h test.cpp)
install(TARGETS test DESTINATION .)
target_include_directories(test PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(test PROPERTIES LINKER_LANGUAGE CXX)

# SDL2::SDL2main may or may not be available. It is e.g. required by Windows GUI applications
if(TARGET SDL2::SDL2main)
    # It has an implicit dependency on SDL2 functions, so it MUST be added before SDL2::SDL2 (or SDL2::SDL2-static)
    target_link_libraries(test PRIVATE SDL2::SDL2main)
endif()

# Link to the actual SDL2 library. SDL2::SDL2 is the shared SDL library, SDL2::SDL2-static is the static SDL libarary.
target_link_libraries(test PRIVATE SDL2::SDL2-static)
