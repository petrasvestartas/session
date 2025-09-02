#pragma once
#include <iostream>

#define my_assert(condition) my_assert_impl((condition), __FUNCTION__, __FILE__)

void my_assert_impl(bool condition, const char* function_name, const char* file_name) {
    if (!condition) {
        // Print in red
        std::cerr << "\033[41;37m" << "FAIL: " << function_name << " IN " << file_name << "\033[0m"
                  << std::endl;
    } else {
        // Print in cyan (light blue) background with black text
        std::cout << "\033[46;30m" << "PASS: " << function_name << " IN " << file_name << "\033[0m"
                  << std::endl;
    }
}