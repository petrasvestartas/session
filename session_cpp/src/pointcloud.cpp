#include "pointcloud.h"
#include <fstream>
#include <sstream>
#include <uuid/uuid.h>

namespace session_cpp {

PointCloud::PointCloud() {
    uuid_t uuid;
    uuid_generate(uuid);
    char uuid_str[37];
    uuid_unparse(uuid, uuid_str);
    _guid = uuid_str;
    _name = "my_pointcloud";
    _xform = Xform::identity();
}

PointCloud::PointCloud(const std::vector<Point>& points, 
                       const std::vector<Vector>& normals, 
                       const std::vector<Color>& colors)
    : _points(points), _normals(normals), _colors(colors) {
    uuid_t uuid;
    uuid_generate(uuid);
    char uuid_str[37];
    uuid_unparse(uuid, uuid_str);
    _guid = uuid_str;
    _name = "my_pointcloud";
    _xform = Xform::identity();
}

///////////////////////////////////////////////////////////////////////////////////////////
// Operators
///////////////////////////////////////////////////////////////////////////////////////////

std::string PointCloud::to_string() const {
    std::ostringstream oss;
    oss << "PointCloud(points=" << _points.size() 
        << ", normals=" << _normals.size() 
        << ", colors=" << _colors.size() 
        << ", guid=" << _guid 
        << ", name=" << _name << ")";
    return oss.str();
}

bool PointCloud::operator==(const PointCloud& other) const {
    return _guid == other._guid;
}

bool PointCloud::operator!=(const PointCloud& other) const {
    return !(*this == other);
}

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

nlohmann::ordered_json PointCloud::to_json_data() const {
    // Flatten points to [x, y, z, x, y, z, ...]
    std::vector<float> points_flat;
    points_flat.reserve(_points.size() * 3);
    for (const auto& p : _points) {
        points_flat.push_back(p.x());
        points_flat.push_back(p.y());
        points_flat.push_back(p.z());
    }

    // Flatten normals to [x, y, z, x, y, z, ...]
    std::vector<float> normals_flat;
    normals_flat.reserve(_normals.size() * 3);
    for (const auto& n : _normals) {
        normals_flat.push_back(n.x());
        normals_flat.push_back(n.y());
        normals_flat.push_back(n.z());
    }

    // Flatten colors to [r, g, b, r, g, b, ...] (no alpha)
    std::vector<int> colors_flat;
    colors_flat.reserve(_colors.size() * 3);
    for (const auto& c : _colors) {
        colors_flat.push_back(c.r);
        colors_flat.push_back(c.g);
        colors_flat.push_back(c.b);
    }

    return nlohmann::ordered_json{
        {"type", "PointCloud"},
        {"guid", _guid},
        {"name", _name},
        {"points", points_flat},
        {"normals", normals_flat},
        {"colors", colors_flat},
        {"xform", _xform.to_json_data()}
    };
}

PointCloud PointCloud::from_json_data(const nlohmann::json& data) {
    PointCloud cloud;
    cloud._guid = data["guid"];
    cloud._name = data["name"];

    // Reconstruct points from flat array
    const auto& points_flat = data["points"];
    cloud._points.clear();
    for (size_t i = 0; i < points_flat.size(); i += 3) {
        cloud._points.emplace_back(points_flat[i], points_flat[i+1], points_flat[i+2]);
    }

    // Reconstruct normals from flat array
    const auto& normals_flat = data["normals"];
    cloud._normals.clear();
    for (size_t i = 0; i < normals_flat.size(); i += 3) {
        cloud._normals.emplace_back(normals_flat[i], normals_flat[i+1], normals_flat[i+2]);
    }

    // Reconstruct colors from flat array (RGB only, alpha always 255)
    const auto& colors_flat = data["colors"];
    cloud._colors.clear();
    for (size_t i = 0; i < colors_flat.size(); i += 3) {
        cloud._colors.emplace_back(colors_flat[i], colors_flat[i+1], colors_flat[i+2], 255);
    }

    cloud._xform = Xform::from_json_data(data["xform"]);

    return cloud;
}

void PointCloud::to_json(const std::string& filepath) const {
    std::ofstream file(filepath);
    file << to_json_data().dump(4);
}

PointCloud PointCloud::from_json(const std::string& filepath) {
    std::ifstream file(filepath);
    nlohmann::json data;
    file >> data;
    return from_json_data(data);
}

///////////////////////////////////////////////////////////////////////////////////////////
// No-copy Operators
///////////////////////////////////////////////////////////////////////////////////////////

PointCloud& PointCloud::operator+=(const Vector& v) {
    for (auto& p : _points) {
        p += v;
    }
    return *this;
}

PointCloud& PointCloud::operator-=(const Vector& v) {
    for (auto& p : _points) {
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
