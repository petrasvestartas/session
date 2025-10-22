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
    : center(0.0, 0.0, 0.0),
      x_axis(1.0, 0.0, 0.0),
      y_axis(0.0, 1.0, 0.0),
      z_axis(0.0, 0.0, 1.0),
      half_size(0.5, 0.5, 0.5),
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

BoundingBox::BoundingBox(const Plane& plane, double dx, double dy, double dz)
    : center(plane.origin()),
      x_axis(plane.x_axis()),
      y_axis(plane.y_axis()),
      z_axis(plane.z_axis()),
      half_size(dx * 0.5, dy * 0.5, dz * 0.5),
      guid(::guid()),
      name("") {}

BoundingBox BoundingBox::from_point(const Point& point, double inflate_amount) {
    BoundingBox box(point, Vector(1.0, 0.0, 0.0), Vector(0.0, 1.0, 0.0), Vector(0.0, 0.0, 1.0), Vector(inflate_amount, inflate_amount, inflate_amount));
    return box;
}

BoundingBox BoundingBox::from_points(const std::vector<Point>& points, double inflate_amount) {
    if (points.empty()) {
        return BoundingBox();
    }
    
    double min_x = std::numeric_limits<double>::max();
    double min_y = std::numeric_limits<double>::max();
    double min_z = std::numeric_limits<double>::max();
    double max_x = std::numeric_limits<double>::lowest();
    double max_y = std::numeric_limits<double>::lowest();
    double max_z = std::numeric_limits<double>::lowest();
    
    for (const auto& pt : points) {
        min_x = std::min(min_x, pt.x());
        min_y = std::min(min_y, pt.y());
        min_z = std::min(min_z, pt.z());
        max_x = std::max(max_x, pt.x());
        max_y = std::max(max_y, pt.y());
        max_z = std::max(max_z, pt.z());
    }
    
    Point center((min_x + max_x) * 0.5, (min_y + max_y) * 0.5, (min_z + max_z) * 0.5);
    Vector half_size(
        (max_x - min_x) * 0.5 + inflate_amount,
        (max_y - min_y) * 0.5 + inflate_amount,
        (max_z - min_z) * 0.5 + inflate_amount
    );
    return BoundingBox(center, Vector(1.0, 0.0, 0.0), Vector(0.0, 1.0, 0.0), Vector(0.0, 0.0, 1.0), half_size);
}

BoundingBox BoundingBox::from_line(const Line& line, double inflate_amount) {
    std::vector<Point> points = {line.start(), line.end()};
    return from_points(points, inflate_amount);
}

BoundingBox BoundingBox::from_polyline(const Polyline& polyline, double inflate_amount) {
    return from_points(polyline.points, inflate_amount);
}

BoundingBox BoundingBox::from_mesh(const Mesh& mesh, double inflate_amount) {
    auto [vertices, faces] = mesh.to_vertices_and_faces();
    return from_points(vertices, inflate_amount);
}

BoundingBox BoundingBox::from_pointcloud(const PointCloud& pointcloud, double inflate_amount) {
    return from_points(pointcloud.points, inflate_amount);
}

Point BoundingBox::point_at(double x, double y, double z) const {
    return Point(
        center.x() + x * x_axis.x() + y * y_axis.x() + z * z_axis.x(),
        center.y() + x * x_axis.y() + y * y_axis.y() + z * z_axis.y(),
        center.z() + x * x_axis.z() + y * y_axis.z() + z * z_axis.z()
    );
}

Point BoundingBox::min_point() const {
    return Point(
        center.x() - half_size.x(),
        center.y() - half_size.y(),
        center.z() - half_size.z()
    );
}

Point BoundingBox::max_point() const {
    return Point(
        center.x() + half_size.x(),
        center.y() + half_size.y(),
        center.z() + half_size.z()
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

void BoundingBox::inflate(double amount) {
    half_size = Vector(
        half_size.x() + amount,
        half_size.y() + amount,
        half_size.z() + amount
    );
}

bool BoundingBox::separating_plane_exists(const Vector& relative_position, const Vector& axis, const BoundingBox& box1, const BoundingBox& box2) {
    Vector rp = relative_position;
    Vector ax = axis;
    double dot_rp = std::abs(rp.dot(ax));
    
    Vector v1 = box1.x_axis * box1.half_size.x();
    Vector v2 = box1.y_axis * box1.half_size.y();
    Vector v3 = box1.z_axis * box1.half_size.z();
    Vector ax1 = axis;
    double proj1 = std::abs(v1.dot(ax1)) + std::abs(v2.dot(ax1)) + std::abs(v3.dot(ax1));
    
    Vector v4 = box2.x_axis * box2.half_size.x();
    Vector v5 = box2.y_axis * box2.half_size.y();
    Vector v6 = box2.z_axis * box2.half_size.z();
    Vector ax2 = axis;
    double proj2 = std::abs(v4.dot(ax2)) + std::abs(v5.dot(ax2)) + std::abs(v6.dot(ax2));
    return dot_rp > (proj1 + proj2);
}

void BoundingBox::transform() {
  xform.transform_point(center);
  xform.transform_vector(x_axis);
  xform.transform_vector(y_axis);
  xform.transform_vector(z_axis);
  xform = Xform::identity();
}

BoundingBox BoundingBox::transformed() const {
  BoundingBox result = *this;
  result.transform();
  return result;
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

nlohmann::ordered_json BoundingBox::jsondump() const {
    return {
        {"type", "BoundingBox"},
        {"center", center.jsondump()},
        {"x_axis", x_axis.jsondump()},
        {"y_axis", y_axis.jsondump()},
        {"z_axis", z_axis.jsondump()},
        {"half_size", half_size.jsondump()},
        {"guid", guid},
        {"name", name}
    };
}

BoundingBox BoundingBox::jsonload(const nlohmann::json& data) {
    BoundingBox box;
    box.center = Point::jsonload(data["center"]);
    box.x_axis = Vector::jsonload(data["x_axis"]);
    box.y_axis = Vector::jsonload(data["y_axis"]);
    box.z_axis = Vector::jsonload(data["z_axis"]);
    box.half_size = Vector::jsonload(data["half_size"]);
    box.guid = data["guid"];
    box.name = data["name"];
    return box;
}

void BoundingBox::to_json_file(const std::string& filepath) const {
    std::ofstream file(filepath);
    file << jsondump().dump(4);
}

BoundingBox BoundingBox::from_json_file(const std::string& filepath) {
    std::ifstream file(filepath);
    nlohmann::json data;
    file >> data;
    return jsonload(data);
}

}
