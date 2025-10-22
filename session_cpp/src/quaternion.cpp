#include "quaternion.h"
#include "guid.h"
#include <cmath>
#include <fstream>

namespace session_cpp {

Quaternion::Quaternion() : typ("Quaternion"), guid(::guid()), name("my_quaternion"), s(1.0), v(0.0, 0.0, 0.0) {}

Quaternion::Quaternion(double s, const Vector& v) 
    : typ("Quaternion"), guid(::guid()), name("my_quaternion"), s(s), v(v) {}

Quaternion Quaternion::identity() {
    return Quaternion(1.0, Vector(0.0, 0.0, 0.0));
}

Quaternion Quaternion::from_sv(double s, double x, double y, double z) {
    return Quaternion(s, Vector(x, y, z));
}

Quaternion Quaternion::from_axis_angle(const Vector& axis, double angle) {
    Vector normalized_axis = axis;
    normalized_axis.normalize_self();
    double half_angle = angle * 0.5;
    double s = std::cos(half_angle);
    Vector v = normalized_axis * std::sin(half_angle);
    return Quaternion(s, v);
}

Vector Quaternion::rotate_vector(const Vector& vec) const {
    Vector qv = v;
    Vector vec_copy = vec;
    Vector uv = qv.cross(vec_copy);
    Vector uuv = qv.cross(uv);
    return vec_copy + (uv * s + uuv) * 2.0;
}

double Quaternion::magnitude() const {
    return std::sqrt(s * s + v.x() * v.x() + v.y() * v.y() + v.z() * v.z());
}

Quaternion Quaternion::normalize() const {
    double mag = magnitude();
    if (mag > 1e-10) {
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
    Quaternion q(s, v * -1.0);
    q.typ = typ;
    q.guid = guid;
    q.name = name;
    return q;
}

Quaternion Quaternion::operator*(const Quaternion& other) const {
    Vector v_copy = v;
    Vector other_v_copy = other.v;
    double new_s = s * other.s - v_copy.dot(other_v_copy);
    Vector new_v = other_v_copy * s + v_copy * other.s + v_copy.cross(other_v_copy);
    return Quaternion(new_s, new_v);
}

nlohmann::ordered_json Quaternion::jsondump() const {
    return nlohmann::ordered_json{
        {"type", typ},
        {"guid", guid},
        {"name", name},
        {"s", static_cast<double>(s)},
        {"x", static_cast<double>(v.x())},
        {"y", static_cast<double>(v.y())},
        {"z", static_cast<double>(v.z())}
    };
}

Quaternion Quaternion::jsonload(const nlohmann::json& data) {
    Quaternion q(data["s"].get<double>(), Vector(data["x"].get<double>(), data["y"].get<double>(), data["z"].get<double>()));
    q.typ = data.value("type", "Quaternion");
    q.guid = data["guid"].get<std::string>();
    q.name = data["name"].get<std::string>();
    return q;
}



}  // namespace session_cpp
