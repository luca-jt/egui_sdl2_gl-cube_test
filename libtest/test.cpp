#include "test.h"
#include <cstdint>


extern "C"
{
    int test_func()
    {
        static int32_t i = 0;
        return i++;
    }
}
