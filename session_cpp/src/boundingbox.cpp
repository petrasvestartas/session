#include "boundingbox.h"
#include "line.h"
#include "polyline.h"
#include "mesh.h"
#include "pointcloud.h"
#include "arrow.h"
#include "cylinder.h"
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

BoundingBox BoundingBox::from_points(const std::vector<Point>& points, const Plane& plane, double inflate_amount) {
    if (points.empty()) {
        return BoundingBox();
    }
    
    Point origin = plane.origin();
    Vector x_axis = plane.x_axis();
    Vector y_axis = plane.y_axis();
    Vector z_axis = plane.z_axis();
    Xform plane_to_xy = Xform::plane_to_xy(origin, x_axis, y_axis, z_axis);
    
    double min_x = std::numeric_limits<double>::max();
    double min_y = std::numeric_limits<double>::max();
    double min_z = std::numeric_limits<double>::max();
    double max_x = std::numeric_limits<double>::lowest();
    double max_y = std::numeric_limits<double>::lowest();
    double max_z = std::numeric_limits<double>::lowest();
    
    for (const auto& pt : points) {
        Point local_pt = plane_to_xy.transformed_point(pt);
        min_x = std::min(min_x, local_pt.x());
        min_y = std::min(min_y, local_pt.y());
        min_z = std::min(min_z, local_pt.z());
        max_x = std::max(max_x, local_pt.x());
        max_y = std::max(max_y, local_pt.y());
        max_z = std::max(max_z, local_pt.z());
    }
    
    Point local_center((min_x + max_x) * 0.5, (min_y + max_y) * 0.5, (min_z + max_z) * 0.5);
    Vector half_size(
        (max_x - min_x) * 0.5 + inflate_amount,
        (max_y - min_y) * 0.5 + inflate_amount,
        (max_z - min_z) * 0.5 + inflate_amount
    );
    
    Xform xy_to_plane = Xform::xy_to_plane(origin, x_axis, y_axis, z_axis);
    Point world_center = xy_to_plane.transformed_point(local_center);
    
    return BoundingBox(world_center, x_axis, y_axis, z_axis, half_size);
}

BoundingBox BoundingBox::from_line(const Line& line, double inflate_amount) {
    std::vector<Point> points = {line.start(), line.end()};
    return from_points(points, inflate_amount);
}

BoundingBox BoundingBox::from_line(const Line& line, const Plane& plane, double inflate_amount) {
    std::vector<Point> points = {line.start(), line.end()};
    return from_points(points, plane, inflate_amount);
}

BoundingBox BoundingBox::from_polyline(const Polyline& polyline, double inflate_amount) {
    return from_points(polyline.points, inflate_amount);
}

BoundingBox BoundingBox::from_polyline(const Polyline& polyline, const Plane& plane, double inflate_amount) {
    return from_points(polyline.points, plane, inflate_amount);
}

BoundingBox BoundingBox::from_mesh(const Mesh& mesh, double inflate_amount) {
    auto [vertices, faces] = mesh.to_vertices_and_faces();
    return from_points(vertices, inflate_amount);
}

BoundingBox BoundingBox::from_mesh(const Mesh& mesh, const Plane& plane, double inflate_amount) {
    auto [vertices, faces] = mesh.to_vertices_and_faces();
    return from_points(vertices, plane, inflate_amount);
}

BoundingBox BoundingBox::from_pointcloud(const PointCloud& pointcloud, double inflate_amount) {
    return from_points(pointcloud.points, inflate_amount);
}

BoundingBox BoundingBox::from_pointcloud(const PointCloud& pointcloud, const Plane& plane, double inflate_amount) {
    return from_points(pointcloud.points, plane, inflate_amount);
}

