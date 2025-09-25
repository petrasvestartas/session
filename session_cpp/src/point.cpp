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

// ###########################################################################################
// # Details
// ###########################################################################################

// @staticmethod
// def ccw(a, b, c):
//     """Check if the points are in counter-clockwise order.

//     Parameters
//     ----------
//     a : :class:`Point`
//         First point.
//     b : :class:`Point`
//         Second point.
//     c : :class:`Point`
//         Third point.

//     Returns
//     -------
//     bool
//         True if the points are in counter-clockwise order, False otherwise.
    
//     """

//     return (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)

// def mid_point(self, p):
//     """Calculate the mid point between this point and another point.

//     Parameters
//     ----------
//     p : :class:`Point`
//         The other point.

//     Returns
//     -------
//     :class:`Point`
//         The mid point between this point and the other point.

//     """

//     return Point((self.x + p.x) / 2, (self.y + p.y) / 2, (self.z + p.z) / 2)

// def distance(self, p, double_min=1e-12):
//     """Calculate the distance between this point and another point.

//     Parameters
//     ----------
//     p : :class:`Point`
//         The other point.
//     double_min : float, optional
//         The minimum value for the distance. Defaults to 1e-12.

//     Returns
//     -------
//     float
//         The distance between this point and the other point.

//     """

//     x = abs(self[0] - p[0])
//     y = abs(self[1] - p[1])
//     z = abs(self[2] - p[2])
//     length = 0.0

//     if y >= x and y >= z:
//         length, x, y = x, y, x
//     elif z >= x and z >= y:
//         length, x, z = x, z, x

//     if x > double_min:
//         y /= x
//         z /= x
//         length = x * math.sqrt(1.0 + y * y + z * z)
//     elif x > 0.0 and math.isfinite(x):
//         length = x
//     else:
//         length = 0.0

//     return length

// @staticmethod
// def area(points):
//     """Calculate the area of a polygon.

//     Parameters
//     ----------
//     points : list of :class:`Point`
//         The points of the polygon.

//     Returns
//     -------
//     float
//         The area of the polygon.
    
//     """

//     n = len(points)
//     area = 0.0
//     for i in range(n):
//         j = (i + 1) % n
//         area += points[i][0] * points[j][1]
//         area -= points[j][0] * points[i][1]

//     return abs(area) / 2.0

// @staticmethod
// def centroid_quad(vertices):
//     """Calculate the centroid of a quadrilateral.

//     Parameters
//     ----------
//     vertices : list of :class:`Point`
//         The vertices of the quadrilateral.

//     Returns
//     -------
//     :class:`Point`
//         The centroid of the quadrilateral.
    
//     """

//     if len(vertices) != 4:
//         raise ValueError("Polygon must have exactly 4 vertices.")
    
//     total_area = 0.0
//     centroid_sum = Point(0, 0, 0)

//     for i in range(4):
//         p0, p1, p2 = vertices[i], vertices[(i+1)%4], vertices[(i+2)%4]
//         tri_area = abs(p0[0]*(p1[1]-p2[1]) + p1[0]*(p2[1]-p0[1]) + p2[0]*(p0[1]-p1[1])) / 2.0
//         total_area += tri_area
//         tri_centroid = Point((p0[0]+p1[0]+p2[0])/3.0,
//                              (p0[1]+p1[1]+p2[1])/3.0,
//                              (p0[2]+p1[2]+p2[2])/3.0)
//         centroid_sum += tri_centroid * tri_area

//     return centroid_sum / total_area
    

///////////////////////////////////////////////////////////////////////////////////////////
// Not class methods
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream &operator<<(std::ostream &os, const Point &point) {
  return os << point.to_string();
}
} // namespace session_cpp