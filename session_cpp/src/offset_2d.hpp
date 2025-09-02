#pragma once
#include "core.hpp"  // Include the header of the class you're testing

namespace geo {

/**
 * @brief Calculate the intersection point of two line segments
 *
 * @param s1 The first line segment
 * @param s2 The second line segment
 * @param tol The tolerance for the intersection point
 * @return Point The intersection point
 */
Point intersect(const std::pair<geo::Point, geo::Point>& s1,
                const std::pair<geo::Point, geo::Point>& s2, double tol);

/**
 * @brief Offset a set of line segments by a given distance
 *
 * @param points The points that define the line segments
 * @param distance The distance by which to offset the line segments
 * @return std::vector<std::pair<geo::Point, geo::Point>> The offset line segments
 */
std::vector<std::pair<geo::Point, geo::Point>> offset_segments(
    const std::vector<geo::Point>& points, const double& distance);

/**
 * @brief Offset a polyline by a given distance
 *
 * @param polyline The polyline to offset
 * @param distance The distance by which to offset the polyline
 * @param tol The tolerance for the intersection point
 * @return std::vector<geo::Point> The offset polyline
 */
std::vector<geo::Point> offset_polyline(const std::vector<geo::Point>& polyline, double distance,
                                        double tol = 1e-6);

}  // namespace geo