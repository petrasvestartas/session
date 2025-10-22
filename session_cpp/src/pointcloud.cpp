#include "pointcloud.h"
#include "guid.h"
#include <fstream>
#include <sstream>

namespace session_cpp {

PointCloud::PointCloud() {
    xform = Xform::identity();
}

PointCloud::PointCloud(const std::vector<Point>& pts, 
                       const std::vector<Vector>& norms, 
                       const std::vector<Color>& cols)
    : points(pts), normals(norms), colors(cols) {
    xform = Xform::identity();
}
///////////////////////////////////////////////////////////////////////////////////////////
// Operators
///////////////////////////////////////////////////////////////////////////////////////////

std::string PointCloud::to_string() const {
    std::ostringstream oss;
    oss << "PointCloud(points=" << points.size() 
        << ", normals=" << normals.size() 
        << ", colors=" << colors.size() 
        << ", guid=" << guid 
        << ", name=" << name << ")";
    return oss.str();
}

bool PointCloud::operator==(const PointCloud& other) const {
    return guid == other.guid;
}

bool PointCloud::operator!=(const PointCloud& other) const {
    return !(*this == other);
}

///////////////////////////////////////////////////////////////////////////////////////////
// Transformation
///////////////////////////////////////////////////////////////////////////////////////////

void PointCloud::transform() {
  for (auto& pt : points) {
    xform.transform_point(pt);
  }
  for (auto& n : normals) {
    xform.transform_vector(n);
  }
  xform = Xform::identity();
}

PointCloud PointCloud::transformed() const {
  PointCloud result = *this;
  result.transform();
  return result;
}

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

nlohmann::ordered_json PointCloud::jsondump() const {
    // Flatten points to [x, y, z, x, y, z, ...]
    std::vector<double> points_flat;
    points_flat.reserve(points.size() * 3);
    for (const auto& p : points) {
        points_flat.push_back(p.x());
        points_flat.push_back(p.y());
        points_flat.push_back(p.z());
    }

    // Flatten normals to [x, y, z, x, y, z, ...]
    std::vector<double> normals_flat;
    normals_flat.reserve(normals.size() * 3);
    for (const auto& n : normals) {
        normals_flat.push_back(n.x());
        normals_flat.push_back(n.y());
        normals_flat.push_back(n.z());
    }

    // Flatten colors to [r, g, b, r, g, b, ...] (no alpha)
    std::vector<int> colors_flat;
    colors_flat.reserve(colors.size() * 3);
    for (const auto& c : colors) {
        colors_flat.push_back(c.r);
        colors_flat.push_back(c.g);
        colors_flat.push_back(c.b);
    }

    return nlohmann::ordered_json{
        {"type", "PointCloud"},
        {"guid", guid},
        {"name", name},
        {"points", points_flat},
        {"normals", normals_flat},
        {"colors", colors_flat},
        {"xform", xform.jsondump()}
    };
}

PointCloud PointCloud::jsonload(const nlohmann::json& data) {
    PointCloud cloud;
    cloud.guid = data["guid"];
    cloud.name = data["name"];

    // Reconstruct points from flat array
    const auto& points_flat = data["points"];
    cloud.points.clear();
    for (size_t i = 0; i < points_flat.size(); i += 3) {
        cloud.points.emplace_back(points_flat[i], points_flat[i+1], points_flat[i+2]);
    }

    // Reconstruct normals from flat array
    const auto& normals_flat = data["normals"];
    cloud.normals.clear();
    for (size_t i = 0; i < normals_flat.size(); i += 3) {
        cloud.normals.emplace_back(normals_flat[i], normals_flat[i+1], normals_flat[i+2]);
    }

    // Reconstruct colors from flat array (RGB only, alpha always 255)
    const auto& colors_flat = data["colors"];
    cloud.colors.clear();
    for (size_t i = 0; i < colors_flat.size(); i += 3) {
        cloud.colors.emplace_back(colors_flat[i], colors_flat[i+1], colors_flat[i+2], 255);
    }

    cloud.xform = Xform::jsonload(data["xform"]);

    return cloud;
}



///////////////////////////////////////////////////////////////////////////////////////////
// No-copy Operators
///////////////////////////////////////////////////////////////////////////////////////////

PointCloud& PointCloud::operator+=(const Vector& v) {
    for (auto& p : points) {
        p += v;
    }
    return *this;
}

PointCloud& PointCloud::operator-=(const Vector& v) {
    for (auto& p : points) {
        p -= v;
    }
    return *this;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Copy Operators
///////////////////////////////////////////////////////////////////////////////////////////

PointCloud PointCloud::operator+(const Vector& v) const {
    PointCloud result = *this;
    result += v;
    return result;
}

PointCloud PointCloud::operator-(const Vector& v) const {
    PointCloud result = *this;
    result -= v;
    return result;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Stream operator
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream& operator<<(std::ostream& os, const PointCloud& cloud) {
    return os << cloud.to_string();
}

} // namespace session_cpp
