add_library(sub STATIC constant.h constant.cpp)
install(TARGETS sub DESTINATION .)

target_include_directories(sub PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(sub PROPERTIES LINKER_LANGUAGE CXX)
