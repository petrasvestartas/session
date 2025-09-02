#pragma once

#include "core.hpp"

namespace geo {

class Xform {
   public:
    // Static methods for transformations
    static Matrix translation(double tx, double ty, double tz);
    static Matrix scaling(double sx, double sy, double sz);

    // Static methods for frame-to-frame transformations
    static Matrix change_basis(geo::Point& origin_1, geo::Vector& x_axis_1, geo::Vector& y_axis_1,
                               geo::Vector& z_axis_1, geo::Point& origin_0, geo::Vector& x_axis_0,
                               geo::Vector& y_axis_0, geo::Vector& z_axis_0);

    static Matrix plane_to_plane(geo::Point& origin_0, geo::Vector& x_axis_0, geo::Vector& y_axis_0,
                                 geo::Point& origin_1, geo::Vector& x_axis_1,
                                 geo::Vector& y_axis_1);

    static Matrix plane_to_xy(geo::Point& origin, geo::Vector& x_axis, geo::Vector& y_axis);
    static Matrix xy_to_plane(geo::Point& origin, geo::Vector& x_axis, geo::Vector& y_axis);

    // Static method to apply transformation to a point
    static geo::Point apply(const Matrix& transform, const geo::Point& point);

    // Static method to apply transformation to a vector of points
    static std::vector<geo::Point> apply(const Matrix& transform,
                                         const std::vector<geo::Point>& points);

    // Static method to apply transformation to an array of points
    template <std::size_t N>
    static std::array<geo::Point, N> apply(const Matrix& transform,
                                           const std::array<geo::Point, N>& points);

    // Static method to apply transformation to a nested vector of points
    template <typename T>
    static std::vector<T> apply(const Matrix& transform, const std::vector<T>& points);

    // Static method to apply transformation to a nested array of points
    template <typename T, std::size_t N>
    static std::array<T, N> apply(const Matrix& transform, const std::array<T, N>& points);
};
}  // namespace geo