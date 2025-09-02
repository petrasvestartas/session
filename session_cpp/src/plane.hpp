#pragma once

#include "point.hpp"
#include "vector.hpp"

namespace geo {
/**
 * @class Plane
 * @brief A class representing a plane in 3D space.
 *
 * This class stores a point and a normal vector that define the plane.
 * It provides methods to calculate the distance from a point to the plane,
 * find the closest point on the plane to a given point, and evaluate the plane equation for a given
 * point.
 */
class Plane {
   public:
    /**
     * @brief Constructor that initializes the plane with the given point and normal vector.
     * @param point The point on the plane.
     * @param normal The normal vector of the plane.
     */
    Plane(const Point& point, const Vector& normal);

    /**
     * @brief Constructor that initializes the plane with the given equation.
     * @param equation An array of 4 doubles representing the coefficients of the plane equation Ax
     * + By + Cz + D = 0. One of equation[0], equation[1], or equation[2] must be non-zero.
     */
    Plane(const double equation[4]);

    /**
     * @brief Gets the point on the plane.
     * @return The point on the plane.
     */
    const Point& get_point() const;

    /**
     * @brief Gets the normal vector of the plane.
     * @return The normal vector of the plane.
     */
    const Vector& get_normal() const;

    /**
     * @brief Calculates the distance from a point to the plane.
     * @param point The point.
     * @return The distance from the point to the plane.
     */
    double distance_to_point(const Point& point) const;

    /**
     * @brief Overloaded subscript operator to access the coefficients of the plane equation.
     * @param index The index of the coefficient (0 to 3).
     * @return The coefficient at the specified index.
     */
    double operator[](size_t index) const;

    /**
     * @brief Calculates the squared distance from a point to the plane.
     * @param point The point.
     * @return The squared distance from the point to the plane.
     */
    double squared_distance_to_point(const Point& point) const;

    /**
     * @brief Finds the closest point on the plane to a given point.
     * @param point The point.
     * @return The closest point on the plane to the given point.
     */
    Point closest_point(const Point& point) const;

    /**
     * @brief Returns the coefficients of the plane equation.
     * @return A C-style array of 4 doubles containing the coefficients [A, B, C, D] of the plane
     * equation Ax + By + Cz + D = 0.
     */
    void get_plane_equation(double (&equation)[4]) const;

    /**
     * @brief Scales the plane up by a factor of global SCALE.
     */
    void scale_up();

    /**
     * @brief Scales the plane down by a factor of global SCALE.
     */
    void scale_down();

    /**
     * @brief Get numeric value of plane from a point.
     */
    double value_at(const Point& P) const;

    /**
     * @brief Get x-axis and y-axis vectors of the plane.
     */
    bool get_xy_vectors(Vector& x_axis, Vector& y_axis);

    /**
     * @brief Get the x-axis vector of the plane.
     * @return The x-axis vector of the plane.
     */
    Vector get_xaxis() const;

    /**
     * @brief Get the y-axis vector of the plane.
     * @return The y-axis vector of the plane.
     */
    Vector get_yaxis() const;

    // Static method to create an XY plane
    static Plane world_xy();

   private:
    Point point;    ///< The point on the plane.
    Vector normal;  ///< The normal vector of the plane.
    Vector _x_axis;
    Vector _y_axis;
    double equation[4];
};
}  // namespace geo