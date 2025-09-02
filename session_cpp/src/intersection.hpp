#pragma once

#include "core.hpp"

namespace geo {
class Intersection {
   public:
    // Constructor
    Intersection();

    /**
     * Get the intersection of a line and a plane
     * https://github.com/petrasvestartas/wood/blob/main/cmake/src/wood/include/cgal_intersection_util.cpp
     *
     * @param [in] line segment
     * @param [in] plane plane
     * @param [out] output the point output
     * @param [out] is_finite true if the line is finite, still the point will be outputed
     * @return true the intersection is sucsessful or point is outside the line, incase the finite
     * search is used
     */
    static bool line_plane(const Line &line, const Plane &plane, Point &output, bool is_finite);

   private:
    // Add private methods and members here
};
}  // namespace geo