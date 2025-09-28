#include "vector.h"
#include <algorithm>

namespace session_cpp {

///////////////////////////////////////////////////////////////////////////////////////////
// Operators
///////////////////////////////////////////////////////////////////////////////////////////

/// Convert vector to string representation
std::string Vector::to_string() const {
  return fmt::format("Vector({}, {}, {}, {}, {})", x, y, z, guid, name);
}

/// Equality operator
bool Vector::operator==(const Vector &other) const {
  return x == other.x && y == other.y && z == other.z;
}

/// Inequality operator
bool Vector::operator!=(const Vector &other) const { return !(*this == other); }

/////////////////////////////////////////////////////////////////////////////////////////////
// No-copy operators
///////////////////////////////////////////////////////////////////////////////////////////

double &Vector::operator[](int index) {
  if (index == 0)
    return x;
  if (index == 1)
    return y;
  return z; // assume index == 2
}

const double &Vector::operator[](int index) const {
  if (index == 0)
    return x;
  if (index == 1)
    return y;
  return z; // assume index == 2
}

Vector &Vector::operator*=(double factor) {
  x *= factor;
  y *= factor;
  z *= factor;
  return *this;
}

Vector &Vector::operator/=(double factor) {
  x /= factor;
  y /= factor;
  z /= factor;
  return *this;
}

Vector &Vector::operator+=(const Vector &other) {
  x += other.x;
  y += other.y;
  z += other.z;
  return *this;
}

Vector &Vector::operator-=(const Vector &other) {
  x -= other.x;
  y -= other.y;
  z -= other.z;
  return *this;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Copy operators
///////////////////////////////////////////////////////////////////////////////////////////

Vector Vector::operator*(double factor) const { return Vector(x * factor, y * factor, z * factor); }

Vector Vector::operator/(double factor) const { return Vector(x / factor, y / factor, z / factor); }

Vector Vector::operator+(const Vector &other) const {
  return Vector(x + other.x, y + other.y, z + other.z);
}

Vector Vector::operator-(const Vector &other) const {
  return Vector(x - other.x, y - other.y, z - other.z);
}

Vector operator*(double factor, const Vector &v) { return v * factor; }

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

/// Convert to JSON-serializable object
nlohmann::ordered_json Vector::to_json_data() const {
  return nlohmann::ordered_json{{"type", "Vector"}, {"guid", guid},
                                {"name", name},     {"x", x},
                                {"y", y},           {"z", z}};
}

/// Create vector from JSON data
Vector Vector::from_json_data(const nlohmann::json &data) {
  Vector vector(data["x"], data["y"], data["z"]);
  vector.guid = data["guid"];
  vector.name = data["name"];
  return vector;
}

/// Serialize to JSON file
void Vector::to_json(const std::string &filepath) const {
  std::ofstream file(filepath);
  file << to_json_data().dump(4);
}

/// Deserialize from JSON file
Vector Vector::from_json(const std::string &filepath) {
  std::ifstream file(filepath);
  nlohmann::json data;
  file >> data;
  return from_json_data(data);
}

///////////////////////////////////////////////////////////////////////////////////////////
// Static methods
///////////////////////////////////////////////////////////////////////////////////////////

Vector Vector::XAxis() { return Vector(1.0, 0.0, 0.0); }
Vector Vector::YAxis() { return Vector(0.0, 1.0, 0.0); }
Vector Vector::ZAxis() { return Vector(0.0, 0.0, 1.0); }

Vector Vector::from_start_and_end(const Vector &start, const Vector &end) {
  return Vector(end.x - start.x, end.y - start.y, end.z - start.z);
}

///////////////////////////////////////////////////////////////////////////////////////////
// Details / Geometry
///////////////////////////////////////////////////////////////////////////////////////////

void Vector::reverse() {
  x = -x;
  y = -y;
  z = -z;
}

double Vector::compute_length() const {
  double len = 0.0;

  double ax = std::abs(x);
  double ay = std::abs(y);
  double az = std::abs(z);

  const bool x_zero = ax < geo::GLOBALS::ZERO_TOLERANCE;
  const bool y_zero = ay < geo::GLOBALS::ZERO_TOLERANCE;
  const bool z_zero = az < geo::GLOBALS::ZERO_TOLERANCE;

  if (x_zero && y_zero && z_zero)
    return 0.0;
  else if (x_zero && y_zero)
    return az;
  else if (x_zero && z_zero)
    return ay;
  else if (y_zero && z_zero)
    return ax;

  // Ensure ax is the largest
  if (ay >= ax && ay >= az) {
    std::swap(ax, ay);
  } else if (az >= ax && az >= ay) {
    std::swap(ax, az);
  }

  if (ax > geo::GLOBALS::DOUBLE_MIN) {
    ay /= ax;
    az /= ax;
    len = ax * std::sqrt(1.0 + ay * ay + az * az);
  } else if (ax > 0.0 && geo::GLOBALS::IS_FINITE(ax)) {
    len = ax;
  } else {
    len = 0.0;
  }

  return len;
}

double Vector::length(double predefined_length) {
  if (predefined_length != 0.0) {
    // Rescale current direction to predefined_length
    double d = compute_length();
    if (d > 0.0) {
      double s = predefined_length / d;
      x *= s;
      y *= s;
      z *= s;
    }
  }
  return compute_length();
}

bool Vector::unitize() {
  double d = compute_length();
  if (d > 0.0) {
    x /= d;
    y /= d;
    z /= d;
    return true;
  }
  return false;
}

Vector Vector::unitized() {
  Vector u(x, y, z);
  u.unitize();
  return u;
}

Vector Vector::projection(
    Vector &projection_vector, double tolerance,
    double *out_projected_vector_length,
    Vector *out_perpendicular_projected_vector,
    double *out_perpendicular_projected_vector_length) {
  double projection_vector_length = projection_vector.length();

  if (projection_vector_length < tolerance) {
    if (out_projected_vector_length)
      *out_projected_vector_length = 0.0;
    if (out_perpendicular_projected_vector)
      *out_perpendicular_projected_vector = Vector(0, 0, 0);
    if (out_perpendicular_projected_vector_length)
      *out_perpendicular_projected_vector_length = 0.0;
    return Vector(0, 0, 0);
  }

  Vector projection_vector_unit(
      projection_vector.x / projection_vector_length,
      projection_vector.y / projection_vector_length,
      projection_vector.z / projection_vector_length);

  double projected_vector_length = this->dot(projection_vector_unit);
  if (out_projected_vector_length)
    *out_projected_vector_length = projected_vector_length;

  Vector out_projection_vector = projection_vector_unit * projected_vector_length;

  if (out_perpendicular_projected_vector) {
    *out_perpendicular_projected_vector = *this - out_projection_vector;
    if (out_perpendicular_projected_vector_length) {
      *out_perpendicular_projected_vector_length =
          out_perpendicular_projected_vector->length();
    }
  } else if (out_perpendicular_projected_vector_length) {
    Vector temp = *this - out_projection_vector;
    *out_perpendicular_projected_vector_length = temp.length();
  }

  return out_projection_vector;
}

int Vector::is_parallel_to(Vector &v) {
  double ll = length() * v.length();
  int result;
  
  if (ll > 0.0) {
    const double cos_angle = ((*this)[0] * v[0] + (*this)[1] * v[1] + (*this)[2] * v[2]) / ll;
    
    const double angle_in_radians = geo::GLOBALS::ANGLE * (geo::GLOBALS::PI / 180.0);
    const double cos_tol = std::cos(angle_in_radians);
    if (cos_angle >= cos_tol)
      result = 1;  // Parallel
    else if (cos_angle <= -cos_tol)
      result = -1; // Antiparallel
    else
      result = 0;  // Not parallel
  } else {
    result = 0;  // Not parallel
  }
  
  return result;
}

double Vector::dot(Vector &other) {
  double result = 0.0;
  for (size_t i = 0; i < 3; ++i) {
    result += (*this)[i] * other[i];
  }
  return result;
}

Vector Vector::cross(Vector &other) {
  double cx = (*this)[1] * other[2] - (*this)[2] * other[1];
  double cy = (*this)[2] * other[0] - (*this)[0] * other[2];
  double cz = (*this)[0] * other[1] - (*this)[1] * other[0];
  Vector result(cx, cy, cz);
  result.unitize();
  return result;
}

double Vector::angle(Vector &other, bool sign_by_cross_product, bool degrees,
                     double tolerance) {
  double dotp = this->dot(other);
  double len0 = this->length();
  double len1 = other.length();
  double denom = len0 * len1;
  if (denom < tolerance) {
    return 0.0;
  }
  double cos_angle = dotp / denom;
  cos_angle = std::max(-1.0, std::min(1.0, cos_angle));
  double ang = std::acos(cos_angle);
  if (sign_by_cross_product) {
    Vector cp = this->cross(other);
    if (cp.z < 0)
      ang = -ang;
  }
  double to_degrees = degrees ? geo::GLOBALS::TO_DEGREES : 1.0;
  return ang * to_degrees;
}

Vector Vector::get_leveled_vector(double &vertical_height) {
  Vector copy(x, y, z);
  if (copy.unitize()) {
    Vector reference(0, 0, 1);
    double angle = copy.angle(reference, true); // returns degrees
    // CRITICAL: statics bug - passes degrees directly to cos (expects radians)
    double inclined_offset_by_vertical_distance = vertical_height / std::cos(angle);
    copy *= inclined_offset_by_vertical_distance;
  }
  return copy;
}

double Vector::cosine_law(double &a, double &b, double &ang_between, bool degrees) {
  double to_rad = degrees ? geo::GLOBALS::TO_RADIANS : 1.0;
  return std::sqrt(a * a + b * b - 2 * a * b * std::cos(ang_between * to_rad));
}

double Vector::sine_law_angle(double &a, double &A, double &b, bool degrees) {
  double to_rad = degrees ? geo::GLOBALS::TO_RADIANS : 1.0;
  double to_deg = degrees ? geo::GLOBALS::TO_DEGREES : 1.0;
  return std::asin((b * std::sin(A * to_rad)) / a) * to_deg;
}

double Vector::sine_law_length(double &a, double &A, double &B, bool degrees) {
  double to_rad = degrees ? geo::GLOBALS::TO_RADIANS : 1.0;
  return (a * std::sin(B * to_rad)) / std::sin(A * to_rad);
}

double Vector::angle_between_vector_xy_components_degrees(Vector &vector, bool degrees) {
  double to_deg = degrees ? geo::GLOBALS::TO_DEGREES : 1.0;
  return std::atan(vector[1] / vector[0]) * to_deg;
}

Vector Vector::sum_of_vectors(std::vector<Vector> &vectors) {
  double sx = 0, sy = 0, sz = 0;
  for (const auto &v : vectors) {
    sx += v[0];
    sy += v[1];
    sz += v[2];
  }
  return Vector(sx, sy, sz);
}

std::array<double, 3> Vector::coordinate_direction_3angles(bool degrees) {
  double x_coord = x;
  double y_coord = y;
  double z_coord = z;
  double r = std::sqrt(x_coord * x_coord + y_coord * y_coord + z_coord * z_coord);
  
  if (r == 0) {
    return {0, 0, 0};
  }
  
  // unit vector proportions
  double x_proportion = x_coord / r;
  double y_proportion = y_coord / r;
  double z_proportion = z_coord / r;
  
  // angles
  double alpha = std::acos(x_proportion);
  double beta = std::acos(y_proportion);
  double gamma = std::acos(z_proportion);
  
  if (degrees) {
    alpha = alpha * 180.0 / geo::GLOBALS::PI;
    beta = beta * 180.0 / geo::GLOBALS::PI;
    gamma = gamma * 180.0 / geo::GLOBALS::PI;
  }
  
  return {alpha, beta, gamma};
}

std::array<double, 2> Vector::coordinate_direction_2angles(bool degrees) {
  double x_coord = x;
  double y_coord = y;
  double z_coord = z;
  double r = std::sqrt(x_coord * x_coord + y_coord * y_coord + z_coord * z_coord);
  
  if (r == 0) {
    return {0, 0};
  }
  
  // spherical coordinates
  double phi = std::acos(z_coord / r);
  double theta = std::atan2(y_coord, x_coord);
  
  if (degrees) {
    phi = phi * 180.0 / geo::GLOBALS::PI;
    theta = theta * 180.0 / geo::GLOBALS::PI;
  }
  
  return {phi, theta};
}

bool Vector::perpendicular_to(Vector &v) {
  int i, j, k;
  double a, b;
  k = 2;
  if (std::fabs(v[1]) > std::fabs(v[0])) {
    if (std::fabs(v[2]) > std::fabs(v[1])) {
      // |v[2]| > |v[1]| > |v[0]|
      i = 2; j = 1; k = 0; a = v[2]; b = -v[1];
    } else if (std::fabs(v[2]) >= std::fabs(v[0])) {
      // |v[1]| >= |v[2]| >= |v[0]|
      i = 1; j = 2; k = 0; a = v[1]; b = -v[2];
    } else {
      // |v[1]| > |v[0]| > |v[2]|
      i = 1; j = 0; k = 2; a = v[1]; b = -v[0];
    }
  } else if (std::fabs(v[2]) > std::fabs(v[0])) {
    // |v[2]| > |v[0]| >= |v[1]|
    i = 2; j = 0; k = 1; a = v[2]; b = -v[0];
  } else if (std::fabs(v[2]) > std::fabs(v[1])) {
    // |v[0]| >= |v[2]| > |v[1]|
    i = 0; j = 2; k = 1; a = v[0]; b = -v[2];
  } else {
    // |v[0]| >= |v[1]| >= |v[2]|
    i = 0; j = 1; k = 2; a = v[0]; b = -v[1];
  }

  double arr[3] = {x, y, z};
  arr[i] = b;
  arr[j] = a;
  arr[k] = 0.0;
  x = arr[0];
  y = arr[1];
  z = arr[2];
  return (a != 0.0) ? true : false;
}

void Vector::scale(double factor) {
  x *= factor; y *= factor; z *= factor;
}

void Vector::scale_up() { scale(geo::GLOBALS::SCALE); }

void Vector::scale_down() { scale(1.0 / geo::GLOBALS::SCALE); }

void Vector::rescale(double factor) {
  unitize();
  scale(factor);
}

Vector Vector::rescaled(double factor) {
  Vector v(x, y, z);
  v.unitize();
  v.scale(factor);
  return v;
}
///////////////////////////////////////////////////////////////////////////////////////////
// Not class methods
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream &operator<<(std::ostream &os, const Vector &point) {
  return os << point.to_string();
}

} // namespace session_cpp