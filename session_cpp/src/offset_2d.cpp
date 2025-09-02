#include "offset_2d.hpp"  // Include the header of the class you're testing

namespace geo {

Point intersect(const std::pair<geo::Point, geo::Point>& s1,
                const std::pair<geo::Point, geo::Point>& s2, double tol) {
    // Line AB represented as a1x + b1y = c1
    double a1 = s1.second[1] - s1.first[1];
    double b1 = s1.first[0] - s1.second[0];
    double c1 = a1 * s1.first[0] + b1 * s1.first[1];

    // Line CD represented as a2x + b2y = c2
    double a2 = s2.second[1] - s2.first[1];
    double b2 = s2.first[0] - s2.second[0];
    double c2 = a2 * s2.first[0] + b2 * s2.first[1];

    double determinant = a1 * b2 - a2 * b1;
    geo::Point intersection(0, 0, 0);
    if (std::abs(determinant) < tol) {
        // The lines are parallel or almost parallel within the given tolerance
        throw std::runtime_error(
            "Lines are parallel or almost parallel within the given tolerance.");
    } else {
        // The lines intersect at a single point
        intersection[0] = (b2 * c1 - b1 * c2) / determinant;
        intersection[1] = (a1 * c2 - a2 * c1) / determinant;
        return intersection;
    }
}

std::vector<std::pair<geo::Point, geo::Point>> offset_segments(
    const std::vector<geo::Point>& points, const double& distance) {
    std::vector<std::pair<geo::Point, geo::Point>> segments;
    for (size_t i = 0; i < points.size() - 1; ++i) {
        geo::Vector v0(points[i][0], points[i][1], points[i][2]);
        geo::Vector v1(points[i + 1][0], points[i + 1][1], points[i + 1][2]);
        geo::Vector v = v1 - v0;
        geo::Vector n(0, 0, -1);
        geo::Vector perp = v.cross(n) * distance;
        geo::Point p0(points[i][0] + perp[0], points[i][1] + perp[1], points[i][2] + perp[2]);
        geo::Point p1(points[i + 1][0] + perp[0], points[i + 1][1] + perp[1],
                      points[i + 1][2] + perp[2]);
        segments.push_back({p0, p1});
    }
    return segments;
}

std::vector<geo::Point> offset_polyline(const std::vector<geo::Point>& polyline, double distance,
                                        double tol) {
    std::vector<std::pair<geo::Point, geo::Point>> segments = offset_segments(polyline, distance);
    std::vector<geo::Point> offset = {segments[0].first};
    for (size_t i = 0; i < segments.size() - 1; ++i) {
        geo::Point point = intersect(segments[i], segments[i + 1], tol);
        offset.push_back(point);
    }
    offset.push_back(segments.back().second);
    return offset;
}

}  // namespace geo