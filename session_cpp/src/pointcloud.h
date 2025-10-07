#pragma once

#include "point.h"
#include "vector.h"
#include "color.h"
#include "xform.hpp"
#include "json.h"
#include <vector>
#include <string>

namespace session_cpp {

/**
 * @class PointCloud
 * @brief A point cloud with points, normals, colors, and transformation.
 */
class PointCloud {
private:
    std::string _guid;
    std::string _name;
    std::vector<Point> _points;
    std::vector<Vector> _normals;
    std::vector<Color> _colors;
    Xform _xform;

public:
    /// Getters
    const std::string& guid() const { return _guid; }
    const std::string& name() const { return _name; }
    const std::vector<Point>& points() const { return _points; }
    const std::vector<Vector>& normals() const { return _normals; }
    const std::vector<Color>& colors() const { return _colors; }
    const Xform& xform() const { return _xform; }

    /// Setters
    void set_guid(const std::string& guid) { _guid = guid; }
    void set_name(const std::string& name) { _name = name; }
    void set_xform(const Xform& xform) { _xform = xform; }

    /// Constructors
    PointCloud();
    PointCloud(const std::vector<Point>& points, 
               const std::vector<Vector>& normals, 
               const std::vector<Color>& colors);

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Operators
    ///////////////////////////////////////////////////////////////////////////////////////////

    std::string to_string() const;
    bool operator==(const PointCloud& other) const;
    bool operator!=(const PointCloud& other) const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    nlohmann::ordered_json to_json_data() const;
    static PointCloud from_json_data(const nlohmann::json& data);
    void to_json(const std::string& filepath) const;
    static PointCloud from_json(const std::string& filepath);

    ///////////////////////////////////////////////////////////////////////////////////////////
    // No-copy Operators
    ///////////////////////////////////////////////////////////////////////////////////////////

    PointCloud& operator+=(const Vector& v);
    PointCloud& operator-=(const Vector& v);

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Copy Operators
    ///////////////////////////////////////////////////////////////////////////////////////////

    PointCloud operator+(const Vector& v) const;
    PointCloud operator-(const Vector& v) const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Details
    ///////////////////////////////////////////////////////////////////////////////////////////

    size_t size() const { return _points.size(); }
    bool empty() const { return _points.empty(); }
};

std::ostream& operator<<(std::ostream& os, const PointCloud& cloud);

} // namespace session_cpp
