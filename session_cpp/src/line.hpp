#pragma once

#include "point.hpp"
#include "vector.hpp"

namespace geo {
/**
 * @class Line
 * @brief A class representing a line in 3D space.
 */
class Line {
   public:
    /**
     * @brief Default constructor. Initializes the line with two points at the origin.
     */
    Line();

    /**
     * @brief Constructor that initializes the line with the given points.
     * @param p1 The first point.
     * @param p2 The second point.
     */
    Line(const Point& p1, const Point& p2);

    /**
     * @brief Constructor that initializes the line with the given point coordinates.
     * @param x0 first point x coordinate.
     * @param y0 first point y coordinate.
     * @param z0 first point z coordinate.
     * @param x1 first point x coordinate.
     * @param y1 first point y coordinate.
     * @param z1 first point z coordinate.
     */
    Line(const double& x0, const double& y0, const double& z0, const double& x1, const double& y1,
         const double& z1);



    /**
     * @brief Subscript operator for non-const access.
     * @param index The index of the point to access (0 for the first point, 1 for the second
     * point).
     * @return A reference to the point at the given index.
     */
    Point& operator[](int index);

    /**
     * @brief Subscript operator for const access.
     * @param index The index of the point to access (0 for the first point, 1 for the second
     * point).
     * @return A const reference to the point at the given index.
     */
    const Point& operator[](int index) const;

    /**
     * @brief Calculates the length of the line.
     * @return The length of the line.
     */
    double length() const;

    /**
     * @brief Calculates the squared length of the line.
     * @return The squared length of the line.
     */
    double squared_length() const;

    /**
     * @brief Converts the line to a vector.
     * @return A vector that points from the first point to the second point of the line.
     */
    Vector to_vector() const;

    /**
     * @brief Calculates the point at a given parameter t.
     * @param t The parameter value.
     * @return The point at the given parameter value.
     */
    Point point_at(const double& t) const;

    /**
     * @brief Scales the line by a given factor.
     * @param factor The factor to scale by.
     */
    void scale(double factor);

    /**
     * @brief Scales the line by a given factor.
     * @param factor The factor to scale by.
     */
    Line scaled(double factor);

    /**
     * @brief Move line by summing vector and point coordinates
     */
    void translate(Vector& translation_vector);

    /**
     * @brief Move line by summing vector and point coordinates
     */
    Line translated(Vector& translation_vector);

   private:
    /**
     * @brief The points of the line.
     */
    Point points[2];
};
}  // namespace geo