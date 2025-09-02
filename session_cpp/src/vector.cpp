#include "vector.hpp"

namespace geo {

Vector::Vector() : _xyz{0.0, 0.0, 0.0} {}

Vector::Vector(double x, double y, double z) {
    _xyz[0] = x;
    _xyz[1] = y;
    _xyz[2] = z;
    _has_length = false;
}

Vector Vector::from_scalars(double a, double b, double c) {
    Vector v(0, 0, 0);
    v(0) = a;
    v(1) = b;
    v(2) = c;
    return v;
}

Vector Vector::XAxis() { return Vector(1, 0, 0); }

Vector Vector::YAxis() { return Vector(0, 1, 0); }

Vector Vector::ZAxis() { return Vector(0, 0, 1); }

Vector Vector::xc() const { return Vector(_xyz[0], 0, 0); }

Vector Vector::yc() const { return Vector(0, _xyz[1], 0); }

Vector Vector::zc() const { return Vector(0, 0, _xyz[2]); }

Vector Vector::from_start_and_end(const Vector& start, const Vector& end) {
    return Vector(end._xyz[0] - start._xyz[0], end._xyz[1] - start._xyz[1],
                  end._xyz[2] - start._xyz[2]);
}

double& Vector::operator[](int index) { return _xyz[index]; }

const double& Vector::operator[](int index) const { return _xyz[index]; }

double& Vector::operator()(int index) { return _abc[index]; }

const double& Vector::operator()(int index) const { return _abc[index]; }

Vector& Vector::operator*=(double factor) {
    for (size_t i = 0; i < 3; ++i) _xyz[i] *= factor;
    _has_length = false;
    return *this;
}

// Friend non-member operator* overload to handle double * Vector
Vector operator*(double factor, const Vector& v) {
    return v * factor;  // Reuse the member operator* overload
}

Vector& Vector::operator/=(double factor) {
    for (size_t i = 0; i < 3; ++i) _xyz[i] /= factor;
    _has_length = false;
    return *this;
}

Vector& Vector::operator+=(const Vector& other) {
    for (size_t i = 0; i < 3; ++i) _xyz[i] += other._xyz[i];
    _has_length = false;
    return *this;
}

Vector& Vector::operator-=(const Vector& other) {
    for (size_t i = 0; i < 3; ++i) _xyz[i] -= other._xyz[i];
    _has_length = false;
    return *this;
}

Vector Vector::operator*(double factor) const {
    Vector result = *this;
    result *= factor;
    result._has_length = false;
    return result;
}

Vector Vector::operator/(double factor) const {
    Vector result = *this;
    result /= factor;
    result._has_length = false;
    return result;
}

Vector Vector::operator+(const Vector& other) const {
    Vector result = *this;
    result += other;
    result._has_length = false;
    return result;
}

Vector Vector::operator-(const Vector& other) const {
    Vector result = *this;
    result -= other;
    result._has_length = false;
    return result;
}

bool Vector::operator==(const Vector& other) const {
    // Replace x, y, z with your actual member variables
    return _xyz[0] == other[0] && _xyz[1] == other[1] && _xyz[2] == other[2];
}

bool Vector::operator!=(const Vector& other) const { return !(*this == other); }

void Vector::reverse() {
    for (size_t i = 0; i < 3; ++i) _xyz[i] = -_xyz[i];
}

double Vector::length(double predefined_length) {
    if (predefined_length != 0.0) {
        // _abc unit vector parameters are needed:
        _xyz[0] = _abc[0] * predefined_length;
        _xyz[1] = _abc[1] * predefined_length;
        _xyz[2] = _abc[2] * predefined_length;
        _has_length = false;
        length();
        _has_length = true;
    }

    if (!_has_length) {
        _length = compute_length();
        _has_length = true;
    }

    return _length;
}

double Vector::compute_length() const {
    double length = 0;

    double x = abs(_xyz[0]);
    double y = abs(_xyz[1]);
    double z = abs(_xyz[2]);

    // Handle two zero case:
    bool x_zero = x < geo::GLOBALS::ZERO_TOLERANCE;
    bool y_zero = y < geo::GLOBALS::ZERO_TOLERANCE;
    bool z_zero = z < geo::GLOBALS::ZERO_TOLERANCE;

    if (x_zero && y_zero && z_zero) {
        length = 0.0;
        return length;
    } else if (x_zero && y_zero) {
        length = z;
        return length;
    } else if (x_zero && z_zero) {
        length = y;
        return length;
    } else if (y_zero && z_zero) {
        length = x;
        return length;
    }

    // Handle one or none zero case:

    if (y >= x && y >= z) {
        length = x;
        x = y;
        y = length;
    } else if (z >= x && z >= y) {
        length = x;
        x = z;
        z = length;
    }

    // For small denormalized doubles (positive but smaller
    // than DOUBLE_MIN), some compilers/FPUs set 1.0/x to +INF.
    // Without the DOUBLE_MIN test we end up with
    // microscopic vectors that have infinite length!
    if (x > GLOBALS::DOUBLE_MIN) {
        y /= x;
        z /= x;
        length = x * sqrt(1.0 + y * y + z * z);
    } else if (x > 0.0 && GLOBALS::IS_FINITE(x))
        length = x;
    else
        length = 0.0;

    return length;
}

bool Vector::unitize() {
    bool rc = false;
    double d = length();
    if (d > 0.0) {
        _xyz[0] /= d;
        _xyz[1] /= d;
        _xyz[2] /= d;
        rc = true;
        _length = 1;
        _has_length = true;
    }
    return rc;
}

Vector Vector::unitized() {
    Vector unitized_vector = *this;
    unitized_vector.unitize();
    return unitized_vector;
}

Vector Vector::projection(Vector& projection_vector, double tolerance,
                          double* out_projected_vector_length,
                          Vector* out_perpendicular_projected_vector,
                          double* out_perpendicular_projected_vector_length) {
    // unitize vector_to_project_on
    double projection_vector_length = projection_vector.length();
    Vector projection_vector_unit = Vector(projection_vector[0] / projection_vector_length,
                                           projection_vector[1] / projection_vector_length,
                                           projection_vector[2] / projection_vector_length);

    // Before unitizing, check if the length is below a tolerance value
    if (projection_vector_length < tolerance) {
        // Handle the case where the projection_vector_length is too small
        // This could involve setting default values or returning an error
        if (out_projected_vector_length) *out_projected_vector_length = 0;
        if (out_perpendicular_projected_vector)
            *out_perpendicular_projected_vector = Vector(0, 0, 0);
        if (out_perpendicular_projected_vector_length)
            *out_perpendicular_projected_vector_length = 0;
        return Vector(0, 0, 0);  // Return a zero vector or handle as appropriate
    }

    // get the scale factor of the projection vector by dot product
    double projected_vector_length = this->dot(projection_vector_unit);
    if (out_projected_vector_length) {
        *out_projected_vector_length = projected_vector_length;
    }

    // get the projection vector
    Vector out_projection_vector = projection_vector_unit * projected_vector_length;

    // get the perpendicular vector
    if (out_perpendicular_projected_vector) {
        *out_perpendicular_projected_vector = *this - out_projection_vector;
        if (out_perpendicular_projected_vector_length) {
            *out_perpendicular_projected_vector_length =
                out_perpendicular_projected_vector->length();
        }
    } else if (out_perpendicular_projected_vector_length) {
        // Calculate length only if needed and perpendicular vector is not requested
        Vector temp_perpendicular_vector = *this - out_projection_vector;
        *out_perpendicular_projected_vector_length = temp_perpendicular_vector.length();
    }

    return out_projection_vector;
}

int Vector::is_parallel_to(Vector& v) {
    double ll = length() * v.length();
    int result;

    if (ll > 0.0) {
        const double cos_angle = (_xyz[0] * v[0] + _xyz[1] * v[1] + _xyz[2] * v[2]) / ll;

        const double angle_in_radians =
            GLOBALS::ANGLE * (GLOBALS::PI / 180.0);  // convert angle from degrees to radians
        const double cos_tol = cos(angle_in_radians);
        if (cos_angle >= cos_tol)
            result = 1;  // "Parallel";
        else if (cos_angle <= -cos_tol)
            result = -1;  // "Antiparallel"
        else
            result = 0;  // "Not parallel"
    } else {
        result = 0;  // "Not parallel"
    }

    return result;
}

double Vector::dot(Vector& other) {
    double result = 0.0;
    for (size_t i = 0; i < 3; ++i) {
        result += _xyz[i] * other._xyz[i];
    }

    return result;
}

Vector Vector::cross(Vector& other) {
    double x = _xyz[1] * other._xyz[2] - _xyz[2] * other._xyz[1];
    double y = _xyz[2] * other._xyz[0] - _xyz[0] * other._xyz[2];
    double z = _xyz[0] * other._xyz[1] - _xyz[1] * other._xyz[0];
    Vector result = Vector(x, y, z);
    result.unitize();

    return result;
}

double Vector::angle(Vector& other, bool sign_by_cross_product, bool degrees, double tolerance) {
    double dot = this->dot(other);
    double len0 = this->length();
    double len1 = other.length();

    // Ensure the denominator is not zero
    double denominator = len0 * len1;
    if (denominator < tolerance) {
        return 0;  // Or handle the error as appropriate
    }

    // Calculate the cosine of the angle
    double cos_angle = dot / denominator;

    // Clamp cos_angle to the range [-1, 1] to avoid NaN errors due to floating point inaccuracies
    cos_angle = std::max(-1.0, std::min(1.0, cos_angle));

    // Calculate the angle in radians
    double angle = acos(cos_angle);

    // Determine the sign of the angle using the z-component of the cross product
    if (sign_by_cross_product) {
        Vector crossProduct = this->cross(other);
        if (crossProduct._xyz[2] < 0) {
            angle = -angle;
        }
    }

    // Return the angle in degrees if requested
    double to_degrees = degrees ? GLOBALS::TO_DEGREES : 1.0;

    return angle * to_degrees;
}

Vector Vector::get_leveled_vector(double& vertical_height) {
    Vector copy = Vector(_xyz[0], _xyz[1], _xyz[2]);

    if (copy.unitize()) {
        Vector referenceVector(0, 0, 1);
        double angle = copy.angle(referenceVector, true);
        double inclined_offset_by_vertical_distance = vertical_height / std::cos(angle);
        copy *= inclined_offset_by_vertical_distance;
    }

    return copy;
}

double Vector::cosine_law(double& triangle_edge_length_a, double& triangle_edge_length_b,
                          double& angle_in_between_edges, bool degrees) {
    double to_radians = degrees ? GLOBALS::TO_RADIANS : 1;
    return sqrt(pow(triangle_edge_length_a, 2) + pow(triangle_edge_length_b, 2) -
                2 * triangle_edge_length_a * triangle_edge_length_b *
                    cos(angle_in_between_edges * to_radians));
}

double Vector::sine_law_angle(double& triangle_edge_length_a, double& angle_in_front_of_a,
                              double& triangle_edge_length_b, bool degrees) {
    double to_radians = degrees ? GLOBALS::TO_RADIANS : 1;
    double to_degrees = degrees ? GLOBALS::TO_DEGREES : 1;
    return asin((triangle_edge_length_b * sin(angle_in_front_of_a * to_radians)) /
                triangle_edge_length_a) *
           to_degrees;
}

double Vector::sine_law_length(double& triangle_edge_length_a, double& angle_in_front_of_a,
                               double& angle_in_front_of_b, bool degrees) {
    double to_radians = degrees ? GLOBALS::TO_RADIANS : 1;
    return (triangle_edge_length_a * sin(angle_in_front_of_b * to_radians)) /
           sin(angle_in_front_of_a * to_radians);
}

double Vector::angle_between_vector_xy_components_degrees(Vector& vector, bool degrees) {
    double to_degrees = degrees ? GLOBALS::TO_DEGREES : 1;
    return atan(vector[1] / vector[0]) * to_degrees;
}

Vector Vector::sum_of_vectors(std::vector<Vector>& vectors) {
    double x = 0, y = 0, z = 0;
    for (Vector vector : vectors) {
        x += vector[0];
        y += vector[1];
        z += vector[2];
    }
    return Vector(x, y, z);
}

std::array<double, 3> Vector::coordinate_direction_3angles(bool degrees) {
    double x = _xyz[0];
    double y = _xyz[1];
    double z = _xyz[2];
    double r = sqrt(x * x + y * y + z * z);

    // unit vector
    // u = F/|F| = F_x/|F| + F_y/|F| + F_z/|F|
    double x_proportion = x / r;
    double y_proportion = y / r;
    double z_proportion = z / r;

    // angles
    double alpha = acos(x_proportion);
    double beta = acos(y_proportion);
    double gamma = acos(z_proportion);

    if (degrees) {
        alpha = alpha * 180.0 / GLOBALS::PI;
        beta = beta * 180.0 / GLOBALS::PI;
        gamma = gamma * 180.0 / GLOBALS::PI;
    }

    return std::array<double, 3>{alpha, beta, gamma};
}

std::array<double, 2> Vector::coordinate_direction_2angles(bool degrees) {
    double x = _xyz[0];
    double y = _xyz[1];
    double z = _xyz[2];
    double r = sqrt(x * x + y * y + z * z);

    // unit vector
    // u = |v|*sin(phi)*cos(theta) i + |v|*sin(phi)*sin(theta) j + |v|*cos(phi) k
    // from the unit vector, lets get angle phi and theta

    double phi = acos(z / r);
    double theta = atan2(y, x);

    if (degrees) {
        phi = phi * 180.0 / GLOBALS::PI;
        theta = theta * 180.0 / GLOBALS::PI;
    }

    return std::array<double, 2>{phi, theta};
}

bool Vector::perpendicular_to(Vector& v) {
    int i, j, k;
    double a, b;
    k = 2;
    if (fabs(v[1]) > fabs(v[0])) {
        if (fabs(v[2]) > fabs(v[1])) {
            // |v[2]| > |v[1]| > |v[0]|
            i = 2;
            j = 1;
            k = 0;
            a = v[2];
            b = -v[1];
        } else if (fabs(v[2]) >= fabs(v[0])) {
            // |v[1]| >= |v[2]| >= |v[0]|
            i = 1;
            j = 2;
            k = 0;
            a = v[1];
            b = -v[2];
        } else {
            // |v[1]| > |v[0]| > |v[2]|
            i = 1;
            j = 0;
            k = 2;
            a = v[1];
            b = -v[0];
        }
    } else if (fabs(v[2]) > fabs(v[0])) {
        // |v[2]| > |v[0]| >= |v[1]|
        i = 2;
        j = 0;
        k = 1;
        a = v[2];
        b = -v[0];
    } else if (fabs(v[2]) > fabs(v[1])) {
        // |v[0]| >= |v[2]| > |v[1]|
        i = 0;
        j = 2;
        k = 1;
        a = v[0];
        b = -v[2];
    } else {
        // |v[0]| >= |v[1]| >= |v[2]|
        i = 0;
        j = 1;
        k = 2;
        a = v[0];
        b = -v[1];
    }

    _xyz[i] = b;
    _xyz[j] = a;
    _xyz[k] = 0.0;
    return (a != 0.0) ? true : false;
}

void Vector::scale(double factor) {
    _xyz[0] *= factor;
    _xyz[1] *= factor;
    _xyz[2] *= factor;
    _has_length = false;
    _has_unit_vector = false;
}

void Vector::scale_up() {
    scale(GLOBALS::SCALE);
    _has_length = false;
}
void Vector::scale_down() {
    scale(1.0 / GLOBALS::SCALE);
    _has_length = false;
}

void Vector::rescale(double factor) {
    unitize();
    scale(factor);
    _has_length = false;
}

Vector Vector::rescaled(double factor) {
    Vector v(_xyz[0], _xyz[1], _xyz[2]);
    v.unitize();
    v.scale(factor);
    return v;
}

std::string Vector::to_string() {
    std::ostringstream oss;

    oss << "geo::Vector " << _xyz[0] << " " << _xyz[1] << " " << _xyz[2] << " " << "length" << " "
        << this->length() << " " << "scalar" << " " << _abc[0] << " " << _abc[1] << " " << _abc[2];

    return oss.str();
}

}  // namespace geo