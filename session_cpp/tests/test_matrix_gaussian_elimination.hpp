#pragma once
#include <cmath>
#include <array>
#include <limits>
#include "test_methods.hpp"
#include "matrix_gaussian_elimination.hpp"
#include "matrix.hpp"

void test_matrix_gaussian_elimination() {
    std::cout << "Testing Gaussian Elimination..." << std::endl;

    // Test 1: Simple 2x2 system
    // x + y = 3
    // 2x + y = 4
    // Solution: x = 1, y = 2
    geo::Matrix matrix1{{1, 1},
                       {2, 1}};
    std::vector<double> b1{3, 4};
    auto result1 = geo::gauss_partial(matrix1, b1);
    my_assert(std::abs(result1[0] - 1.0) < 1e-10 && std::abs(result1[1] - 2.0) < 1e-10);

    // Test 2: 3x3 system
    // 2x + y - z = 8
    // -3x - y + 2z = -11
    // -2x + y + 2z = -3
    // Solution: x = 2, y = 3, z = -1
    geo::Matrix matrix2{{2, 1, -1},
                       {-3, -1, 2},
                       {-2, 1, 2}};
    std::vector<double> b2{8, -11, -3};
    auto result2 = geo::gauss_partial(matrix2, b2);
    my_assert(std::abs(result2[0] - 2.0) < 1e-10 && 
              std::abs(result2[1] - 3.0) < 1e-10 && 
              std::abs(result2[2] - (-1.0)) < 1e-10);

    // Test 3: Singular matrix (no unique solution)
    // x + y = 1
    // x + y = 2
    geo::Matrix matrix3{{1, 1},
                       {1, 1}};
    std::vector<double> b3{1, 2};
    bool caught_exception = false;
    try {
        auto result3 = geo::gauss_partial(matrix3, b3);
    } catch (const std::runtime_error& e) {
        caught_exception = true;
    }
    my_assert(caught_exception);

    // Test 4: Zero pivot test with row swapping
    // 0x + y = 1
    // x + y = 2
    geo::Matrix matrix4{{0, 1},
                       {1, 1}};
    std::vector<double> b4{1, 2};
    auto result4 = geo::gauss_partial(matrix4, b4);
    my_assert(std::abs(result4[0] - 1.0) < 1e-10 && std::abs(result4[1] - 1.0) < 1e-10);

    std::cout << "All Gaussian Elimination tests passed!" << std::endl;
}

int test_matrix_gaussian_elimination_main() {
    std::cout << "Running test_matrix_gaussian_elimination..." << std::endl;
    test_matrix_gaussian_elimination();
    return 0;
}