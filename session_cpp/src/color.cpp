#include "color.hpp"
#include <random>
#include <sstream>
#include <iomanip>

namespace geo {

std::string generate_uuid() {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<> dis(0, 15);
    std::uniform_int_distribution<> dis2(8, 11);

    std::stringstream ss;
    ss << std::hex;
    for (int i = 0; i < 8; i++) {
        ss << dis(gen);
    }
    ss << "-";
    for (int i = 0; i < 4; i++) {
        ss << dis(gen);
    }
    ss << "-4";
    for (int i = 0; i < 3; i++) {
        ss << dis(gen);
    }
    ss << "-";
    ss << dis2(gen);
    for (int i = 0; i < 3; i++) {
        ss << dis(gen);
    }
    ss << "-";
    for (int i = 0; i < 12; i++) {
        ss << dis(gen);
    }
    return ss.str();
}

Color::Color(unsigned char r, unsigned char g, unsigned char b, unsigned char a, const std::string& guid) 
    : r(r), g(g), b(b), a(a), guid(guid.empty() ? generate_uuid() : guid) {}

Color Color::white() {
    return Color(255, 255, 255, 255);
}

Color Color::black() {
    return Color(0, 0, 0, 255);
}

std::array<double, 4> Color::to_float_array() const {
    return {r / 255.0, g / 255.0, b / 255.0, a / 255.0};
}

Color Color::from_float(double r, double g, double b, double a) {
    return Color(
        static_cast<unsigned char>(r * 255.0 + 0.5),
        static_cast<unsigned char>(g * 255.0 + 0.5),
        static_cast<unsigned char>(b * 255.0 + 0.5),
        static_cast<unsigned char>(a * 255.0 + 0.5)
    );
}

nlohmann::ordered_json Color::to_json_data() const {
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

Color Color::from_json_data(const nlohmann::json& data) {
    return Color(
        static_cast<unsigned char>(data["r"]),
        static_cast<unsigned char>(data["g"]),
        static_cast<unsigned char>(data["b"]),
        static_cast<unsigned char>(data["a"]),
        data["guid"]
    );
}

}  // namespace geo