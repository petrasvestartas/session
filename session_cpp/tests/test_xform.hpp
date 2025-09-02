#pragma once
#include <cmath>
#include <iomanip>

#include "core.hpp"  // Include the header of the class you're testing
#include "test_methods.hpp"
#include "xform.hpp"

void test_translation() {
    geo::Matrix xform = geo::Xform::translation(1, 2, 3);
    geo::Point p{1, 2, 3};
    geo::Point p_transformed = geo::Xform::apply(xform, p);
    // geo::log(p_transformed.to_string());
    my_assert(p_transformed == geo::Point(2, 4, 6));
}

void test_scaling() {
    geo::Matrix xform = geo::Xform::scaling(1, 2, 3);
    geo::Point p{1, 2, 3};
    geo::Point p_transformed = geo::Xform::apply(xform, p);
    // geo::log(p_transformed.to_string());
    my_assert(p_transformed == geo::Point(1, 4, 9));
}

void test_change_of_basis() {
    geo::log("WARNING: test_change_of_basis method is not implemented!", geo::LogLevel::WARNING);
}

void test_xy_to_plane_transformation() {
    std::vector<geo::Point> points = {{0, 0, 0}, {1, 0, 0}, {0, 1, 0}, {0, 0, 1}};

    std::vector<geo::Point> result = {{0, 0, 0}, {0, 1, 0}, {0, 0, 1}, {1, 0, 0}};
    geo::Point p0{0, 0, 0};
    geo::Vector x_axis = geo::Vector(0, 1, 0);
    geo::Vector y_axis = geo::Vector(0, 0, 1);
    geo::Matrix xform = geo::Xform::xy_to_plane(p0, x_axis, y_axis);
    for (size_t i = 0; i < points.size(); i++) {
        geo::Point point_transformed = geo::Xform::apply(xform, points[i]);
        // geo::log(point_transformed.to_string());
        my_assert(point_transformed == result[i]);
    }
}

void test_plane_to_xy_transformation() {
    geo::Point p0{0, 0, 0};
    geo::Vector x_axis{0, 0, 1};
    geo::Vector y_axis{0, 1, 0};
    geo::Matrix xform = geo::Xform::plane_to_xy(p0, x_axis, y_axis);

    std::vector<geo::Point> points = {{0, 0, 0}, {0, 1, 0}, {0, 1, 1}, {0, 0, 1}};

    std::vector<geo::Point> result = {{0, 0, 0}, {0, 1, 0}, {1, 1, 0}, {1, 0, 0}};

    for (size_t i = 0; i < points.size(); i++) {
        geo::Point point_transformed = geo::Xform::apply(xform, points[i]);
        // geo::log(point_transformed.to_string());
        my_assert(point_transformed == result[i]);
    }
}

void test_plane_to_plane_transformation_xy_to_plane() {
    geo::Point p0{0, 0, 0};
    geo::Vector x_axis_0{1, 0, 0};
    geo::Vector y_axis_0{0, 1, 0};
    geo::Vector x_axis_1{0, 1, 0};
    geo::Vector y_axis_1{0, 0, 1};
    geo::Matrix xform = geo::Xform::plane_to_plane(p0, x_axis_0, y_axis_0, p0, x_axis_1, y_axis_1);

    std::vector<geo::Point> points = {{0, 0, 0}, {1, 0, 0}, {0, 1, 0}, {0, 0, 1}};

    std::vector<geo::Point> result = {{0, 0, 0}, {0, 1, 0}, {0, 0, 1}, {1, 0, 0}};

    for (size_t i = 0; i < points.size(); i++) {
        geo::Point point_transformed = geo::Xform::apply(xform, points[i]);
        // geo::log(point_transformed.to_string());
        my_assert(point_transformed == result[i]);
    }
}

void test_plane_to_plane_transformation_plane_to_xy() {
    geo::Point p0{0, 0, 0};
    geo::Vector x_axis_0{0, 0, 1};
    geo::Vector y_axis_0{0, 1, 0};
    geo::Vector x_axis_1{1, 0, 0};
    geo::Vector y_axis_1{0, 1, 0};
    geo::Matrix xform = geo::Xform::plane_to_plane(p0, x_axis_0, y_axis_0, p0, x_axis_1, y_axis_1);

    std::vector<geo::Point> points = {{0, 0, 0}, {0, 1, 0}, {0, 1, 1}, {0, 0, 1}};

    std::vector<geo::Point> result = {{0, 0, 0}, {0, 1, 0}, {1, 1, 0}, {1, 0, 0}};

    for (size_t i = 0; i < points.size(); i++) {
        geo::Point point_transformed = geo::Xform::apply(xform, points[i]);
        // geo::log(point_transformed.to_string());
        my_assert(point_transformed == result[i]);
    }
}

void test_plane_to_plane_xy_to_plane_to_xy() {
    geo::Point p0{0, 0, 0};
    geo::Vector x_axis_0{1, 0, 0};
    geo::Vector y_axis_0{0, 1, 0};
    geo::Vector x_axis_1{0, 1, 0};
    geo::Vector y_axis_1{0, 0, 1};
    geo::Matrix xform = geo::Xform::plane_to_plane(p0, x_axis_0, y_axis_0, p0, x_axis_1, y_axis_1);

    std::vector<geo::Point> points = {{0, 0, 0}, {1, 0, 0}, {0, 1, 0}, {0, 0, 1}};

    std::vector<geo::Point> result = {{0, 0, 0}, {0, 1, 0}, {0, 0, 1}, {1, 0, 0}};

    for (size_t i = 0; i < points.size(); i++) {
        geo::Point point_transformed = geo::Xform::apply(xform, points[i]);
        // geo::log(point_transformed.to_string());
        my_assert(point_transformed == result[i]);
        result[i] = point_transformed;
    }

    geo::Matrix xform2 = geo::Xform::plane_to_plane(p0, x_axis_1, y_axis_1, p0, x_axis_0, y_axis_0);

    for (size_t i = 0; i < result.size(); i++) {
        geo::Point point_transformed = geo::Xform::apply(xform2, result[i]);
        // geo::log(point_transformed.to_string());
        my_assert(point_transformed == points[i]);
    }
}

int test_xform_main() {
    std::cout << "Running test_xform..." << std::endl;
    test_translation();
    test_scaling();
    test_change_of_basis();
    test_xy_to_plane_transformation();
    test_plane_to_xy_transformation();
    test_plane_to_plane_transformation_xy_to_plane();
    test_plane_to_plane_transformation_plane_to_xy();
    test_plane_to_plane_xy_to_plane_to_xy();
    return 0;
}