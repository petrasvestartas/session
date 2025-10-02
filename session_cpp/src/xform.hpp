
#pragma once
#include "color.h"
#include "fmt/core.h"
#include "guid.h"
#include "json.h"
#include "vector.h"
#include "point.h"
#include <array>
#include <cmath>
#include <fstream>
#include <iostream>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>

namespace session_cpp {

class Xform {
public:
    std::string guid = ::guid();
    std::string name = "my_xform";
    std::array<float, 16> m;

    Xform();
    Xform(const std::array<float, 16>& matrix);

    static Xform identity();
    static Xform from_matrix(const std::array<float, 16>& matrix);
    
    ///////////////////////////////////////////////////////////////////////////////////////////
    // Transformations
    ///////////////////////////////////////////////////////////////////////////////////////////
    
    static Xform translation(float x, float y, float z);
    static Xform scaling(float x, float y, float z);
    static Xform rotation_x(float angle_radians);
    static Xform rotation_y(float angle_radians);
    static Xform rotation_z(float angle_radians);
    static Xform rotation(Vector& axis, float angle_radians);
    static Xform change_basis(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis);
    static Xform change_basis_alt(Point& origin_1, Vector& x_axis_1, Vector& y_axis_1, Vector& z_axis_1,
                                   Point& origin_0, Vector& x_axis_0, Vector& y_axis_0, Vector& z_axis_0);
    static Xform plane_to_plane(Point& origin_0, Vector& x_axis_0, Vector& y_axis_0, Vector& z_axis_0,
                                Point& origin_1, Vector& x_axis_1, Vector& y_axis_1, Vector& z_axis_1);
    static Xform plane_to_xy(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis);
    static Xform xy_to_plane(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis);
    static Xform scale_xyz(float scale_x, float scale_y, float scale_z);
    static Xform scale_uniform(Point& origin, float scale_value);
    static Xform scale_non_uniform(Point& origin, float scale_x, float scale_y, float scale_z);
    static Xform axis_rotation(float angle, Vector& axis);

    std::optional<Xform> inverse() const;
    bool is_identity() const;

    Point transformed_point(const Point& point) const;
    Vector transformed_vector(const Vector& vector) const;
    void transform_point(Point& point) const;
    void transform_vector(Vector& vector) const;

    nlohmann::json to_json_data() const;
    static Xform from_json_data(const nlohmann::json& data);
    void to_json(const std::string& filepath) const;
    static Xform from_json(const std::string& filepath);

    Xform operator*(const Xform& other) const;
    Xform& operator*=(const Xform& other);

};

} // namespace session_cpp