#include "boundingbox.h"
#include "line.h"
#include "polyline.h"
#include "mesh.h"
#include "pointcloud.h"
#include "guid.h"
#include <fstream>
#include <cmath>
#include <algorithm>

namespace session_cpp {

BoundingBox::BoundingBox() 
    : center(0.0f, 0.0f, 0.0f),
      x_axis(1.0f, 0.0f, 0.0f),
      y_axis(0.0f, 1.0f, 0.0f),
      z_axis(0.0f, 0.0f, 1.0f),
      half_size(0.5f, 0.5f, 0.5f),
      guid(::guid()),
      name("my_boundingbox") {}

BoundingBox::BoundingBox(const Point& center, const Vector& x_axis, const Vector& y_axis, const Vector& z_axis, const Vector& half_size)
    : center(center),
      x_axis(x_axis),
      y_axis(y_axis),
      z_axis(z_axis),
      half_size(half_size),
      guid(::guid()),
      name("my_boundingbox") {}

BoundingBox::BoundingBox(const Plane& plane, float dx, float dy, float dz)
    : center(plane.origin()),
      x_axis(plane.x_axis()),
      y_axis(plane.y_axis()),
      z_axis(plane.z_axis()),
      half_size(dx * 0.5f, dy * 0.5f, dz * 0.5f),
      guid(::guid()),
      name("") {}

BoundingBox BoundingBox::from_point(const Point& point, float inflate_amount) {
    BoundingBox box(point, Vector(1.0f, 0.0f, 0.0f), Vector(0.0f, 1.0f, 0.0f), Vector(0.0f, 0.0f, 1.0f), Vector(inflate_amount, inflate_amount, inflate_amount));
    return box;
}

BoundingBox BoundingBox::from_points(const std::vector<Point>& points, float inflate_amount) {
    if (points.empty()) {
        return BoundingBox();
    }
    
    float min_x = std::numeric_limits<float>::max();
    float min_y = std::numeric_limits<float>::max();
    float min_z = std::numeric_limits<float>::max();
    float max_x = std::numeric_limits<float>::lowest();
    float max_y = std::numeric_limits<float>::lowest();
    float max_z = std::numeric_limits<float>::lowest();
    
    for (const auto& pt : points) {
        min_x = std::min(min_x, pt.x());
        min_y = std::min(min_y, pt.y());
        min_z = std::min(min_z, pt.z());
        max_x = std::max(max_x, pt.x());
        max_y = std::max(max_y, pt.y());
        max_z = std::max(max_z, pt.z());
    }
    
    Point center((min_x + max_x) * 0.5f, (min_y + max_y) * 0.5f, (min_z + max_z) * 0.5f);
    Vector half_size(
        (max_x - min_x) * 0.5f + inflate_amount,
        (max_y - min_y) * 0.5f + inflate_amount,
        (max_z - min_z) * 0.5f + inflate_amount
    );
    
    return BoundingBox(center, Vector(1.0f, 0.0f, 0.0f), Vector(0.0f, 1.0f, 0.0f), Vector(0.0f, 0.0f, 1.0f), half_size);
}

BoundingBox BoundingBox::from_line(const Line& line, float inflate_amount) {
    std::vector<Point> points = {line.start(), line.end()};
    return from_points(points, inflate_amount);
}

BoundingBox BoundingBox::from_polyline(const Polyline& polyline, float inflate_amount) {
    return from_points(polyline.points, inflate_amount);
}

BoundingBox BoundingBox::from_mesh(const Mesh& mesh, float inflate_amount) {
    auto [vertices, faces] = mesh.to_vertices_and_faces();
    return from_points(vertices, inflate_amount);
}

BoundingBox BoundingBox::from_pointcloud(const PointCloud& pointcloud, float inflate_amount) {
    return from_points(pointcloud.points(), inflate_amount);
}

Point BoundingBox::point_at(float x, float y, float z) const {
    return Point(
        center.x() + x * x_axis.x() + y * y_axis.x() + z * z_axis.x(),
        center.y() + x * x_axis.y() + y * y_axis.y() + z * z_axis.y(),
        center.z() + x * x_axis.z() + y * y_axis.z() + z * z_axis.z()
    );
}

std::array<Point, 8> BoundingBox::corners() const {
    std::array<Point, 8> result;
    
    result[0] = point_at(half_size.x(), half_size.y(), -half_size.z());
    result[1] = point_at(-half_size.x(), half_size.y(), -half_size.z());
    result[2] = point_at(-half_size.x(), -half_size.y(), -half_size.z());
    result[3] = point_at(half_size.x(), -half_size.y(), -half_size.z());
    
    result[4] = point_at(half_size.x(), half_size.y(), half_size.z());
    result[5] = point_at(-half_size.x(), half_size.y(), half_size.z());
    result[6] = point_at(-half_size.x(), -half_size.y(), half_size.z());
    result[7] = point_at(half_size.x(), -half_size.y(), half_size.z());
    
    return result;
}

std::array<Point, 10> BoundingBox::two_rectangles() const {
    std::array<Point, 10> result;
    
    result[0] = point_at(half_size.x(), half_size.y(), -half_size.z());
    result[1] = point_at(-half_size.x(), half_size.y(), -half_size.z());
    result[2] = point_at(-half_size.x(), -half_size.y(), -half_size.z());
    result[3] = point_at(half_size.x(), -half_size.y(), -half_size.z());
    result[4] = point_at(half_size.x(), half_size.y(), -half_size.z());
    
    result[5] = point_at(half_size.x(), half_size.y(), half_size.z());
    result[6] = point_at(-half_size.x(), half_size.y(), half_size.z());
    result[7] = point_at(-half_size.x(), -half_size.y(), half_size.z());
    result[8] = point_at(half_size.x(), -half_size.y(), half_size.z());
    result[9] = point_at(half_size.x(), half_size.y(), half_size.z());
    
    return result;
}

void BoundingBox::inflate(float amount) {
    half_size = Vector(
        half_size.x() + amount,
        half_size.y() + amount,
        half_size.z() + amount
    );
}

bool BoundingBox::separating_plane_exists(const Vector& relative_position, const Vector& axis, const BoundingBox& box1, const BoundingBox& box2) {
    Vector rp = relative_position;
    Vector ax = axis;
    float dot_rp = std::abs(rp.dot(ax));
    
    Vector v1 = box1.x_axis * box1.half_size.x();
    Vector v2 = box1.y_axis * box1.half_size.y();
    Vector v3 = box1.z_axis * box1.half_size.z();
    Vector ax1 = axis;
    float proj1 = std::abs(v1.dot(ax1)) + std::abs(v2.dot(ax1)) + std::abs(v3.dot(ax1));
    
    Vector v4 = box2.x_axis * box2.half_size.x();
    Vector v5 = box2.y_axis * box2.half_size.y();
    Vector v6 = box2.z_axis * box2.half_size.z();
    Vector ax2 = axis;
    float proj2 = std::abs(v4.dot(ax2)) + std::abs(v5.dot(ax2)) + std::abs(v6.dot(ax2));
    
    return dot_rp > (proj1 + proj2);
}

bool BoundingBox::collides_with(const BoundingBox& other) const {
    Vector center_vec(center.x(), center.y(), center.z());
    Vector other_center_vec(other.center.x(), other.center.y(), other.center.z());
    Vector relative_position = Vector::from_start_and_end(center_vec, other_center_vec);
    
    Vector x1 = x_axis, y1 = y_axis, z1 = z_axis;
    Vector x2 = other.x_axis, y2 = other.y_axis, z2 = other.z_axis;
    
    return !(
        separating_plane_exists(relative_position, x1, *this, other) ||
        separating_plane_exists(relative_position, y1, *this, other) ||
        separating_plane_exists(relative_position, z1, *this, other) ||
        separating_plane_exists(relative_position, x2, *this, other) ||
        separating_plane_exists(relative_position, y2, *this, other) ||
        separating_plane_exists(relative_position, z2, *this, other) ||
        separating_plane_exists(relative_position, x1.cross(x2), *this, other) ||
        separating_plane_exists(relative_position, x1.cross(y2), *this, other) ||
        separating_plane_exists(relative_position, x1.cross(z2), *this, other) ||
        separating_plane_exists(relative_position, y1.cross(x2), *this, other) ||
        separating_plane_exists(relative_position, y1.cross(y2), *this, other) ||
        separating_plane_exists(relative_position, y1.cross(z2), *this, other) ||
        separating_plane_exists(relative_position, z1.cross(y2), *this, other) ||
        separating_plane_exists(relative_position, z1.cross(z2), *this, other)
    );
}

nlohmann::ordered_json BoundingBox::to_json_data() const {
    return {
        {"center", center.to_json_data()},
        {"x_axis", x_axis.to_json_data()},
        {"y_axis", y_axis.to_json_data()},
        {"z_axis", z_axis.to_json_data()},
        {"half_size", half_size.to_json_data()},
        {"guid", guid},
        {"name", name}
    };
}

BoundingBox BoundingBox::from_json_data(const nlohmann::json& data) {
    BoundingBox box;
    box.center = Point::from_json_data(data["center"]);
    box.x_axis = Vector::from_json_data(data["x_axis"]);
    box.y_axis = Vector::from_json_data(data["y_axis"]);
    box.z_axis = Vector::from_json_data(data["z_axis"]);
    box.half_size = Vector::from_json_data(data["half_size"]);
    box.guid = data["guid"];
    box.name = data["name"];
    return box;
}

void BoundingBox::to_json_file(const std::string& filepath) const {
    std::ofstream file(filepath);
    file << to_json_data().dump(4);
}

BoundingBox BoundingBox::from_json_file(const std::string& filepath) {
    std::ifstream file(filepath);
    nlohmann::json data;
    file >> data;
    return from_json_data(data);
}

}
