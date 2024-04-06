#include <iostream>
#include "test.h"

extern "C"
{
    void test_func()
    {
        std::cout << "test success" << std::endl;
    }
}
