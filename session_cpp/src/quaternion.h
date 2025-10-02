#pragma once

#include "vector.h"
#include "json.h"
#include <string>

namespace session_cpp {

class Quaternion {
public:
    std::string typ;
    std::string guid;
    std::string name;
    float s;
    Vector v;

    Quaternion();
    Quaternion(float s, const Vector& v);
    
    static Quaternion identity();
    static Quaternion from_sv(float s, float x, float y, float z);
    static Quaternion from_axis_angle(const Vector& axis, float angle);

    Vector rotate_vector(const Vector& vec) const;
    float magnitude() const;
    Quaternion normalize() const;
    Quaternion conjugate() const;

    Quaternion operator*(const Quaternion& other) const;

    nlohmann::ordered_json to_json_data() const;
    static Quaternion from_json_data(const nlohmann::json& data);
    void to_json(const std::string& filepath) const;
    static Quaternion from_json(const std::string& filepath);
};

}  // namespace session_cpp
