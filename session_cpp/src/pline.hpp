#pragma once

#include "core.hpp"

namespace geo {
class Pline {
   public:
    // Declaration of the cut method
    // https://github.com/petrasvestartas/compas_timbervaultedfloor/blob/main/src/compas_timbervaultedfloor/floor_geometry/mesh_conic_projection.py

    /**
     * @brief Split a polyline into two parts by a plane and return the part above the plane.
     *
     * @param [in] points: The points of the polygon, no duplicate start and end points.
     * @param [in] plane: The plane to cut the polygon.
     * @param [out] result: The points of the polygon above the plane.
     * @return bool: True if the polygon was cut, false otherwise.
     */
    static bool cut(std::vector<Point>& points, const Plane& plane, std::vector<Point>& result);
};
}  // namespace geo