#pragma once

#include "line.h"
#include "mesh.h"
#include "point.h"
#include "vector.h"
#include "xform.h"
#include "guid.h"
#include "json.h"
#include <string>
#include <vector>
#include <array>

namespace session_cpp {

/**
 * @class Arrow
 * @brief An arrow geometry defined by a line and radius, the head is uniformly scaled.
 * 
 * The arrow is generated as a 10-sided cylinder body and an 8-sided cone head
 * that is oriented along the line direction and scaled to match the line length and specified radius.
 */
class Arrow {
public:
    std::string guid = ::guid();
    std::string name = "my_arrow";
    double radius;
    Line line;
    Mesh mesh;
    Xform xform;

    /**
     * @brief Creates a new Arrow from a line and radius.
     * @param line The centerline of the arrow
     * @param radius The radius of the arrow body
     * @return A new Arrow with a cylinder body and cone head mesh
     */
    Arrow(const Line& line, double radius);

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Transformation
    ///////////////////////////////////////////////////////////////////////////////////////////

    void transform();
    Arrow transformed() const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Serializes the Arrow to a JSON string
    nlohmann::ordered_json jsondump() const;
    
    /// Deserializes an Arrow from JSON data
    static Arrow jsonload(const nlohmann::json& data);
    
    /// Serializes the Arrow to a JSON file
    
    /// Deserializes an Arrow from a JSON file

private:
    static Mesh create_arrow_mesh(const Line& line, double radius);
    static std::pair<std::vector<Point>, std::vector<std::array<size_t, 3>>> unit_cylinder_geometry();
    static std::pair<std::vector<Point>, std::vector<std::array<size_t, 3>>> unit_cone_geometry();
};

} // namespace session_cpp
