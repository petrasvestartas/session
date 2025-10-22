#pragma once

#include "vector.h"
#include "json.h"
#include <string>

namespace session_cpp {

/**
 * @class Quaternion
 * @brief A quaternion for 3D rotations.
 */
class Quaternion {
private:
    std::string typ;   ///< Type tag used for JSON serialization (always "Quaternion")

public:
    std::string guid;  ///< Unique identifier generated on construction
    std::string name;  ///< Human-readable name
    double s;          ///< Scalar component
    Vector v;          ///< Vector component (x, y, z)

    /**
     * @brief Default constructor (identity quaternion).
     */
    Quaternion();

    /**
     * @brief Construct from scalar and vector parts.
     * @param s Scalar component.
     * @param v Vector component.
     */
    Quaternion(double s, const Vector& v);
    
    /// Get the type tag (always "Quaternion")
    const std::string& type() const { return typ; }
    
    /// Create identity quaternion (no rotation)
    static Quaternion identity();
    /// Create quaternion from scalar and vector components
    static Quaternion from_sv(double s, double x, double y, double z);
    /// Create quaternion from rotation axis and angle (radians)
    static Quaternion from_axis_angle(const Vector& axis, double angle);

    /// Rotate a vector by this quaternion
    Vector rotate_vector(const Vector& vec) const;
    /// Magnitude (norm) of the quaternion
    double magnitude() const;
    /// Normalized quaternion (unit length)
    Quaternion normalize() const;
    /// Conjugate of the quaternion
    Quaternion conjugate() const;

    /// Hamilton product between two quaternions
    Quaternion operator*(const Quaternion& other) const;

    /// Serialize to ordered JSON object
    nlohmann::ordered_json jsondump() const;
    /// Deserialize from JSON object
    static Quaternion jsonload(const nlohmann::json& data);
    /// Write JSON to file
    /// Read JSON from file
};

}  // namespace session_cpp
