#include "quaternion.h"
#include "guid.h"
#include <cmath>
#include <fstream>

namespace session_cpp {

Quaternion::Quaternion() : typ("Quaternion"), guid(::guid()), name("my_quaternion"), s(1.0f), v(0.0f, 0.0f, 0.0f) {}

Quaternion::Quaternion(float s, const Vector& v) 
    : typ("Quaternion"), guid(::guid()), name("my_quaternion"), s(s), v(v) {}

Quaternion Quaternion::identity() {
    return Quaternion(1.0f, Vector(0.0f, 0.0f, 0.0f));
}

Quaternion Quaternion::from_sv(float s, float x, float y, float z) {
    return Quaternion(s, Vector(x, y, z));
}

Quaternion Quaternion::from_axis_angle(const Vector& axis, float angle) {
    Vector axis_copy = axis;
    Vector normalized_axis = axis_copy.normalize();
    float half_angle = angle * 0.5f;
    float s = std::cos(half_angle);
    Vector v = normalized_axis * std::sin(half_angle);
    return Quaternion(s, v);
}

Vector Quaternion::rotate_vector(const Vector& vec) const {
    Vector qv = v;
    Vector vec_copy = vec;
    Vector uv = qv.cross(vec_copy);
    Vector uuv = qv.cross(uv);
    return vec_copy + (uv * s + uuv) * 2.0f;
}

float Quaternion::magnitude() const {
    return std::sqrt(s * s + v.x() * v.x() + v.y() * v.y() + v.z() * v.z());
}

Quaternion Quaternion::normalize() const {
    float mag = magnitude();
    if (mag > 1e-10f) {
        Quaternion q(s / mag, v / mag);
        q.typ = typ;
        q.guid = guid;
        q.name = name;
        return q;
    } else {
        return Quaternion::identity();
    }
}

Quaternion Quaternion::conjugate() const {
    Quaternion q(s, v * -1.0f);
    q.typ = typ;
    q.guid = guid;
    q.name = name;
    return q;
}

Quaternion Quaternion::operator*(const Quaternion& other) const {
    Vector v_copy = v;
    Vector other_v_copy = other.v;
    float new_s = s * other.s - v_copy.dot(other_v_copy);
    Vector new_v = other_v_copy * s + v_copy * other.s + v_copy.cross(other_v_copy);
    return Quaternion(new_s, new_v);
}

nlohmann::ordered_json Quaternion::to_json_data() const {
    auto clean_float = [](float val) -> double { 
        // For very small values, keep high precision
        if (std::abs(val) < 0.01f) return static_cast<double>(val);
        // For normal values, round to 2 decimal places
        return static_cast<double>(std::round(val * 100.0f) / 100.0f);
    };
    return nlohmann::ordered_json{
        {"type", typ},
        {"guid", guid},
        {"name", name},
        {"s", clean_float(s)},
        {"x", clean_float(v.x())},
        {"y", clean_float(v.y())},
        {"z", clean_float(v.z())}
    };
}

Quaternion Quaternion::from_json_data(const nlohmann::json& data) {
    Quaternion q(data["s"].get<float>(), Vector(data["x"].get<float>(), data["y"].get<float>(), data["z"].get<float>()));
    q.typ = data.value("type", "Quaternion");
    q.guid = data["guid"].get<std::string>();
    q.name = data["name"].get<std::string>();
    return q;
}

void Quaternion::to_json(const std::string& filepath) const {
    std::ofstream file(filepath);
    file << to_json_data().dump(4);
    file.close();
}

Quaternion Quaternion::from_json(const std::string& filepath) {
    std::ifstream file(filepath);
    nlohmann::json data;
    file >> data;
    file.close();
    return from_json_data(data);
}

}  // namespace session_cpp
