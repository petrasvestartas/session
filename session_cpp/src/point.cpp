#include "point.h"

namespace session_cpp {

/// Convert point to string representation
std::string Point::to_string() const {
  return fmt::format("Point({}, {}, {}, {}, {}, {}, {})", x, y, z, guid, name,
                     pointcolor.to_string(), width);
}

/// Equality operator
bool Point::operator==(const Point &other) const {
  return x == other.x && y == other.y && z == other.z;
}

/// Inequality operator
bool Point::operator!=(const Point &other) const { return !(*this == other); }

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

/// Convert to JSON-serializable object
nlohmann::ordered_json Point::to_json_data() const {
  return nlohmann::ordered_json{
      {"type", "Point"}, {"guid", guid},
      {"name", name},    {"x", x},
      {"y", y},          {"z", z},
      {"width", width},  {"pointcolor", pointcolor.to_json_data()}};
}

/// Create point from JSON data
Point Point::from_json_data(const nlohmann::json &data) {
  Point point(data["x"], data["y"], data["z"]);
  point.guid = data["guid"];
  point.name = data["name"];
  point.pointcolor = Color::from_json_data(data["pointcolor"]);
  point.width = data["width"];
  return point;
}

/// Serialize to JSON file
void Point::to_json(const std::string &filepath) const {
  std::ofstream file(filepath);
  file << to_json_data().dump(4);
}

/// Deserialize from JSON file
Point Point::from_json(const std::string &filepath) {
  std::ifstream file(filepath);
  nlohmann::json data;
  file >> data;
  return from_json_data(data);
}

///////////////////////////////////////////////////////////////////////////////////////////
// No-copy Operators
///////////////////////////////////////////////////////////////////////////////////////////

double& Point::operator[](int index) {
  if (index == 0) {
    return x;
  } else if (index == 1) {
    return y;
  } else if (index == 2) {
    return z;
  } else {
    throw std::out_of_range("Index out of range");
  }
}

const double &Point::operator[](int index) const{
  if (index == 0) {
    return x;
  } else if (index == 1) {
    return y;
  } else if (index == 2) {
    return z;
  } else {
    throw std::out_of_range("Index out of range");
  }
}

Point &Point::operator*=(double factor) {
  x *= factor;
  y *= factor;
  z *= factor;
  return *this;
}

Point &Point::operator/=(double factor) {
  x /= factor;
  y /= factor;
  z /= factor;
  return *this;
}

Point &Point::operator+=(const Point &other) {
  x += other.x;
  y += other.y;
  z += other.z;
  return *this;
}

Point &Point::operator-=(const Point &other) {
  x -= other.x;
  y -= other.y;
  z -= other.z;
  return *this;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Copy Operators
///////////////////////////////////////////////////////////////////////////////////////////

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
  Point result(x + other.x, y + other.y, z + other.z);
  return result;
}

Point Point::operator-(const Vector& other) const {
  Point result(x - other.x, y - other.y, z - other.z);
  return result;
}

Point Point::operator+(const Point& other) const {
  Point result(x + other.x, y + other.y, z + other.z);
  return result;
}

Point Point::operator-(const Point& other) const {
  Point result(x - other.x, y - other.y, z - other.z);
  return result;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Details
///////////////////////////////////////////////////////////////////////////////////////////

bool Point::ccw(const Point& a, const Point& b, const Point& c) {
    return (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x);
}

Point Point::mid_point(const Point& p) const {
    return Point((x + p.x) / 2, (y + p.y) / 2, (z + p.z) / 2);
}

double Point::distance(const Point& p, double double_min) const {
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

    if (dx > double_min) {
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
    Point centroid_sum(0, 0, 0);
    
    for (int i = 0; i < 4; ++i) {
        const Point& p0 = vertices[i];
        const Point& p1 = vertices[(i + 1) % 4];
        const Point& p2 = vertices[(i + 2) % 4];
        
        double tri_area = std::abs(p0[0] * (p1[1] - p2[1]) + 
                                  p1[0] * (p2[1] - p0[1]) + 
                                  p2[0] * (p0[1] - p1[1])) / 2.0;
        total_area += tri_area;
        
        Point tri_centroid((p0[0] + p1[0] + p2[0]) / 3.0,
                          (p0[1] + p1[1] + p2[1]) / 3.0,
                          (p0[2] + p1[2] + p2[2]) / 3.0);
        centroid_sum += tri_centroid * tri_area;
    }
    
    return centroid_sum / total_area;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Not class methods
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream &operator<<(std::ostream &os, const Point &point) {
  return os << point.to_string();
}
} // namespace session_cpp