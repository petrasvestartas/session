#pragma once

#include "line.hpp"
#include "vector.hpp"
#include <string>
#include <ostream>

namespace geo {
/**
 * @class LoadLine
 * @brief A class representing a distributed load along a line in 3D space.
 */
class LoadLine {
   public:
    /**
     * @brief Default constructor. Initializes with default points and zero loads.
     */
    LoadLine();

    /**
     * @brief Constructor that initializes the load line with two points and two load vectors.
     * @param point0 The start point of the line.
     * @param point1 The end point of the line.
     * @param load_start The load vector at the start of the line.
     * @param load_end The load vector at the end of the line.
     */
    LoadLine(const Point& point0, const Point& point1, const Vector& load_start, const Vector& load_end);
    
    /**
     * @brief Constructor that initializes the load line with a line and two load vectors.
     * @param line The line along which the load is applied.
     * @param load_start The load vector at the start of the line.
     * @param load_end The load vector at the end of the line.
     */
    LoadLine(const Line& line, const Vector& load_start, const Vector& load_end);
    
    /**
     * @brief Converts the load line to a string representation.
     * @return String representation of the load line.
     */
    std::string to_string() const;

    // Public data members
    Point p0;           // The start point of the line
    Point p1;           // The end point of the line
    Vector l0;          // The load vector at the start of the line
    Vector l1;          // The load vector at the end of the line
};

// Stream insertion operator for LoadLine
std::ostream& operator<<(std::ostream& os, const LoadLine& load_line);

}  // namespace geo
