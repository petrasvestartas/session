#pragma once
#include "core.hpp" // Include the header of the class you're testing


namespace geo {

    class Arc {
    public:
        Point start_point;
        Point mid_point;
        Point end_point;
        Point center;
        double radius;
        double start_angle;
        double mid_angle;
        double end_angle;

        Arc(const Point& start, const Point& mid, const Point& end)
            : start_point(start), mid_point(mid), end_point(end) {
            calculate_arc_properties();
        }

        void calculate_arc_properties();

        std::vector<Point> divide_arc_into_points(int divisions) const;
    };

} // namespace geo