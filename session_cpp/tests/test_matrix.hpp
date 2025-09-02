#pragma once
#include <cmath>
#include <iomanip>

#include "core.hpp"  // Include the header of the class you're testing
#include "test_methods.hpp"

void test_matrix() {
    // Create a 3x4 matrix with specific values
    geo::Matrix m1 = {{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}};

    // Print the matrix
    my_assert(m1(0, 0) == 1 && m1(0, 1) == 2 && m1(0, 2) == 3 && m1(0, 3) == 4 && m1(1, 0) == 5 &&
              m1(1, 1) == 6 && m1(1, 2) == 7 && m1(1, 3) == 8 && m1(2, 0) == 9 && m1(2, 1) == 10 &&
              m1(2, 2) == 11 && m1(2, 3) == 12);
}

int test_matrix_main() {
    std::cout << "Running test_matrix..." << std::endl;
    test_matrix();
    return 0;
}