BoundingBox BoundingBox::from_arrow(const Arrow& arrow, double inflate_amount) {
    const Line& ln = arrow.line;
    Point p0 = ln.start();
    Point p1 = ln.end();
    Point c((p0.x() + p1.x()) * 0.5, (p0.y() + p1.y()) * 0.5, (p0.z() + p1.z()) * 0.5);
    Vector axis = ln.to_vector();
    double L = ln.length();
    if (L <= 0.0) {
        axis = Vector(1.0, 0.0, 0.0);
    } else {
        axis.normalize_self();
    }
    Vector ux = axis;
    Vector uy;
    if (std::abs(ux.z()) < 0.9) {
        uy = Vector(0.0, 0.0, 1.0).cross(ux);
        uy.normalize_self();
    } else {
        uy = Vector(1.0, 0.0, 0.0).cross(ux);
        uy.normalize_self();
    }
    Vector uz = ux.cross(uy);
    uz.normalize_self();
    double r_eff = arrow.radius * 1.5;
    Vector half((L * 0.5) + inflate_amount, r_eff + inflate_amount, r_eff + inflate_amount);
    return BoundingBox(c, ux, uy, uz, half);
}

BoundingBox BoundingBox::from_arrow(const Arrow& arrow, const Plane& plane, double inflate_amount) {
    const Line& ln = arrow.line;
    Point p0 = ln.start();
    Point p1 = ln.end();
    Point c((p0.x() + p1.x()) * 0.5, (p0.y() + p1.y()) * 0.5, (p0.z() + p1.z()) * 0.5);
    Vector dir = ln.to_vector();
    double L = ln.length();
    if (L > 0.0) dir.normalize_self();
    double r_eff = arrow.radius * 1.5;
    Vector Ux = plane.x_axis();
    Vector Uy = plane.y_axis();
    Vector Uz = plane.z_axis();
    auto proj_half = [&](const Vector& U) {
        double d = std::abs(dir.dot(U));
        double radial = r_eff * std::sqrt(std::max(0.0, 1.0 - d * d));
        return (L * 0.5) * d + radial + inflate_amount;
    };
    Vector half(proj_half(Ux), proj_half(Uy), proj_half(Uz));
    return BoundingBox(c, Ux, Uy, Uz, half);
}

BoundingBox BoundingBox::from_cylinder(const Cylinder& cylinder, double inflate_amount) {
    const Line& ln = cylinder.line;
    Point p0 = ln.start();
    Point p1 = ln.end();
    Point c((p0.x() + p1.x()) * 0.5, (p0.y() + p1.y()) * 0.5, (p0.z() + p1.z()) * 0.5);
    Vector axis = ln.to_vector();
    double L = ln.length();
    if (L <= 0.0) {
        axis = Vector(1.0, 0.0, 0.0);
    } else {
        axis.normalize_self();
    }
    Vector ux = axis;
    Vector uy;
    if (std::abs(ux.z()) < 0.9) {
        uy = Vector(0.0, 0.0, 1.0).cross(ux);
        uy.normalize_self();
    } else {
        uy = Vector(1.0, 0.0, 0.0).cross(ux);
        uy.normalize_self();
    }
    Vector uz = ux.cross(uy);
    uz.normalize_self();
    double r = cylinder.radius;
    Vector half((L * 0.5) + inflate_amount, r + inflate_amount, r + inflate_amount);
    return BoundingBox(c, ux, uy, uz, half);
}

BoundingBox BoundingBox::from_cylinder(const Cylinder& cylinder, const Plane& plane, double inflate_amount) {
    const Line& ln = cylinder.line;
    Point p0 = ln.start();
    Point p1 = ln.end();
    Point c((p0.x() + p1.x()) * 0.5, (p0.y() + p1.y()) * 0.5, (p0.z() + p1.z()) * 0.5);
    Vector dir = ln.to_vector();
    double L = ln.length();
    if (L > 0.0) dir.normalize_self();
    double r = cylinder.radius;
    Vector Ux = plane.x_axis();
    Vector Uy = plane.y_axis();
    Vector Uz = plane.z_axis();
    auto proj_half = [&](const Vector& U) {
        double d = std::abs(dir.dot(U));
        double radial = r * std::sqrt(std::max(0.0, 1.0 - d * d));
        return (L * 0.5) * d + radial + inflate_amount;
    };
    Vector half(proj_half(Ux), proj_half(Uy), proj_half(Uz));
    return BoundingBox(c, Ux, Uy, Uz, half);
}

