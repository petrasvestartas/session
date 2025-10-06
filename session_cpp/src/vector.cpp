#include "vector.h"
#include <algorithm>
#include <limits>
#include <cmath>

namespace session_cpp {

///////////////////////////////////////////////////////////////////////////////////////////
// Operators
///////////////////////////////////////////////////////////////////////////////////////////

/// Convert vector to string representation
std::string Vector::to_string() const {
  return fmt::format("Vector({}, {}, {}, {}, {})", _x, _y, _z, guid, name);
}

/// Equality operator
bool Vector::operator==(const Vector &other) const {
  return _x == other._x && _y == other._y && _z == other._z;
}

/// Inequality operator
bool Vector::operator!=(const Vector &other) const { return !(*this == other); }

/////////////////////////////////////////////////////////////////////////////////////////////
// No-copy operators
///////////////////////////////////////////////////////////////////////////////////////////

float &Vector::operator[](int index) {
  invalidate_length_cache();
  if (index == 0)
    return _x;
  if (index == 1)
    return _y;
  return _z; // assume index == 2
}

const float &Vector::operator[](int index) const {
  if (index == 0)
    return _x;
  if (index == 1)
    return _y;
  return _z; // assume index == 2
}

Vector &Vector::operator*=(float factor) {
  set_x(_x * factor);
  set_y(_y * factor);
  set_z(_z * factor);
  return *this;
}

Vector &Vector::operator/=(float factor) {
  set_x(_x / factor);
  set_y(_y / factor);
  set_z(_z / factor);
  return *this;
}

Vector &Vector::operator+=(const Vector &other) {
  set_x(_x + other._x);
  set_y(_y + other._y);
  set_z(_z + other._z);
  return *this;
}

Vector &Vector::operator-=(const Vector &other) {
  set_x(_x - other._x);
  set_y(_y - other._y);
  set_z(_z - other._z);
  return *this;
}

///////////////////////////////////////////////////////////////////////////////////////////
// Copy operators
///////////////////////////////////////////////////////////////////////////////////////////

Vector Vector::operator*(float factor) const { return Vector(_x * factor, _y * factor, _z * factor); }

Vector Vector::operator/(float factor) const { return Vector(_x / factor, _y / factor, _z / factor); }

Vector Vector::operator+(const Vector &other) const {
  return Vector(_x + other._x, _y + other._y, _z + other._z);
}

Vector Vector::operator-(const Vector &other) const {
  return Vector(_x - other._x, _y - other._y, _z - other._z);
}

Vector operator*(float factor, const Vector &v) { return v * factor; }

///////////////////////////////////////////////////////////////////////////////////////////
// JSON
///////////////////////////////////////////////////////////////////////////////////////////

/// Convert to JSON-serializable object
nlohmann::ordered_json Vector::to_json_data() const {
  auto clean_float = [](float val) -> double { return static_cast<double>(std::round(val * 100.0f) / 100.0f); };
  return nlohmann::ordered_json{{"type", "Vector"}, {"guid", guid},
                                {"name", name},     {"x", clean_float(_x)},
                                {"y", clean_float(_y)}, {"z", clean_float(_z)}};
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

Vector Vector::x_axis() { return Vector(1.0f, 0.0f, 0.0f); }
Vector Vector::y_axis() { return Vector(0.0f, 1.0f, 0.0f); }
Vector Vector::z_axis() { return Vector(0.0f, 0.0f, 1.0f); }

Vector Vector::from_start_and_end(const Vector &start, const Vector &end) {
  return Vector(end._x - start._x, end._y - start._y, end._z - start._z);
}

///////////////////////////////////////////////////////////////////////////////////////////
// Details / Geometry
///////////////////////////////////////////////////////////////////////////////////////////

void Vector::reverse() {
  set_x(-_x);
  set_y(-_y);
  set_z(-_z);
  // Length magnitude stays the same, no need to invalidate cache
}

float Vector::compute_length() const {
  float len = 0.0f;

  float ax = std::abs(_x);
  float ay = std::abs(_y);
  float az = std::abs(_z);

  const bool x_zero = ax < static_cast<float>(session_cpp::Tolerance::ZERO_TOLERANCE);
  const bool y_zero = ay < static_cast<float>(session_cpp::Tolerance::ZERO_TOLERANCE);
  const bool z_zero = az < static_cast<float>(session_cpp::Tolerance::ZERO_TOLERANCE);

  if (x_zero && y_zero && z_zero)
    return 0.0f;
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

  if (ax > std::numeric_limits<float>::min()) {
    ay /= ax;
    az /= ax;
    len = ax * sqrtf(1.0f + ay * ay + az * az);
  } else if (ax > 0.0f && session_cpp::is_finite(ax)) {
    len = ax;
  } else {
    len = 0.0f;
  }

  return len;
}

float Vector::cached_length() const {
  if (!_has_length) {
    _length = compute_length();
    _has_length = true;
  }
  return _length;
}

float Vector::magnitude() const { return cached_length(); }

float Vector::length_squared() const {
  return _x * _x + _y * _y + _z * _z;
}

bool Vector::normalize_self() {
  float d = compute_length();
  if (d > 0.0f) {
    set_x(_x / d);
    set_y(_y / d);
    set_z(_z / d);
    return true;
  }
  return false;
}


std::tuple<Vector, float, Vector, float>
Vector::projection(Vector &projection_vector, float tolerance) {
  float projection_vector_length = projection_vector.magnitude();

  if (projection_vector_length < tolerance) {
    return {Vector(0, 0, 0), 0.0f, Vector(0, 0, 0), 0.0f};
  }

  Vector projection_vector_unit(
      projection_vector._x / projection_vector_length,
      projection_vector._y / projection_vector_length,
      projection_vector._z / projection_vector_length);

  float projected_vector_length = this->dot(projection_vector_unit);
  Vector out_projection_vector = projection_vector_unit * projected_vector_length;

  Vector out_perpendicular_projected_vector = *this - out_projection_vector;
  float out_perpendicular_projected_vector_length = out_perpendicular_projected_vector.magnitude();

  return {out_projection_vector,
          projected_vector_length,
          out_perpendicular_projected_vector,
          out_perpendicular_projected_vector_length};
}

int Vector::is_parallel_to(const Vector &other) {
  float ll = cached_length() * other.cached_length();
  int result;
  
  if (ll > 0.0f) {
    const float cos_angle = ((*this)[0] * other[0] + (*this)[1] * other[1] + (*this)[2] * other[2]) / ll;
    const float angle_in_radians = static_cast<float>(session_cpp::Tolerance::ANGLE_TOLERANCE_DEGREES) * static_cast<float>(session_cpp::TO_RADIANS);
    const float cos_tol = cosf(angle_in_radians);
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

float Vector::dot(const Vector &other) {
  float result = 0.0f;
  for (int i = 0; i < 3; ++i) {
    result += (*this)[i] * other[i];
  }
  return result;
}

Vector Vector::cross(const Vector &other) {
  float cx = (*this)[1] * other[2] - (*this)[2] * other[1];
  float cy = (*this)[2] * other[0] - (*this)[0] * other[2];
  float cz = (*this)[0] * other[1] - (*this)[1] * other[0];
  return Vector(cx, cy, cz);
}

float Vector::angle(const Vector &other, bool sign_by_cross_product, bool degrees,
                     float tolerance) {
  float dotp = this->dot(other);
  float len0 = this->cached_length();
  float len1 = other.cached_length();
  float denom = len0 * len1;
  if (denom < tolerance) {
    return 0.0f;
  }
  float cos_angle = dotp / denom;
  cos_angle = std::max(-1.0f, std::min(1.0f, cos_angle));
  float ang = acosf(cos_angle);
  if (sign_by_cross_product) {
    Vector cp = this->cross(other);
    if (cp._z < 0)
      ang = -ang;
  }
  float to_degrees = degrees ? static_cast<float>(session_cpp::TO_DEGREES) : 1.0f;
  return ang * to_degrees;
}

Vector Vector::get_leveled_vector(float &vertical_height) {
  Vector copy(_x, _y, _z);
  if (copy.normalize_self()) {
    Vector reference(0, 0, 1);
    float angle = copy.angle(reference, true); // returns degrees
    // CRITICAL: statics bug - passes degrees directly to cos (expects radians)
    float inclined_offset_by_vertical_distance = vertical_height / std::cos(angle);
    copy *= inclined_offset_by_vertical_distance;
  }
  return copy;
}

float Vector::cosine_law(float &a, float &b, float &ang_between, bool degrees) {
  float to_rad = degrees ? static_cast<float>(session_cpp::TO_RADIANS) : 1.0f;
  return sqrtf(a * a + b * b - 2.0f * a * b * cosf(ang_between * to_rad));
}

float Vector::sine_law_angle(float &a, float &A, float &b, bool degrees) {
  float to_rad = degrees ? static_cast<float>(session_cpp::TO_RADIANS) : 1.0f;
  float to_deg = degrees ? static_cast<float>(session_cpp::TO_DEGREES) : 1.0f;
  return asinf((b * sinf(A * to_rad)) / a) * to_deg;
}

float Vector::sine_law_length(float &a, float &A, float &B, bool degrees) {
  float to_rad = degrees ? static_cast<float>(session_cpp::TO_RADIANS) : 1.0f;
  return (a * sinf(B * to_rad)) / sinf(A * to_rad);
}

float Vector::angle_between_vector_xy_components(Vector &vector) {
  return atan2f(vector[1], vector[0]) * static_cast<float>(session_cpp::TO_DEGREES);
}

Vector Vector::sum_of_vectors(std::vector<Vector> &vectors) {
  float sx = 0, sy = 0, sz = 0;
  for (const auto &v : vectors) {
    sx += v[0];
    sy += v[1];
    sz += v[2];
  }
  return Vector(sx, sy, sz);
}

std::array<float, 3> Vector::coordinate_direction_3angles(bool degrees) {
  float x_coord = _x;
  float y_coord = _y;
  float z_coord = _z;
  float r = sqrtf(x_coord * x_coord + y_coord * y_coord + z_coord * z_coord);
  
  if (r == 0) {
    return {0, 0, 0};
  }
  
  // unit vector proportions
  float x_proportion = x_coord / r;
  float y_proportion = y_coord / r;
  float z_proportion = z_coord / r;
  
  // angles
  float alpha = acosf(x_proportion);
  float beta = acosf(y_proportion);
  float gamma = acosf(z_proportion);
  
  if (degrees) {
    alpha = alpha * static_cast<float>(session_cpp::TO_DEGREES);
    beta = beta * static_cast<float>(session_cpp::TO_DEGREES);
    gamma = gamma * static_cast<float>(session_cpp::TO_DEGREES);
  }
  
  return {alpha, beta, gamma};
}

std::array<float, 2> Vector::coordinate_direction_2angles(bool degrees) {
  float x_coord = _x;
  float y_coord = _y;
  float z_coord = _z;
  float r = std::sqrt(x_coord * x_coord + y_coord * y_coord + z_coord * z_coord);
  
  if (r == 0) {
    return {0, 0};
  }
  
  // spherical coordinates
  float phi = acosf(z_coord / r);
  float theta = atan2f(y_coord, x_coord);
  
  if (degrees) {
    phi = phi * static_cast<float>(session_cpp::TO_DEGREES);
    theta = theta * static_cast<float>(session_cpp::TO_DEGREES);
  }
  
  return {phi, theta};
}

bool Vector::perpendicular_to(Vector &v) {
  int i, j, k;
  float a, b;
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

  float arr[3] = {_x, _y, _z};
  arr[i] = b;
  arr[j] = a;
  arr[k] = 0.0f;
  set_x(arr[0]);
  set_y(arr[1]);
  set_z(arr[2]);
  return (a != 0.0f) ? true : false;
}

void Vector::scale(float factor) {
  set_x(_x * factor);
  set_y(_y * factor);
  set_z(_z * factor);
}

void Vector::scale_up() { scale(static_cast<float>(session_cpp::SCALE)); }

void Vector::scale_down() { scale(1.0f / static_cast<float>(session_cpp::SCALE)); }

///////////////////////////////////////////////////////////////////////////////////////////
// Not class methods
///////////////////////////////////////////////////////////////////////////////////////////

std::ostream &operator<<(std::ostream &os, const Vector &point) {
  return os << point.to_string();
}

} // namespace session_cpp