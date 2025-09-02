#include "plane.hpp"

#include <cmath>

namespace geo {
Plane::Plane(const Point& point, const Vector& normal_) : point(point), normal(normal_) {
    // Calculate the plane equation coefficients Ax + By + Cz + D = 0
    this->normal.unitize();
    equation[0] = normal[0];
    equation[1] = normal[1];
    equation[2] = normal[2];
    equation[3] = -(normal[0] * point[0] + normal[1] * point[1] + normal[2] * point[2]);

    // Get the x and y axis vectors
    if (!get_xy_vectors(_x_axis, _y_axis))
        throw std::invalid_argument("Normal vector cannot be zero.");
}

Plane::Plane(const double e[4]) {
    for (size_t i = 0; i < 4; ++i) {
        equation[i] = e[i];
    }
    // Calculate the point and normal vector from the equation
    normal = Vector(equation[0], equation[1], equation[2]);
    if (normal.length() == 0) {
        throw std::invalid_argument("Normal vector cannot be zero.");
    }
    point = Point(0, 0, -equation[3] / equation[2]);  // Simplified for demonstration

    // Get the x and y axis vectors
    if (!get_xy_vectors(_x_axis, _y_axis))
        throw std::invalid_argument("Normal vector cannot be zero.");
}

const Point& Plane::get_point() const { return point; }

const Vector& Plane::get_normal() const { return normal; }

double Plane::distance_to_point(const Point& point) const {
    double length = normal.compute_length();
    return std::abs(normal[0] * point[0] + normal[1] * point[1] + normal[2] * point[2] +
                    equation[3]) /
           length;
}

double Plane::operator[](size_t index) const {
    if (index >= 4) {
        throw std::out_of_range("Index out of range. Valid indices are 0 to 3.");
    }
    return equation[index];
}

double Plane::squared_distance_to_point(const Point& point) const {
    double equation[4];
    get_plane_equation(equation);
    double numerator =
        equation[0] * point[0] + equation[1] * point[1] + equation[2] * point[2] + equation[3];
    numerator = numerator * numerator;  // Square the numerator
    double denominator =
        equation[0] * equation[0] + equation[1] * equation[1] + equation[2] * equation[2];
    return numerator / denominator;
}

Point Plane::closest_point(const Point& point) const {
    double equation[4];
    get_plane_equation(equation);
    double distance =
        equation[0] * point[0] + equation[1] * point[1] + equation[2] * point[2] + equation[3];
    return Point(point[0] - equation[0] * distance, point[1] - equation[1] * distance,
                 point[2] - equation[2] * distance);
}

void Plane::get_plane_equation(double (&equation)[4]) const {
    equation[0] = normal[0];
    equation[1] = normal[1];
    equation[2] = normal[2];
    equation[3] = -(normal[0] * point[0] + normal[1] * point[1] + normal[2] * point[2]);
}

double Plane::value_at(const Point& point) const {
    return (equation[0] * point[0] + equation[1] * point[1] + equation[2] * point[2] + equation[3]);
}

bool Plane::get_xy_vectors(Vector& x_axis, Vector& y_axis) {
    bool result = x_axis.perpendicular_to(this->normal);
    x_axis.unitize();
    y_axis = this->normal.cross(x_axis);
    return result;
}

Vector Plane::get_xaxis() const { return _x_axis; }

Vector Plane::get_yaxis() const { return _y_axis; }

// Static method to create an XY plane
Plane Plane::world_xy() {
    Point origin(0.0, 0.0, 0.0);
    Vector normal(0.0, 0.0, 1.0);  // Normal vector pointing along the Z-axis
    return Plane(origin, normal);
}

}  // namespace geo