BoundingBox BoundingBox::aabb() const {
    double ex = half_size.x();
    double ey = half_size.y();
    double ez = half_size.z();
    double hx = std::abs(x_axis.x()) * ex + std::abs(y_axis.x()) * ey + std::abs(z_axis.x()) * ez;
    double hy = std::abs(x_axis.y()) * ex + std::abs(y_axis.y()) * ey + std::abs(z_axis.y()) * ez;
    double hz = std::abs(x_axis.z()) * ex + std::abs(y_axis.z()) * ey + std::abs(z_axis.z()) * ez;
    return BoundingBox(center, Vector(1,0,0), Vector(0,1,0), Vector(0,0,1), Vector(hx, hy, hz));
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
    // Fallback (unused by optimized path, but kept for API completeness)
    Vector rp = relative_position;
    double dot_rp = std::abs(rp.dot(axis));
    Vector v1 = box1.x_axis * box1.half_size.x();
    Vector v2 = box1.y_axis * box1.half_size.y();
    Vector v3 = box1.z_axis * box1.half_size.z();
    double proj1 = std::abs(v1.dot(axis)) + std::abs(v2.dot(axis)) + std::abs(v3.dot(axis));
    Vector v4 = box2.x_axis * box2.half_size.x();
    Vector v5 = box2.y_axis * box2.half_size.y();
    Vector v6 = box2.z_axis * box2.half_size.z();
    double proj2 = std::abs(v4.dot(axis)) + std::abs(v5.dot(axis)) + std::abs(v6.dot(axis));
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
    return collides_with_rtcd(other);
}

bool BoundingBox::collides_with_rtcd(const BoundingBox& other) const {
    const double EPS = 1e-9;
    const Vector A0 = x_axis;
    const Vector A1 = y_axis;
    const Vector A2 = z_axis;
    const Vector B0 = other.x_axis;
    const Vector B1 = other.y_axis;
    const Vector B2 = other.z_axis;
    const double a0 = half_size.x();
    const double a1 = half_size.y();
    const double a2 = half_size.z();
    const double b0 = other.half_size.x();
    const double b1 = other.half_size.y();
    const double b2 = other.half_size.z();

    double R00 = A0.dot(B0), R01 = A0.dot(B1), R02 = A0.dot(B2);
    double R10 = A1.dot(B0), R11 = A1.dot(B1), R12 = A1.dot(B2);
    double R20 = A2.dot(B0), R21 = A2.dot(B1), R22 = A2.dot(B2);

    Vector d(other.center.x() - center.x(), other.center.y() - center.y(), other.center.z() - center.z());
    double t0 = d.dot(A0);
    double t1 = d.dot(A1);
    double t2 = d.dot(A2);

    double AbsR00 = std::abs(R00) + EPS, AbsR01 = std::abs(R01) + EPS, AbsR02 = std::abs(R02) + EPS;
    double AbsR10 = std::abs(R10) + EPS, AbsR11 = std::abs(R11) + EPS, AbsR12 = std::abs(R12) + EPS;
    double AbsR20 = std::abs(R20) + EPS, AbsR21 = std::abs(R21) + EPS, AbsR22 = std::abs(R22) + EPS;

    double ra, rb, t;

    ra = a0; rb = b0 * AbsR00 + b1 * AbsR01 + b2 * AbsR02; t = std::abs(t0); if (t > ra + rb) return false;
    ra = a1; rb = b0 * AbsR10 + b1 * AbsR11 + b2 * AbsR12; t = std::abs(t1); if (t > ra + rb) return false;
    ra = a2; rb = b0 * AbsR20 + b1 * AbsR21 + b2 * AbsR22; t = std::abs(t2); if (t > ra + rb) return false;

    ra = a0 * AbsR00 + a1 * AbsR10 + a2 * AbsR20; rb = b0; t = std::abs(t0 * R00 + t1 * R10 + t2 * R20); if (t > ra + rb) return false;
    ra = a0 * AbsR01 + a1 * AbsR11 + a2 * AbsR21; rb = b1; t = std::abs(t0 * R01 + t1 * R11 + t2 * R21); if (t > ra + rb) return false;
    ra = a0 * AbsR02 + a1 * AbsR12 + a2 * AbsR22; rb = b2; t = std::abs(t0 * R02 + t1 * R12 + t2 * R22); if (t > ra + rb) return false;

    ra = a1 * AbsR20 + a2 * AbsR10; rb = b1 * AbsR02 + b2 * AbsR01; t = std::abs(t2 * R10 - t1 * R20); if (t > ra + rb) return false;
    ra = a1 * AbsR21 + a2 * AbsR11; rb = b0 * AbsR02 + b2 * AbsR00; t = std::abs(t2 * R11 - t1 * R21); if (t > ra + rb) return false;
    ra = a1 * AbsR22 + a2 * AbsR12; rb = b0 * AbsR01 + b1 * AbsR00; t = std::abs(t2 * R12 - t1 * R22); if (t > ra + rb) return false;

    ra = a0 * AbsR20 + a2 * AbsR00; rb = b1 * AbsR12 + b2 * AbsR11; t = std::abs(t0 * R20 - t2 * R00); if (t > ra + rb) return false;
    ra = a0 * AbsR21 + a2 * AbsR01; rb = b0 * AbsR12 + b2 * AbsR10; t = std::abs(t0 * R21 - t2 * R01); if (t > ra + rb) return false;
    ra = a0 * AbsR22 + a2 * AbsR02; rb = b0 * AbsR11 + b1 * AbsR10; t = std::abs(t0 * R22 - t2 * R02); if (t > ra + rb) return false;

    ra = a0 * AbsR10 + a1 * AbsR00; rb = b1 * AbsR22 + b2 * AbsR21; t = std::abs(t1 * R00 - t0 * R10); if (t > ra + rb) return false;
    ra = a0 * AbsR11 + a1 * AbsR01; rb = b0 * AbsR22 + b2 * AbsR20; t = std::abs(t1 * R01 - t0 * R11); if (t > ra + rb) return false;
    ra = a0 * AbsR12 + a1 * AbsR02; rb = b0 * AbsR21 + b1 * AbsR20; t = std::abs(t1 * R02 - t0 * R12); if (t > ra + rb) return false;

    return true;
}

