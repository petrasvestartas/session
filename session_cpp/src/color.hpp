#pragma once
#include <array>

namespace session {

/**
 * @brief A simple color class with RGBA values.
 */
class Color {
public:
    double r, g, b, a;
    
    /**
     * @brief Constructor with default white color values.
     */
    Color(double r = 255.0, double g = 255.0, double b = 255.0, double a = 100.0);
    
    /**
     * @brief Create a white color.
     */
    static Color white();
    
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