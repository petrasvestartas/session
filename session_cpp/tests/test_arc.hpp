#pragma once
#include <cmath>
#include <iomanip>

#include "arc.hpp"
#include "core.hpp"  // Include the header of the class you're testing
#include "offset_2d.hpp"
#include "point_and_vector.hpp"
#include "test_methods.hpp"

void test_arc() {
    // Three point arc
    geo::Point p0(-4, 0, 0);
    geo::Point p1(0, 0.4, 0);
    geo::Point p2(4, 0, 0);
    geo::Arc arc(p0, p1, p2);

    // Divide arc into points
    std::vector<geo::Point> points = arc.divide_arc_into_points(3);

    // Offset polyline
    std::vector<geo::Point> offset = geo::offset_polyline(points, 0.12);

    // Create rectangular polygons
    std::vector<std::vector<geo::Point>> polygons;
    for (size_t i = 0; i < points.size() - 1; ++i) {
        std::vector<geo::Point> polygon = {points[i], points[i + 1], offset[i + 1], offset[i]};
        polygons.push_back(polygon);
    }

    // Assert the results
    std::vector<geo::Point> expected_points = {
        geo::Point(-4.000000, 0.000000, 0.000000), geo::Point(-1.341217, 0.355424, 0.000000),
        geo::Point(-1.349202, 0.475424, 0.000000), geo::Point(-4.015900, 0.118942, 0.000000),
        geo::Point(-1.341217, 0.355424, 0.000000), geo::Point(1.341217, 0.355424, 0.000000),
        geo::Point(1.349202, 0.475424, 0.000000),  geo::Point(-1.349202, 0.475424, 0.000000),
        geo::Point(1.341217, 0.355424, 0.000000),  geo::Point(4.000000, 0.000000, 0.000000),
        geo::Point(4.015900, 0.118942, 0.000000),  geo::Point(1.349202, 0.475424, 0.000000),
    };

    int counter = 0;
    for (auto polygon : polygons)
        for (auto p : polygon)
            my_assert(std::abs(p[0] - expected_points[counter][0]) < 0.0001 &&
                      std::abs(p[1] - expected_points[counter++][1]) < 0.0001);

    my_assert(std::abs(arc.radius - 20.2) < 0.0001);
}

int test_arc_main() {
    test_arc();
    return 0;
}
