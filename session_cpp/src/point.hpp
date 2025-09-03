#pragma once
#include <iostream>
#include <string>
#include <sstream>
#include <fstream>
#include "color.hpp"
#include "json.hpp"
#include "globals.hpp"


namespace geo {
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
    std::string name = "my_point";
    Color pointcolor = Color::white();
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
        std::ostringstream oss;
        oss << "Point(" << x << ", " << y << ", " << z << ", " 
            << guid << ", " << name << ", " << pointcolor.r << "," 
            << pointcolor.g << "," << pointcolor.b << "," << pointcolor.a 
            << ", " << width << ")";
        return oss.str();
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /**
     * @brief Convert to JSON-serializable object
     */
    nlohmann::ordered_json to_json_data() const {
        return nlohmann::ordered_json{
            {"type", "Point"},
            {"guid", guid},
            {"name", name},
            {"x", x},
            {"y", y},
            {"z", z},
            {"width", width},
            {"pointcolor", pointcolor.to_json_data()}
        };
    }

    /**
     * @brief Create point from JSON data
     */
    static Point from_json_data(const nlohmann::json& data) {
        Point point(data["x"], data["y"], data["z"]);
        point.guid = data["guid"];
        point.name = data["name"];
        point.pointcolor = Color::from_json_data(data["pointcolor"]);
        point.width = data["width"];
        return point;
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

} // namespace geo