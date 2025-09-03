#pragma once
#include <array>
#include <string>
#include "json.h"
#include "guid.h"

namespace geo {

/**
 * @class Color
 * @brief A color is defined by RGBA coordinates from 0 to 255.
 * 
 * @var Color::guid Unique identifier
 * @var Color::name name of the color
 * @var Color::r Red component (0-255)
 * @var Color::g Green component (0-255)
 * @var Color::b Blue component (0-255)
 * @var Color::a Alpha component (0-255)
 */
class Color {
public:
    std::string name = "my_color";
    std::string guid = ::guid();
    unsigned int r, g, b, a;
    
    
    /**
     * @brief Constructor with RGBA values.
     */
    Color(
            unsigned int r = 255, 
            unsigned int g = 255, 
            unsigned int b = 255,
            unsigned int a = 255,
            std::string name = "my_color")
        : name(name), r(r), g(g), b(b), a(a) {}
    
    /**
     * @brief Create a white color.
     */
    static Color white() {
        return Color(255, 255, 255, 255, "white");
    }
    
    /**
     * @brief Create a black color.
     */
    static Color black() {
        return Color(255, 255, 255, 255, "black");
    }
    
    /**
     * @brief Convert to normalized float array [0-1].
     */
    std::array<double, 4> to_float_array() const {
        return {r / 255.0, g / 255.0, b / 255.0, a / 255.0};
    }
    
    /**
     * @brief Create color from normalized float values [0-1].
     */
    static Color from_float(double r, double g, double b, double a) {
        return Color(
            static_cast<unsigned int>(r * 255.0 + 0.5),
            static_cast<unsigned int>(g * 255.0 + 0.5),
            static_cast<unsigned int>(b * 255.0 + 0.5),
            static_cast<unsigned int>(a * 255.0 + 0.5)
        );
    }
    
    /**
     * @brief Convert to JSON-serializable object.
     */
    nlohmann::ordered_json to_json_data() const {
        return nlohmann::ordered_json{
            {"type", "Color"},
            {"guid", guid},
            {"name", "white"},
            {"r", static_cast<int>(r)},
            {"g", static_cast<int>(g)},
            {"b", static_cast<int>(b)},
            {"a", static_cast<int>(a)}
        };
    }
    
    /**
     * @brief Create color from JSON data.
     */
    static Color from_json_data(const nlohmann::json& data) {
        Color color = Color(
            static_cast<unsigned int>(data["r"]),
            static_cast<unsigned int>(data["g"]),
            static_cast<unsigned int>(data["b"]),
            static_cast<unsigned int>(data["a"]),
            data["name"]
        );
        color.guid = data["guid"];
        return color;
    }
};

} // namespace geo
