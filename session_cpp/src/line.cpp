#include "line.h"
#include <cmath>
#include <stdexcept>

namespace session_cpp {

Line::Line() : _x0(0.0), _y0(0.0), _z0(0.0), _x1(0.0), _y1(0.0), _z1(1.0) {}

Line::Line(double x0, double y0, double z0, double x1, double y1, double z1) 
    : _x0(x0), _y0(y0), _z0(z0), _x1(x1), _y1(y1), _z1(z1) {}

Line Line::from_points(const Point& p1, const Point& p2) {
    return Line(p1.x(), p1.y(), p1.z(), p2.x(), p2.y(), p2.z());
}

Line Line::with_name(const std::string& name, double x0, double y0, double z0, double x1, double y1, double z1) {
    Line line(x0, y0, z0, x1, y1, z1);
    line.name = name;
    return line;
}

std::string Line::to_string() const {
    return fmt::format("Line({}, {}, {}, {}, {}, {})", _x0, _y0, _z0, _x1, _y1, _z1);
}

///////////////////////////////////////////////////////////////////////////////////////////
// Transformation
///////////////////////////////////////////////////////////////////////////////////////////

void Line::transform() {
  Point start(_x0, _y0, _z0);
  Point end(_x1, _y1, _z1);
  
  xform.transform_point(start);
  xform.transform_point(end);
  
  _x0 = start.x();
  _y0 = start.y();
  _z0 = start.z();
  _x1 = end.x();
  _y1 = end.y();
  _z1 = end.z();
  xform = Xform::identity();
}

Line Line::transformed() const {
  Line result = *this;
  result.transform();
  return result;
}

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

nlohmann::ordered_json Line::jsondump() const {
    return nlohmann::ordered_json{
        {"type", "Line"},
        {"guid", guid},
        {"name", name},
        {"x0", _x0},
        {"y0", _y0},
        {"z0", _z0},
        {"x1", _x1},
        {"y1", _y1},
        {"z1", _z1},
        {"width", width},
        {"linecolor", linecolor.jsondump()}};
}

Line Line::jsonload(const nlohmann::json& data) {
    Line line(data["x0"], data["y0"], data["z0"], data["x1"], data["y1"], data["z1"]);
    line.guid = data["guid"];
    line.name = data["name"];
    line.linecolor = Color::jsonload(data["linecolor"]);
    line.width = data["width"];
    return line;
}



///////////////////////////////////////////////////////////////////////////////////////////
// Operators
///////////////////////////////////////////////////////////////////////////////////////////

double& Line::operator[](int index) {
    switch(index) {
        case 0: return _x0;
        case 1: return _y0;
        case 2: return _z0;
        case 3: return _x1;
        case 4: return _y1;
        case 5: return _z1;
        default: throw std::out_of_range("Index out of bounds");
    }
}

const double& Line::operator[](int index) const {
    switch(index) {
        case 0: return _x0;
        case 1: return _y0;
        case 2: return _z0;
        case 3: return _x1;
        case 4: return _y1;
        case 5: return _z1;
        default: throw std::out_of_range("Index out of bounds");
    }
}

Line& Line::operator+=(const Vector& other) {
    _x0 += other.x();
    _y0 += other.y();
    _z0 += other.z();
    _x1 += other.x();
    _y1 += other.y();
    _z1 += other.z();
    return *this;
}

Line& Line::operator-=(const Vector& other) {
    _x0 -= other.x();
    _y0 -= other.y();
    _z0 -= other.z();
    _x1 -= other.x();
    _y1 -= other.y();
    _z1 -= other.z();
    return *this;
}

Line& Line::operator*=(double factor) {
    _x0 *= factor;
    _y0 *= factor;
    _z0 *= factor;
    _x1 *= factor;
    _y1 *= factor;
    _z1 *= factor;
    return *this;
}

Line& Line::operator/=(double factor) {
    _x0 /= factor;
    _y0 /= factor;
    _z0 /= factor;
    _x1 /= factor;
    _y1 /= factor;
    _z1 /= factor;
    return *this;
}

Line Line::operator+(const Vector& other) const {
    Line result = *this;
    result += other;
    return result;
}

Line Line::operator-(const Vector& other) const {
    Line result = *this;
    result -= other;
    return result;
}

Line Line::operator*(double factor) const {
    Line result = *this;
    result *= factor;
    return result;
}

Line Line::operator/(double factor) const {
    Line result = *this;
    result /= factor;
    return result;
}

Vector Line::to_vector() const {
    return Vector(_x1 - _x0, _y1 - _y0, _z1 - _z0);
}

///////////////////////////////////////////////////////////////////////////////////////////
// Geometry Methods
///////////////////////////////////////////////////////////////////////////////////////////

double Line::length() const {
    double dx = _x1 - _x0;
    double dy = _y1 - _y0;
    double dz = _z1 - _z0;
    return std::sqrt(dx * dx + dy * dy + dz * dz);
}

double Line::squared_length() const {
    double dx = _x1 - _x0;
    double dy = _y1 - _y0;
    double dz = _z1 - _z0;
    return dx * dx + dy * dy + dz * dz;
}

Point Line::point_at(double t) const {
    double s = 1.0 - t;
    return Point(s * _x0 + t * _x1, s * _y0 + t * _y1, s * _z0 + t * _z1);
}

Point Line::start() const {
    return Point(_x0, _y0, _z0);
}

Point Line::end() const {
    return Point(_x1, _y1, _z1);
}

std::ostream& operator<<(std::ostream& os, const Line& line) {
    return os << line.to_string();
}

}  // namespace session_cpp