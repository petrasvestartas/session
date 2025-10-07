#pragma once

#include "point.h"
#include "vector.h"
#include "color.h"
#include "xform.h"
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
    /// Get the unique identifier
    const std::string& guid() const { return _guid; }
    
    /// Get the name
    const std::string& name() const { return _name; }
    
    /// Get the collection of points
    const std::vector<Point>& points() const { return _points; }
    /// Get the collection of normals
    const std::vector<Vector>& normals() const { return _normals; }
    
    /// Get the collection of colors
    const std::vector<Color>& colors() const { return _colors; }
    
    /// Get the transformation matrix
    const Xform& xform() const { return _xform; }

    /// Set the name
    void set_name(const std::string& name) { _name = name; }
    
    /// Set the transformation matrix
    void set_xform(const Xform& xform) { _xform = xform; }
    /**
     * @brief Default constructor.
     */
    PointCloud();
    
    /**
     * @brief Constructor with points, normals, and colors.
     * @param points Collection of points.
     * @param normals Collection of normals.
     * @param colors Collection of colors.
     */
    PointCloud(const std::vector<Point>& points, 
               const std::vector<Vector>& normals, 
               const std::vector<Color>& colors);

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Operators
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Convert point cloud to string representation
    std::string to_string() const;
    
    /// Equality operator
    bool operator==(const PointCloud& other) const;
    
    /// Inequality operator
    bool operator!=(const PointCloud& other) const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Convert to JSON-serializable object
    nlohmann::ordered_json to_json_data() const;
    
    /// Create point cloud from JSON data
    static PointCloud from_json_data(const nlohmann::json& data);
    
    /// Serialize to JSON file
    void to_json(const std::string& filepath) const;
    
    /// Deserialize from JSON file
    static PointCloud from_json(const std::string& filepath);

    ///////////////////////////////////////////////////////////////////////////////////////////
    // No-copy Operators
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Translate point cloud by vector (in-place)
    PointCloud& operator+=(const Vector& v);
    
    /// Translate point cloud by negative vector (in-place)
    PointCloud& operator-=(const Vector& v);

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Copy Operators
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Translate point cloud by vector (returns new point cloud)
    PointCloud operator+(const Vector& v) const;
    
    /// Translate point cloud by negative vector (returns new point cloud)
    PointCloud operator-(const Vector& v) const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Details
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Get the number of points
    size_t size() const { return _points.size(); }
    
    /// Check if point cloud is empty
    bool empty() const { return _points.empty(); }
};

/// Stream output operator for point cloud
std::ostream& operator<<(std::ostream& os, const PointCloud& cloud);

} // namespace session_cpp
