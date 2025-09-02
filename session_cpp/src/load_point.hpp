#pragma once

#include "point.hpp"
#include "vector.hpp"
#include <string>
#include <ostream>

namespace geo {
/**
 * @class LoadPoint
 * @brief A class representing a point load in 3D space.
 */
class LoadPoint {
   public:
    /**
     * @brief Default constructor. Initializes the load at the origin with zero load.
     */
    LoadPoint();

    /**
     * @brief Constructor that initializes the load point with a point and load vector.
     * @param point The position where the load is applied.
     * @param load The load vector.
     */
    LoadPoint(const Point& point, const Vector& load);

    /**
     * @brief Converts the load point to a string representation.
     * @return String representation of the load point.
     */
    std::string to_string() const;

    // Public data members
    Point p;   // The position where the load is applied
    Vector l;  // The load vector
};

// Stream insertion operator for LoadPoint
std::ostream& operator<<(std::ostream& os, const LoadPoint& load_point);

}  // namespace geo
