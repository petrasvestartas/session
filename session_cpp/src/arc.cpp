#include "arc.hpp"

namespace geo {

void Arc::calculate_arc_properties() {
    // Calculate the perpendicular bisector of start and mid, and mid and end
    Point midStart = start_point.mid_point(mid_point);
    Point midEnd = mid_point.mid_point(end_point);


    Vector startToMid = mid_point - Vector(start_point[0], start_point[1], start_point[2]);
    Vector midToEnd = end_point - Vector(mid_point[0], mid_point[1], mid_point[2]);

    Point bisector1 = Point(-startToMid[1], startToMid[0], 0);
    Point bisector2 = Point(-midToEnd[1], midToEnd[0], 0);

    // Solve the linear equations to find the intersection (center)
    double det = bisector1[0] * bisector2[1] - bisector1[1] * bisector2[0];
    if (std::abs(det) < 1e-10) {
        // The points are collinear or too close to each other
        throw std::runtime_error("Invalid arc: points are collinear or too close to each other.");
    }

    Vector vecBetweenMidpoints = midEnd - Vector(midStart[0], midStart[1], midStart[2]);
    double t = (vecBetweenMidpoints[0] * bisector2[1] - vecBetweenMidpoints[1] * bisector2[0]) / det;
    center = midStart + Vector(bisector1[0], bisector1[1], bisector1[2]) * t;

    // Calculate radius
    radius = center.distance(start_point);

    // Calculate angles
    start_angle = std::atan2(start_point[1] - center[1], start_point[0] - center[0]);
    double mid_angle = std::atan2(mid_point[1] - center[1], mid_point[0] - center[0]);
    end_angle = std::atan2(end_point[1] - center[1], end_point[0] - center[0]);
}

std::vector<Point> Arc::divide_arc_into_points(int divisions) const {
    std::vector<Point> points;
    if (divisions < 1) return points; // Return empty if invalid divisions

    double total_angle = end_angle - start_angle;
    // Normalize total_angle if necessary
    // For example, if working in radians and total_angle is negative or too large,
    // adjust it to be within a valid range (0 to 2*PI for a full circle in radians)

    double angle_increment = total_angle / divisions;

    for (int i = 0; i <= divisions; ++i) {
        double angle = start_angle + angle_increment * i;
        double x = center[0] + radius * cos(angle);
        double y = center[1] + radius * sin(angle);
        points.push_back(Point(x, y));
    }

    return points;
}

} // namespace geo