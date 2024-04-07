#include <iostream>
#include "test.h"

extern "C"
{
    int test_func()
    {
        return 12345;
    }
}
