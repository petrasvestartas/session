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
    float radius;
    Line line;
    Mesh mesh;

    /**
     * @brief Creates a new Arrow from a line and radius.
     * @param line The centerline of the arrow
     * @param radius The radius of the arrow body
     * @return A new Arrow with a cylinder body and cone head mesh
     */
    Arrow(const Line& line, float radius);

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Serializes the Arrow to a JSON string
    nlohmann::ordered_json to_json_data() const;
    
    /// Deserializes an Arrow from JSON data
    static Arrow from_json_data(const nlohmann::json& data);
    
    /// Serializes the Arrow to a JSON file
    void to_json(const std::string& filepath) const;
    
    /// Deserializes an Arrow from a JSON file
    static Arrow from_json(const std::string& filepath);

private:
    static Mesh create_arrow_mesh(const Line& line, float radius);
    static std::pair<std::vector<Point>, std::vector<std::array<size_t, 3>>> unit_cylinder_geometry();
    static std::pair<std::vector<Point>, std::vector<std::array<size_t, 3>>> unit_cone_geometry();
};

} // namespace session_cpp
