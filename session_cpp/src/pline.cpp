#include "pline.hpp"

#include <algorithm>

#include "intersection.hpp"
#include "xform.hpp"

namespace geo {
bool Pline::cut(std::vector<Point>& points, const Plane& plane, std::vector<Point>& result) {
    // Orient the polygon from the cutting frame to the XY frame
    Point origin = plane.get_point();
    Vector x_axis = plane.get_xaxis();
    Vector y_axis = plane.get_yaxis();

    Matrix xform = Xform::plane_to_xy(origin, x_axis, y_axis);
    Matrix xformI = Xform::xy_to_plane(origin, x_axis, y_axis);
    std::vector<Point> points_transformed = Xform::apply(xform, points);

    // Check if the points are below the plane
    bool flag = true;
    size_t counter = 0;
    std::vector<bool> below;

    for (size_t i = 0; i < points_transformed.size(); i++) {
        if (points_transformed[i][2] < 0) {
            below.push_back(true);
            flag = false;
            counter++;
        } else {
            below.push_back(false);
        }
    }

    // For faces that coincide with plane
    std::vector<std::vector<Point>> polygons_culled;
    Plane xy_plane = Plane::world_xy();

    if ((counter != 0 && counter != points_transformed.size()) == false) return false;

    // Intersect polyline segments with the plane
    std::vector<Point> polygon_points;
    std::vector<bool> polygon_points_bool;

    counter = 0;
    for (size_t i = 0; i < points_transformed.size(); i++) {  // Polylines have end point duplicated

        polygon_points.push_back(points_transformed[i]);
        polygon_points_bool.push_back(false);
        Line line =
            Line(points_transformed[i], points_transformed[(i + 1) % points_transformed.size()]);

        Point intersection_point;
        bool result = Intersection::line_plane(line, xy_plane, intersection_point, true);

        if (result) {
            polygon_points.push_back(intersection_point);
            polygon_points_bool.push_back(true);

            if (counter == 0) {
                counter = polygon_points.size() - 1;
            }
        }
    }

    std::rotate(polygon_points.begin(), polygon_points.begin() + counter, polygon_points.end());
    std::rotate(polygon_points_bool.begin(), polygon_points_bool.begin() + counter,
                polygon_points_bool.end());

    // Split points into sub-lists.
    std::vector<std::vector<Point>> points_lists;
    for (size_t i = 0; i < polygon_points.size(); i++) {
        if (polygon_points_bool[i]) points_lists.push_back(std::vector<Point>());

        if (points_lists.size() > 1 && polygon_points_bool[i]) {
            points_lists[points_lists.size() - 2].push_back(polygon_points[i]);
        }

        points_lists[points_lists.size() - 1].push_back(polygon_points[i]);
    }

    points_lists[points_lists.size() - 1].push_back(polygon_points[0]);

    // Which side is needed by measuring the distance from the plane
    for (size_t i = 0; i < points_lists.size(); i++) {
        bool found = false;
        for (auto& p : points_lists[i]) {
            if (p[2] > geo::GLOBALS::TOLERANCE) {
                found = true;
                break;
            }
        }

        if (found) {
            result = Xform::apply(xformI, points_lists[i]);
            break;
        }
    }

    return true;
}
}  // namespace geo