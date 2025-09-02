#pragma once
#include <cmath>
#include <iomanip>

#include "core.hpp"  // Include the header of the class you're testing
#include "intersection.hpp"
#include "test_methods.hpp"

void test_line_plane_intersection() {
    // Create a line
    geo::Line line = geo::Line(geo::Point(0, 0, 0), geo::Point(1, 1, 1));

    // Create a plane
    geo::Plane plane = geo::Plane(geo::Point(0, 0, 0.5), geo::Vector(0, 0, 1));

    // Intersect
    geo::Point output;
    // Store result but use it to avoid warning
    bool result = geo::Intersection::line_plane(line, plane, output, false);
    std::string status = result ? "success" : "failed";
    geo::log("Line-plane intersection calculated: " + status, 
             result ? geo::LogLevel::INFO : geo::LogLevel::WARNING);
    geo::log("WARNING: no tests implemented!", geo::LogLevel::WARNING);
}

int test_intersection_main() {
    std::cout << "Running test_intersection..." << std::endl;
    test_line_plane_intersection();
    return 0;
}