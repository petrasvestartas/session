#pragma once
#include <array>
#include <string>
#include "json.hpp"

namespace geo {

/**
 * @class Color
 * @brief A color is defined by RGBA coordinates from 0 to 255.
 * 
 * @var Color::r Red component (0-255)
 * @var Color::g Green component (0-255)
 * @var Color::b Blue component (0-255)
 * @var Color::a Alpha component (0-255)
 * @var Color::guid Unique identifier
 */
class Color {
public:
    unsigned char r, g, b, a;
    std::string guid;
    
    /**
     * @brief Constructor with RGBA values.
     */
    Color(unsigned char r = 255, unsigned char g = 255, unsigned char b = 255, unsigned char a = 255, const std::string& guid = "");
    
    /**
     * @brief Create a white color.
     */
    static Color white();
    
    /**
     * @brief Create a black color.
     */
    static Color black();
    
    /**
     * @brief Convert to normalized float array [0-1].
     */
    std::array<double, 4> to_float_array() const;
    
    /**
     * @brief Create color from normalized float values [0-1].
     */
    static Color from_float(double r, double g, double b, double a);
    
    /**
     * @brief Convert to JSON-serializable object.
     */
    nlohmann::ordered_json to_json_data() const;
    
    /**
     * @brief Create color from JSON data.
     */
    static Color from_json_data(const nlohmann::json& data);
};

} // namespace geo