bool BoundingBox::collides_with_naive(const BoundingBox& other) const {
    Vector center_vec(center.x(), center.y(), center.z());
    Vector other_center_vec(other.center.x(), other.center.y(), other.center.z());
    Vector relative_position = Vector::from_start_and_end(center_vec, other_center_vec);
    
    const Vector x1 = x_axis, y1 = y_axis, z1 = z_axis;
    const Vector x2 = other.x_axis, y2 = other.y_axis, z2 = other.z_axis;

    if (separating_plane_exists(relative_position, x1, *this, other)) return false;
    if (separating_plane_exists(relative_position, y1, *this, other)) return false;
    if (separating_plane_exists(relative_position, z1, *this, other)) return false;
    if (separating_plane_exists(relative_position, x2, *this, other)) return false;
    if (separating_plane_exists(relative_position, y2, *this, other)) return false;
    if (separating_plane_exists(relative_position, z2, *this, other)) return false;

    if (separating_plane_exists(relative_position, x1.cross(x2), *this, other)) return false;
    if (separating_plane_exists(relative_position, x1.cross(y2), *this, other)) return false;
    if (separating_plane_exists(relative_position, x1.cross(z2), *this, other)) return false;
    if (separating_plane_exists(relative_position, y1.cross(x2), *this, other)) return false;
    if (separating_plane_exists(relative_position, y1.cross(y2), *this, other)) return false;
    if (separating_plane_exists(relative_position, y1.cross(z2), *this, other)) return false;
    if (separating_plane_exists(relative_position, z1.cross(x2), *this, other)) return false;
    if (separating_plane_exists(relative_position, z1.cross(y2), *this, other)) return false;
    if (separating_plane_exists(relative_position, z1.cross(z2), *this, other)) return false;

    return true;
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
