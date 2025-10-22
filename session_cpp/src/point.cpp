#include "point.h"
#include "tolerance.h"

namespace session_cpp {

/// Convert point to string representation
std::string Point::to_string() const {
  return fmt::format("Point({}, {}, {}, {}, {}, {}, {})", _x, _y, _z, guid, name,
                     pointcolor.to_string(), width);
}

/// Equality operator
bool Point::operator==(const Point &other) const {
  return _x == other._x && _y == other._y && _z == other._z;
}

/// Inequality operator
bool Point::operator!=(const Point &other) const { return !(*this == other); }

///////////////////////////////////////////////////////////////////////////////////////////
// Transformation
///////////////////////////////////////////////////////////////////////////////////////////

void Point::transform() {
  xform.transform_point(*this);
  xform = Xform::identity();
}

Point Point::transformed() const {
  Point result = *this;
  result.transform();
  return result;
}

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

/// Convert to JSON-serializable object
nlohmann::ordered_json Point::jsondump() const {
  auto clean_float = [](double val) -> double { return std::round(val * 100.0) / 100.0; };
  return nlohmann::ordered_json{
      {"type", "Point"}, {"guid", guid},
      {"name", name},    {"x", clean_float(_x)},
      {"y", clean_float(_y)}, {"z", clean_float(_z)},
      {"width", width},  {"pointcolor", pointcolor.jsondump()}};
}

/// Create point from JSON data
Point Point::jsonload(const nlohmann::json &data) {
  Point point(data["x"], data["y"], data["z"]);
  point.guid = data["guid"];
  point.name = data["name"];
  point.pointcolor = Color::jsonload(data["pointcolor"]);
  point.width = data["width"];
  return point;
}

/// Serialize to JSON file

/// Deserialize from JSON file

///////////////////////////////////////////////////////////////////////////////////////////
// No-copy Operators
///////////////////////////////////////////////////////////////////////////////////////////

double& Point::operator[](int index) {
  if (index == 0) {
    return _x;
  } else if (index == 1) {
    return _y;
  } else if (index == 2) {
    return _z;
  } else {
    throw std::out_of_range("Index out of range");
  }
}

const double &Point::operator[](int index) const{
  if (index == 0) {
    return _x;
  } else if (index == 1) {
    return _y;
  } else if (index == 2) {
    return _z;
  } else {
    throw std::out_of_range("Index out of range");
  }
}

Point &Point::operator*=(double factor) {
  _x *= factor;
  _y *= factor;
  _z *= factor;
  return *this;
}

Point &Point::operator/=(double factor) {
  _x /= factor;
  _y /= factor;
  _z /= factor;
  return *this;
}

Point& Point::operator+=(const Vector& other) {
  _x += other.x();
  _y += other.y();
  _z += other.z();
  return *this;
}

Point& Point::operator-=(const Vector& other) {
  _x -= other.x();
  _y -= other.y();
  _z -= other.z();
  return *this;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Copy Operators

Point Point::operator*(double factor) const {
  Point result = *this;
  result *= factor;
  return result;
}

Point Point::operator/(double factor) const {
  Point result = *this;
  result /= factor;
  return result;
}

Point Point::operator+(const Vector& other) const {
  Point result(_x + other.x(), _y + other.y(), _z + other.z());
  return result;
}

Point Point::operator-(const Vector& other) const {
  Point result(_x - other.x(), _y - other.y(), _z - other.z());
  return result;
}

Vector Point::operator-(const Point& other) const {
  Vector result(_x - other._x, _y - other._y, _z - other._z);
  return result;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Details
///////////////////////////////////////////////////////////////////////////////////////////

bool Point::ccw(const Point& a, const Point& b, const Point& c) {
    return (c._y - a._y) * (b._x - a._x) > (b._y - a._y) * (c._x - a._x);
}

Point Point::mid_point(const Point& p) const {
    return Point((_x + p._x) / 2, (_y + p._y) / 2, (_z + p._z) / 2);
}

double Point::distance(const Point& p, double float_min) const {
    double dx = std::abs((*this)[0] - p[0]);
    double dy = std::abs((*this)[1] - p[1]);
    double dz = std::abs((*this)[2] - p[2]);
    double length = 0.0;

    // Reorder coordinates to put largest in dx
    if (dy >= dx && dy >= dz) {
        std::swap(dx, dy);
    } else if (dz >= dx && dz >= dy) {
        std::swap(dx, dz);
    }

    if (dx > float_min) {
        dy /= dx;
        dz /= dx;
        length = dx * std::sqrt(1.0 + dy * dy + dz * dz);
    } else if (dx > 0.0 && std::isfinite(dx)) {
        length = dx;
    } else {
        length = 0.0;
    }

    return length;
}

double Point::area(const std::vector<Point>& points) {
    size_t n = points.size();
    double area = 0.0;
    
    for (size_t i = 0; i < n; ++i) {
        size_t j = (i + 1) % n;
        area += points[i][0] * points[j][1];
        area -= points[j][0] * points[i][1];
    }
    return std::abs(area) / 2.0;
}

Point Point::centroid_quad(const std::vector<Point>& vertices) {
    if (vertices.size() != 4) {
        throw std::invalid_argument("Polygon must have exactly 4 vertices.");
    }
    
    double total_area = 0.0;
    Vector centroid_sum(0, 0, 0);
    
    for (int i = 0; i < 4; ++i) {
        const Point& p0 = vertices[i];
        const Point& p1 = vertices[(i + 1) % 4];
        const Point& p2 = vertices[(i + 2) % 4];
        
        double tri_area = std::abs(p0[0] * (p1[1] - p2[1]) + 
                                  p1[0] * (p2[1] - p0[1]) + 
                                  p2[0] * (p0[1] - p1[1])) / 2.0;
        total_area += tri_area;
        
        Vector tri_centroid((p0[0] + p1[0] + p2[0]) / 3.0,
                          (p0[1] + p1[1] + p2[1]) / 3.0,
                          (p0[2] + p1[2] + p2[2]) / 3.0);
        centroid_sum += tri_centroid * tri_area;
    }
    
    Vector result = centroid_sum / total_area;
    return Point(result.x(), result.y(), result.z());
}

///////////////////////////////////////////////////////////////////////////////////////////
// Not class methods
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream &operator<<(std::ostream &os, const Point &point) {
  return os << fmt::format("Point(x={}, y={}, z={})", 
                           TOL.format_number(point.x()), 
                           TOL.format_number(point.y()), 
                           TOL.format_number(point.z()));
}
} // namespace session_cpp