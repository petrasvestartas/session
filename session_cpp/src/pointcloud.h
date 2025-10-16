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
public:
    std::string guid = ::guid();
    std::string name = "my_pointcloud";
    std::vector<Point> points;
    std::vector<Vector> normals;
    std::vector<Color> colors;
    Xform xform;
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
    // Transformation
    ///////////////////////////////////////////////////////////////////////////////////////////

    void transform();
    PointCloud transformed() const;

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Convert to JSON-serializable object
    nlohmann::ordered_json jsondump() const;
    
    /// Create point cloud from JSON data
    static PointCloud jsonload(const nlohmann::json& data);
    
    /// Serialize to JSON file
    
    /// Deserialize from JSON file

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
    size_t size() const { return points.size(); }
    
    /// Check if point cloud is empty
    bool empty() const { return points.empty(); }
};

/// Stream output operator for point cloud
std::ostream& operator<<(std::ostream& os, const PointCloud& cloud);

} // namespace session_cpp
