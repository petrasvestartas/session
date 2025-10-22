#include "plane.h"
#include "tolerance.h"
#include <algorithm>

namespace session_cpp {

///////////////////////////////////////////////////////////////////////////////////////////
// Constructors
///////////////////////////////////////////////////////////////////////////////////////////

Plane::Plane() {
    xform = Xform::identity();
    _origin = Point(0.0, 0.0, 0.0);
    _x_axis = Vector::x_axis();
    _y_axis = Vector::y_axis();
    _z_axis = Vector::z_axis();
    _a = 0.0;
    _b = 0.0;
    _c = 1.0;
    _d = 0.0;
}

Plane::Plane(Point& point, Vector& x_axis, Vector& y_axis, std::string name) {
    xform = Xform::identity();
    this->name = name;
    _origin = point;
    _x_axis = x_axis;
    _x_axis.normalize_self();
    _y_axis = y_axis - x_axis * (y_axis.dot(_x_axis));
    _y_axis.normalize_self();
    _z_axis = _x_axis.cross(_y_axis);
    _z_axis.normalize_self();
    
    _a = _z_axis.x();
    _b = _z_axis.y();
    _c = _z_axis.z();
    _d = -(_a * _origin.x() + _b * _origin.y() + _c * _origin.z());
}

Plane Plane::from_point_normal(Point& point, Vector& normal) {
    Plane plane;
    plane._origin = point;
    plane._z_axis = normal;
    plane._z_axis.normalize_self();
    plane._x_axis.perpendicular_to(plane._z_axis);
    plane._x_axis.normalize_self();
    plane._y_axis = plane._z_axis.cross(plane._x_axis);
    plane._y_axis.normalize_self();
    
    plane._a = plane._z_axis.x();
    plane._b = plane._z_axis.y();
    plane._c = plane._z_axis.z();
    plane._d = -(plane._a * plane._origin.x() + plane._b * plane._origin.y() + plane._c * plane._origin.z());
    return plane;
}

Plane Plane::from_points(std::vector<Point>& points) {
    if (points.size() < 3) {
        return Plane();
    }
    
    Plane plane;
    plane._origin = points[0];
    
    Vector v1 = points[1] - points[0];
    Vector v2 = points[2] - points[0];
    plane._z_axis = v1.cross(v2);
    plane._z_axis.normalize_self();
    
    plane._x_axis = v1;
    plane._x_axis.normalize_self();
    plane._y_axis = plane._z_axis.cross(plane._x_axis);
    plane._y_axis.normalize_self();
    
    plane._a = plane._z_axis.x();
    plane._b = plane._z_axis.y();
    plane._c = plane._z_axis.z();
    plane._d = -(plane._a * plane._origin.x() + plane._b * plane._origin.y() + plane._c * plane._origin.z());
    return plane;
}

Plane Plane::from_two_points(Point& point1, Point& point2) {
    Plane plane;
    plane._origin = point1;
    
    Vector direction = point2 - point1;
    direction.normalize_self();
    plane._z_axis.perpendicular_to(direction);
    plane._z_axis.normalize_self();
    
    plane._x_axis = direction;
    
    plane._y_axis = plane._z_axis.cross(plane._x_axis);
    plane._y_axis.normalize_self();
    
    plane._a = plane._z_axis.x();
    plane._b = plane._z_axis.y();
    plane._c = plane._z_axis.z();
    plane._d = -(plane._a * plane._origin.x() + plane._b * plane._origin.y() + plane._c * plane._origin.z());
    return plane;
}

Plane Plane::xy_plane() {
    Plane plane;
    plane.name = "xy_plane";
    plane._origin = Point(0.0, 0.0, 0.0);
    plane._x_axis = Vector::x_axis();
    plane._y_axis = Vector::y_axis();
    plane._z_axis = Vector::z_axis();
    plane._a = 0.0;
    plane._b = 0.0;
    plane._c = 1.0;
    plane._d = 0.0;
    return plane;
}

Plane Plane::yz_plane() {
    Plane plane;
    plane.name = "yz_plane";
    plane._origin = Point(0.0, 0.0, 0.0);
    plane._x_axis = Vector::y_axis();
    plane._y_axis = Vector::z_axis();
    plane._z_axis = Vector::x_axis();
    plane._a = 1.0;
    plane._b = 0.0;
    plane._c = 0.0;
    plane._d = 0.0;
    return plane;
}

Plane Plane::xz_plane() {
    Plane plane;
    plane.name = "xz_plane";
    plane._origin = Point(0.0, 0.0, 0.0);
    plane._x_axis = Vector::x_axis();
    plane._y_axis = Vector(0.0, 0.0, -1.0);
    plane._z_axis = Vector(0.0, 1.0, 0.0);
    plane._a = 0.0;
    plane._b = 1.0;
    plane._c = 0.0;
    plane._d = 0.0;
    return plane;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Operators
///////////////////////////////////////////////////////////////////////////////////////////

std::string Plane::to_string() const {
    return fmt::format("Plane(origin={}, x_axis={}, y_axis={}, z_axis={}, guid={}, name={})",
                       _origin.to_string(), _x_axis.to_string(), _y_axis.to_string(),
                       _z_axis.to_string(), guid, name);
}

///////////////////////////////////////////////////////////////////////////////////////////
// Transformation
///////////////////////////////////////////////////////////////////////////////////////////

void Plane::transform() {
  xform.transform_point(_origin);
  xform.transform_vector(_x_axis);
  xform.transform_vector(_y_axis);
  xform.transform_vector(_z_axis);
  xform = Xform::identity();
}

Plane Plane::transformed() const {
  Plane result = *this;
  result.transform();
  return result;
}

bool Plane::operator==(const Point &other) const {
    return _origin == other;
}

bool Plane::operator!=(const Point &other) const {
    return !(*this == other);
}

///////////////////////////////////////////////////////////////////////////////////////////
// No-copy Operators
///////////////////////////////////////////////////////////////////////////////////////////

Vector &Plane::operator[](int index) {
    if (index == 0)
        return _x_axis;
    if (index == 1)
        return _y_axis;
    return _z_axis;
}

const Vector &Plane::operator[](int index) const {
    if (index == 0)
        return _x_axis;
    if (index == 1)
        return _y_axis;
    return _z_axis;
}

Plane &Plane::operator+=(const Vector &other) {
    _origin += other;
    _d = -(_a * _origin.x() + _b * _origin.y() + _c * _origin.z());
    return *this;
}

Plane &Plane::operator-=(const Vector &other) {
    _origin -= other;
    _d = -(_a * _origin.x() + _b * _origin.y() + _c * _origin.z());
    return *this;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Copy Operators
///////////////////////////////////////////////////////////////////////////////////////////

Plane Plane::operator+(const Vector &other) const {
    Plane result = *this;
    result += other;
    return result;
}

Plane Plane::operator-(const Vector &other) const {
    Plane result = *this;
    result -= other;
    return result;
}

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

nlohmann::ordered_json Plane::jsondump() const {
    auto clean_float = [](double val) -> double { 
        return static_cast<double>(std::round(val * 100.0) / 100.0); 
    };
    return nlohmann::ordered_json{
        {"type", "Plane"},
        {"guid", guid},
        {"name", name},
        {"origin", _origin.jsondump()},
        {"x_axis", _x_axis.jsondump()},
        {"y_axis", _y_axis.jsondump()},
        {"z_axis", _z_axis.jsondump()},
        {"a", clean_float(_a)},
        {"b", clean_float(_b)},
        {"c", clean_float(_c)},
        {"d", clean_float(_d)}
    };
}

Plane Plane::jsonload(const nlohmann::json &data) {
    Plane plane;
    plane._origin = Point::jsonload(data["origin"]);
    plane._x_axis = Vector::jsonload(data["x_axis"]);
    plane._y_axis = Vector::jsonload(data["y_axis"]);
    plane._z_axis = Vector::jsonload(data["z_axis"]);
    plane.guid = data["guid"];
    plane.name = data["name"];
    
    plane._a = plane._z_axis.x();
    plane._b = plane._z_axis.y();
    plane._c = plane._z_axis.z();
    plane._d = -(plane._a * plane._origin.x() + plane._b * plane._origin.y() + plane._c * plane._origin.z());
    return plane;
}



///////////////////////////////////////////////////////////////////////////////////////////
// Details
///////////////////////////////////////////////////////////////////////////////////////////

void Plane::reverse() {
    Vector temp = _x_axis;
    _x_axis = _y_axis;
    _y_axis = temp;
    _z_axis.reverse();
    
    _a = _z_axis.x();
    _b = _z_axis.y();
    _c = _z_axis.z();
    _d = -(_a * _origin.x() + _b * _origin.y() + _c * _origin.z());
}

void Plane::rotate(double angles_in_radians) {
    double cos_angle = std::cos(angles_in_radians);
    double sin_angle = std::sin(angles_in_radians);
    
    Vector new_x = _x_axis * cos_angle + _y_axis * sin_angle;
    Vector new_y = _y_axis * cos_angle - _x_axis * sin_angle;
    
    _x_axis = new_x;
    _y_axis = new_y;
    
    _a = _z_axis.x();
    _b = _z_axis.y();
    _c = _z_axis.z();
    _d = -(_a * _origin.x() + _b * _origin.y() + _c * _origin.z());
}

bool Plane::is_right_hand() const {
    Vector x_copy = _x_axis;
    Vector y_copy = _y_axis;
    Vector z_copy = _z_axis;
    Vector cross = x_copy.cross(y_copy);
    double dot_product = cross.dot(z_copy);
    return dot_product > 0.999;
}

bool Plane::is_same_direction(const Plane &plane0, const Plane &plane1, bool can_be_flipped) {
    Vector n0 = plane0._z_axis;
    Vector n1 = plane1._z_axis;
    
    int parallel = n0.is_parallel_to(n1);
    
    if (can_be_flipped) {
        return parallel != 0;
    } else {
        return parallel == 1;
    }
}

bool Plane::is_same_position(const Plane &plane0, const Plane &plane1) {
    double dist0 = std::abs(plane0._a * plane1._origin.x() + 
                           plane0._b * plane1._origin.y() + 
                           plane0._c * plane1._origin.z() + 
                           plane0._d);
    
    double dist1 = std::abs(plane1._a * plane0._origin.x() + 
                           plane1._b * plane0._origin.y() + 
                           plane1._c * plane0._origin.z() + 
                           plane1._d);
    
    double tolerance = static_cast<double>(session_cpp::Tolerance::ZERO_TOLERANCE);
    return dist0 < tolerance && dist1 < tolerance;
}

bool Plane::is_coplanar(const Plane &plane0, const Plane plane1, bool can_be_flipped) {
    return is_same_direction(plane0, plane1, can_be_flipped) && 
           is_same_position(plane0, plane1);
}

Plane Plane::translate_by_normal(double distance) const {
    // Get normalized normal vector (z_axis)
    Vector normal = _z_axis;
    normal.normalize_self();
    
    // Move origin along the normal
    Point new_origin = _origin + (normal * distance);
    
    // Create new plane with same orientation but new origin
    Vector x_copy = _x_axis;
    Vector y_copy = _y_axis;
    return Plane(new_origin, x_copy, y_copy, name);
}

///////////////////////////////////////////////////////////////////////////////////////////
// Stream operator
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream &operator<<(std::ostream &os, const Plane &plane) {
    os << plane.to_string();
    return os;
}

} // namespace session_cpp
