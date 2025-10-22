
#pragma once
#include "color.h"
#include "fmt/core.h"
#include "guid.h"
#include "json.h"
#include "vector.h"
#include <array>
#include <cmath>
#include <fstream>
#include <iostream>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>

namespace session_cpp {

class Point; // Forward declaration

/**
 * @class Xform
 * @brief A 4x4 transformation matrix.
 */
class Xform {
public:
    std::string guid = ::guid();      ///< Unique identifier generated on construction
    std::string name = "my_xform";   ///< Human-readable name
    std::array<double, 16> m;          ///< Column-major 4x4 matrix values

    /// Default constructor (identity)
    Xform();
    /// Construct from matrix values (column-major)
    Xform(const std::array<double, 16>& matrix);

    /// Identity matrix
    static Xform identity();
    /// Build from array values (column-major)
    static Xform from_matrix(const std::array<double, 16>& matrix);
    
    ///////////////////////////////////////////////////////////////////////////////////////////
    // Transformations
    ///////////////////////////////////////////////////////////////////////////////////////////
    
    /// Translation by x, y, z
    static Xform translation(double x, double y, double z);
    /// Non-uniform scale by x, y, z
    static Xform scaling(double x, double y, double z);
    /// Rotation around X axis (radians)
    static Xform rotation_x(double angle_radians);
    /// Rotation around Y axis (radians)
    static Xform rotation_y(double angle_radians);
    /// Rotation around Z axis (radians)
    static Xform rotation_z(double angle_radians);
    /// Rotation around arbitrary axis (radians)
    static Xform rotation(Vector& axis, double angle_radians);
    /// Change of basis from origin and axes
    static Xform change_basis(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis);
    /// Alternative change of basis (explicit from/to bases)
    static Xform change_basis_alt(Point& origin_1, Vector& x_axis_1, Vector& y_axis_1, Vector& z_axis_1,
                                   Point& origin_0, Vector& x_axis_0, Vector& y_axis_0, Vector& z_axis_0);
    /// Transform mapping one plane (origin and axes) to another
    static Xform plane_to_plane(Point& origin_0, Vector& x_axis_0, Vector& y_axis_0, Vector& z_axis_0,
                                Point& origin_1, Vector& x_axis_1, Vector& y_axis_1, Vector& z_axis_1);
    /// Transform from a plane coordinate system to XY
    static Xform plane_to_xy(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis);
    /// Transform from XY to a plane coordinate system
    static Xform xy_to_plane(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis);
    /// Scale along XYZ about the world origin
    static Xform scale_xyz(double scale_x, double scale_y, double scale_z);
    /// Uniform scale about a point
    static Xform scale_uniform(Point& origin, double scale_value);
    /// Non-uniform scale about a point
    static Xform scale_non_uniform(Point& origin, double scale_x, double scale_y, double scale_z);
    /// Axis-angle rotation using Rodrigues' formula
    static Xform axis_rotation(double angle, Vector& axis);
    /// Right-handed look-at matrix
    static Xform look_at_rh(const Point& eye, const Point& target, const Vector& up);

    /// Matrix inverse if invertible
    std::optional<Xform> inverse() const;
    /// Check if matrix is identity (within tolerance)
    bool is_identity() const;

    /// Return transformed copy of a point
    Point transformed_point(const Point& point) const;
    /// Return transformed copy of a vector
    Vector transformed_vector(const Vector& vector) const;
    /// Transform a point in place
    void transform_point(Point& point) const;
    /// Transform a vector in place
    void transform_vector(Vector& vector) const;

    /// Serialize to ordered JSON object
    nlohmann::ordered_json jsondump() const;
    /// Deserialize from JSON object
    static Xform jsonload(const nlohmann::json& data);
    /// Write JSON to file
    /// Read JSON from file

    /// Matrix multiplication
    Xform operator*(const Xform& other) const;
    /// In-place matrix multiplication
    Xform& operator*=(const Xform& other);

    /// Matrix element (row, col) mutable accessor
    double& operator()(int row, int col);
    /// Matrix element (row, col) const accessor
    const double& operator()(int row, int col) const;

};

} // namespace session_cpp