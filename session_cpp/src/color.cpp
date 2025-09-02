#include "color.hpp"

namespace session {

Color::Color(double r, double g, double b, double a) : r(r), g(g), b(b), a(a) {}

Color Color::white() {
    return Color(255.0, 255.0, 255.0, 100.0);
}

std::array<double, 4> Color::to_float_array() const {
    return {r / 255.0, g / 255.0, b / 255.0, a / 100.0};
}

Color Color::from_float(double r, double g, double b, double a) {
    return Color(r * 255.0, g * 255.0, b * 255.0, a * 100.0);
}

} // namespace session