#pragma once
#include <array>

namespace session {

/**
 * @class Color
 * @brief A color is defined by RGBA coordinates from 0 to 255 and alpha from 0 to 100.
 * 
 * @var Color::r Red component (0-255)
 * @var Color::g Green component (0-255)
 * @var Color::b Blue component (0-255)
 * @var Color::a Alpha component (0-100)
 */
class Color {
public:
    int r, g, b, a;
    
    /**
     * @brief Constructor with RGBA values.
     */
    Color(int r = 255, int g = 255, int b = 255, int a = 100);
    
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
};

} // namespace session