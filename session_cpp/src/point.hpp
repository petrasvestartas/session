#pragma once
#include <iostream>
#include <string>
#include <color.hpp>
#include <format>
#include "json.hpp"
#include "globals.hpp"


namespace session_cpp {
/**
 * @class Point
 * @brief A point defined by XYZ coordinates with display properties.
 * 
 * @var Point::x X coordinate
 * @var Point::y Y coordinate  
 * @var Point::z Z coordinate
 * @var Point::name Point identifier
 * @var Point::pointcolor Visual color
 * @var Point::width Display width
 */
class Point {
   public:

    double x = 0.0;
    double y = 0.0;
    double z = 0.0;
    std::string guid = generate_uuid();
    std::string name = "Point";
    Color pointcolor = Color.white();
    double width = 1.0;


public:

    /**
     * @brief Constructor.
     * @param x The X coordinate of the point.
     * @param y The Y coordinate of the point.
     * @param z The Z coordinate of the point.
     */
    Point(double x, double y, double z) : x(x), y(y), z(z) {}

    /**
     * @brief Convert point to string representation
     */
    std::string to_string() const {
        return std::format("Point({}, {}, {}, {}, {}, {}, {})", 
                          x(), y(), z(), guid, name, pointcolor, width);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /**
     * @brief Convert to JSON-serializable object
     */
    nlohmann::json to_json_data() const {
        return nlohmann::json{
            {"dtype", "Point"},
            {"x", x},
            {"y", y},
            {"z", z},
            {"guid", guid},
            {"name", name},
            {"pointcolor", pointcolor.to_float_array()},
            {"width", width}
        };
    }

    /**
     * @brief Create point from JSON data
     */
    static Point from_json_data(const nlohmann::json& data) {
        auto color_array = data["pointcolor"];
        return Point(
            data["x"], 
            data["y"], 
            data["z"], 
            data["guid"],
            data["name"],
            Color::from_float(color_array[0], color_array[1], color_array[2], color_array[3]),
            data["width"]
        );
    }

    /**
     * @brief Serialize to JSON file
     */
    void to_json(const std::string& filepath) const {
        std::ofstream file(filepath);
        file << to_json_data().dump(4);
    }

    /**
     * @brief Deserialize from JSON file
     */
    static Point from_json(const std::string& filepath) {
        std::ifstream file(filepath);
        nlohmann::json data;
        file >> data;
        return from_json_data(data);
    }




}; // End of Point class

    /**
     * @brief Stream insertion operator for Point objects.
     * @param os The output stream.
     * @param point The Point to insert into the stream.
     * @return A reference to the output stream.
     */
    std::ostream& operator<<(std::ostream& os, const Point& point) {
        return os << point.to_string();
    }

} // namespace session_cpp