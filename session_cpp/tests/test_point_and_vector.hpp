#pragma once
#include <cmath>
#include <iomanip>

#include "core.hpp"  // Include the header of the class you're testing
#include "point_and_vector.hpp"

void test_moment_varignon() {
    // Varignon theorem: M = sign_x*(Fx * y) + sign_y*(Fy * x)
    // Test case 1
    geo::Point p(5, 2, 0);
    geo::Vector v(4, -3, 0);
    v.rescale(100);
    // Calculate length but use it in a comment to avoid warning
    double length = v.length();
    std::cout << "Vector length after rescaling: " << length << std::endl;
    double moment = geo::moment_varignon(p, v);
    my_assert(std::abs(moment + 460) < geo::GLOBALS::ZERO_TOLERANCE);

    // Test case 2
    p = geo::Point(0.4 + 0.3 * cos(45 * geo::GLOBALS::TO_RAD), 0.3 * sin(45 * geo::GLOBALS::TO_RAD),
                   0);
    v = geo::Vector(300 * cos(30 * geo::GLOBALS::TO_RAD), 300 * sin(30 * geo::GLOBALS::TO_RAD), 0);
    moment = geo::moment_varignon(p, v);
    my_assert(std::abs(moment - 36.70628594077311390720) < geo::GLOBALS::ZERO_TOLERANCE);

    // Test case 3
    p = geo::Point(4 + 3 * cos(45 * geo::GLOBALS::TO_RAD) - 1, 3 * sin(45 * geo::GLOBALS::TO_RAD),
                   0);
    v = geo::Vector(0, 600, 0);
    moment = geo::moment_varignon(p, v);
    my_assert(std::abs(moment - 3072.79220613578581833281) < geo::GLOBALS::ZERO_TOLERANCE);

    // Test case 4
    p = geo::Point(0.1 + 0.2 * cos(45 * geo::GLOBALS::TO_RAD) + 0.1,
                   0.2 * sin(45 * geo::GLOBALS::TO_RAD), 0);
    v = geo::Vector(50 * cos(60 * geo::GLOBALS::TO_RAD), 50 * sin(60 * geo::GLOBALS::TO_RAD), 0);
    moment = geo::moment_varignon(p, v);
    my_assert(std::abs(moment - 11.2484) < 0.0001);

    // Test case 5
    p = geo::Point(2.5 * cos(30 * geo::GLOBALS::TO_RAD) - 0.25 * cos(60 * geo::GLOBALS::TO_RAD),
                   2.5 * sin(30 * geo::GLOBALS::TO_RAD) + 0.25 * sin(60 * geo::GLOBALS::TO_RAD), 0);
    v = geo::Vector(-600 * cos(20 * geo::GLOBALS::TO_RAD), 600 * sin(20 * geo::GLOBALS::TO_RAD), 0);
    moment = geo::moment_varignon(p, v);
    my_assert(std::abs(moment - 1245.48) < 0.01);

    // Test case 6
    p = geo::Point(3 + 3 * cos(45 * geo::GLOBALS::TO_RAD), 3 * sin(45 * geo::GLOBALS::TO_RAD), 0);
    v = geo::Vector(500 * cos(45 * geo::GLOBALS::TO_RAD), 500 * sin(45 * geo::GLOBALS::TO_RAD), 0);
    moment = geo::moment_varignon(p, v);
    my_assert(std::abs(moment - 1060.660) < 0.01);
}

void test_sum_moment_varignon() {
    // Varignon theorem: M = sum(sign_x*(Fx * y) + sign_y*(Fy * x))

    // Test case 1
    std::vector<geo::Point> points = {geo::Point(1, 0, 0),
                                      geo::Point(3 + 2.5 * cos(45 * geo::GLOBALS::TO_RAD),
                                                 2.5 * sin(45 * geo::GLOBALS::TO_RAD), 0)};

    std::vector<geo::Vector> forces = {geo::Vector(0, -600, 0), geo::Vector(300, 500, 0)};

    double moment = geo::moments_varignon_sum(points, forces);
    my_assert(std::abs(moment - 1253.553) < 0.001);

    // Test case 2
    points = {geo::Point(0.125 + 0.3, 0.25, 0), geo::Point(0.125 + 0.3, 0.25, 0)};

    forces = {geo::Vector(0.8 * 500, 0.6 * 500, 0),
              geo::Vector(cos(60 * geo::GLOBALS::TO_RAD) * 600,
                          -sin(60 * geo::GLOBALS::TO_RAD) * 600, 0)};

    moment = geo::moments_varignon_sum(points, forces);
    my_assert(std::abs(moment + 268.336) < 0.001);

    // Test case 3
    points = {
        geo::Point(-3 - 3 * sin(30 * geo::GLOBALS::TO_RAD), 3 * cos(30 * geo::GLOBALS::TO_RAD), 0),
        geo::Point(-3 - 3 * sin(30 * geo::GLOBALS::TO_RAD), 3 * cos(30 * geo::GLOBALS::TO_RAD), 0)};

    forces = {geo::Vector(-200, 0, 0), geo::Vector(300 * sin(30 * geo::GLOBALS::TO_RAD),
                                                   -300 * cos(30 * geo::GLOBALS::TO_RAD), 0)};

    moment = geo::moments_varignon_sum(points, forces);
    my_assert(std::abs(moment - 1299.04) < 0.01);
}

int test_point_and_vector_main() {
    std::cout << "Running test_point_and_vector..." << std::endl;
    test_moment_varignon();
    test_sum_moment_varignon();
    return 0;
}