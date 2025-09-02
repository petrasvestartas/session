#include "xform.hpp"

#include <cmath>

namespace geo {

namespace internal {
double length(double x, double y, double z) {
    double len;
    x = fabs(x);
    y = fabs(y);
    z = fabs(z);
    if (y >= x && y >= z) {
        len = x;
        x = y;
        y = len;
    } else if (z >= x && z >= y) {
        len = x;
        x = z;
        z = len;
    }

    if (x > geo::GLOBALS::DOUBLE_MIN) {
        y /= x;
        z /= x;
        len = x * sqrt(1.0 + y * y + z * z);
    } else if (x > 0.0 && std::isfinite(x)) {
        len = x;
    } else {
        len = 0.0;
    }

    return len;
}

bool unitize(geo::Vector& vector) {
    bool rc = false;
    double d = length(vector[0], vector[1], vector[2]);
    if (d > 0.0) {
        vector = geo::Vector(vector[0] / d, vector[1] / d, vector[2] / d);
        rc = true;
    }
    return rc;
}
}  // namespace internal

// Static method for translation transformation
Matrix Xform::translation(double tx, double ty, double tz) {
    Matrix result(4, 4);
    result(0, 0) = 1;
    result(1, 1) = 1;
    result(2, 2) = 1;
    result(3, 3) = 1;
    result(0, 3) = tx;
    result(1, 3) = ty;
    result(2, 3) = tz;
    return result;
}

// Static method for scaling transformation
Matrix Xform::scaling(double sx, double sy, double sz) {
    Matrix result(4, 4);
    result(0, 0) = sx;
    result(1, 1) = sy;
    result(2, 2) = sz;
    result(3, 3) = 1;
    return result;
}

// Static method for change of basis transformation
Matrix Xform::change_basis(geo::Point& origin_1, geo::Vector& x_axis_1, geo::Vector& y_axis_1,
                           geo::Vector& z_axis_1, geo::Point& origin_0, geo::Vector& x_axis_0,
                           geo::Vector& y_axis_0, geo::Vector& z_axis_0) {
    // Q = a0*x_axis_0 + b0*y_axis_0 + c0*z_axis_0 = a1*x_axis_1 + b1*y_axis_1 + c1*z_axis_1
    // then this transform will map the point (a0,b0,c0) to (a1,b1,c1)

    double a, b, c, d;
    a = x_axis_1.dot(y_axis_1);
    b = x_axis_1.dot(z_axis_1);
    c = y_axis_1.dot(z_axis_1);
    double R[3][6] = {{x_axis_1.dot(x_axis_1), a, b, x_axis_1.dot(x_axis_0), x_axis_1.dot(y_axis_0),
                       x_axis_1.dot(z_axis_0)},
                      {a, y_axis_1.dot(y_axis_1), c, y_axis_1.dot(x_axis_0), y_axis_1.dot(y_axis_0),
                       y_axis_1.dot(z_axis_0)},
                      {b, c, z_axis_1.dot(z_axis_1), z_axis_1.dot(x_axis_0), z_axis_1.dot(y_axis_0),
                       z_axis_1.dot(z_axis_0)}};

    // Row reduce R
    int i0 = (R[0][0] >= R[1][1]) ? 0 : 1;
    if (R[2][2] > R[i0][i0]) i0 = 2;
    int i1 = (i0 + 1) % 3;
    int i2 = (i1 + 1) % 3;

    if (R[i0][i0] == 0.0) return Matrix::identity(4);

    d = 1.0 / R[i0][i0];
    for (int j = 0; j < 6; ++j) R[i0][j] *= d;
    R[i0][i0] = 1.0;

    if (R[i1][i0] != 0.0) {
        d = -R[i1][i0];
        for (int j = 0; j < 6; ++j) R[i1][j] += d * R[i0][j];
        R[i1][i0] = 0.0;
    }
    if (R[i2][i0] != 0.0) {
        d = -R[i2][i0];
        for (int j = 0; j < 6; ++j) R[i2][j] += d * R[i0][j];
        R[i2][i0] = 0.0;
    }

    if (fabs(R[i1][i1]) < fabs(R[i2][i2])) std::swap(i1, i2);
    if (R[i1][i1] == 0.0) return Matrix::identity(4);

    d = 1.0 / R[i1][i1];
    for (int j = 0; j < 6; ++j) R[i1][j] *= d;
    R[i1][i1] = 1.0;

    if (R[i0][i1] != 0.0) {
        d = -R[i0][i1];
        for (int j = 0; j < 6; ++j) R[i0][j] += d * R[i1][j];
        R[i0][i1] = 0.0;
    }
    if (R[i2][i1] != 0.0) {
        d = -R[i2][i1];
        for (int j = 0; j < 6; ++j) R[i2][j] += d * R[i1][j];
        R[i2][i1] = 0.0;
    }

    if (R[i2][i2] == 0.0) return Matrix::identity(4);

    d = 1.0 / R[i2][i2];
    for (int j = 0; j < 6; ++j) R[i2][j] *= d;
    R[i2][i2] = 1.0;

    if (R[i0][i2] != 0.0) {
        d = -R[i0][i2];
        for (int j = 0; j < 6; ++j) R[i0][j] += d * R[i2][j];
        R[i0][i2] = 0.0;
    }
    if (R[i1][i2] != 0.0) {
        d = -R[i1][i2];
        for (int j = 0; j < 6; ++j) R[i1][j] += d * R[i2][j];
        R[i1][i2] = 0.0;
    }

    Matrix m_xform(4, 4);
    m_xform(0, 0) = R[0][3];
    m_xform(0, 1) = R[0][4];
    m_xform(0, 2) = R[0][5];
    m_xform(1, 0) = R[1][3];
    m_xform(1, 1) = R[1][4];
    m_xform(1, 2) = R[1][5];
    m_xform(2, 0) = R[2][3];
    m_xform(2, 1) = R[2][4];
    m_xform(2, 2) = R[2][5];
    m_xform(3, 3) = 1.0;

    Matrix T0 = Xform::translation(-origin_1[0], -origin_1[1], -origin_1[2]);
    Matrix T2 = Xform::translation(origin_0[0], origin_0[1], origin_0[2]);
    return T2 * m_xform * T0;
}

// Static method for plane-to-plane transformation
Matrix Xform::plane_to_plane(geo::Point& origin_0, geo::Vector& x_axis_0, geo::Vector& y_axis_0,
                             geo::Point& origin_1, geo::Vector& x_axis_1, geo::Vector& y_axis_1) {
    geo::Vector z_axis_0 = x_axis_0.cross(y_axis_0);
    geo::Vector z_axis_1 = x_axis_1.cross(y_axis_1);

    geo::Vector _x_axis_0 = x_axis_0;
    geo::Vector _y_axis_0 = y_axis_0;
    geo::Vector _z_axis_0 = z_axis_0;
    internal::unitize(_x_axis_0);
    internal::unitize(_y_axis_0);
    internal::unitize(_z_axis_0);

    geo::Vector _x_axis_1 = x_axis_1;
    geo::Vector _y_axis_1 = y_axis_1;
    geo::Vector _z_axis_1 = z_axis_1;
    internal::unitize(_x_axis_1);
    internal::unitize(_y_axis_1);
    internal::unitize(_z_axis_1);

    Matrix t0 = Xform::translation(-origin_0[0], -origin_0[1], -origin_0[2]);

    Matrix f0(4, 4);
    f0(0, 0) = _x_axis_0[0];
    f0(0, 1) = _x_axis_0[1];
    f0(0, 2) = _x_axis_0[2];
    f0(1, 0) = _y_axis_0[0];
    f0(1, 1) = _y_axis_0[1];
    f0(1, 2) = _y_axis_0[2];
    f0(2, 0) = _z_axis_0[0];
    f0(2, 1) = _z_axis_0[1];
    f0(2, 2) = _z_axis_0[2];
    f0(3, 3) = 1.0;

    Matrix f1(4, 4);
    f1(0, 0) = _x_axis_1[0];
    f1(0, 1) = _y_axis_1[0];
    f1(0, 2) = _z_axis_1[0];
    f1(1, 0) = _x_axis_1[1];
    f1(1, 1) = _y_axis_1[1];
    f1(1, 2) = _z_axis_1[1];
    f1(2, 0) = _x_axis_1[2];
    f1(2, 1) = _y_axis_1[2];
    f1(2, 2) = _z_axis_1[2];
    f1(3, 3) = 1.0;

    Matrix r = f1 * f0;

    Matrix t1 = Xform::translation(origin_1[0], origin_1[1], origin_1[2]);

    return t1 * r * t0;
}

// Static method for plane to XY transformation
Matrix Xform::plane_to_xy(geo::Point& origin, geo::Vector& x_axis, geo::Vector& y_axis) {
    geo::Vector z_axis = x_axis.cross(y_axis);

    geo::Vector _x_axis = x_axis;
    geo::Vector _y_axis = y_axis;
    geo::Vector _z_axis = z_axis;
    internal::unitize(_x_axis);
    internal::unitize(_y_axis);
    internal::unitize(_z_axis);

    Matrix t = Xform::translation(-origin[0], -origin[1], -origin[2]);

    Matrix f(4, 4);
    f(0, 0) = _x_axis[0];
    f(0, 1) = _x_axis[1];
    f(0, 2) = _x_axis[2];
    f(1, 0) = _y_axis[0];
    f(1, 1) = _y_axis[1];
    f(1, 2) = _y_axis[2];
    f(2, 0) = _z_axis[0];
    f(2, 1) = _z_axis[1];
    f(2, 2) = _z_axis[2];
    f(3, 3) = 1.0;

    return f * t;
}

// Static method for XY to plane transformation
Matrix Xform::xy_to_plane(geo::Point& origin, geo::Vector& x_axis, geo::Vector& y_axis) {
    geo::Vector z_axis = x_axis.cross(y_axis);

    geo::Vector _x_axis = x_axis;
    geo::Vector _y_axis = y_axis;
    geo::Vector _z_axis = z_axis;
    internal::unitize(_x_axis);
    internal::unitize(_y_axis);
    internal::unitize(_z_axis);

    Matrix f(4, 4);
    f(0, 0) = _x_axis[0];
    f(0, 1) = _y_axis[0];
    f(0, 2) = _z_axis[0];
    f(1, 0) = _x_axis[1];
    f(1, 1) = _y_axis[1];
    f(1, 2) = _z_axis[1];
    f(2, 0) = _x_axis[2];
    f(2, 1) = _y_axis[2];
    f(2, 2) = _z_axis[2];
    f(3, 3) = 1.0;

    Matrix t = Xform::translation(origin[0], origin[1], origin[2]);

    return t * f;
}

// Static method to apply transformation to a point
geo::Point Xform::apply(const Matrix& transform, const geo::Point& point) {
    std::vector<double> p = {point.x(), point.y(), point.z(), 1.0};
    std::vector<double> result(4, 0.0);

    for (size_t i = 0; i < 4; ++i) {
        for (size_t j = 0; j < 4; ++j) {
            result[i] += transform(i, j) * p[j];
        }
    }

    return geo::Point(result[0], result[1], result[2]);
}

// Static method to apply transformation to a vector of points
std::vector<geo::Point> Xform::apply(const Matrix& transform,
                                     const std::vector<geo::Point>& points) {
    std::vector<geo::Point> transformed_points;
    transformed_points.reserve(points.size());

    for (const auto& point : points) {
        transformed_points.push_back(apply(transform, point));
    }

    return transformed_points;
}

// Static method to apply transformation to an array of points
template <std::size_t N>
std::array<geo::Point, N> Xform::apply(const Matrix& transform,
                                       const std::array<geo::Point, N>& points) {
    std::array<geo::Point, N> transformed_points;

    for (std::size_t i = 0; i < N; ++i) {
        transformed_points[i] = apply(transform, points[i]);
    }

    return transformed_points;
}

// Static method to apply transformation to a nested vector of points
template <typename T>
std::vector<T> Xform::apply(const Matrix& transform, const std::vector<T>& points) {
    std::vector<T> transformed_points;
    transformed_points.reserve(points.size());

    for (const auto& nested_points : points) {
        transformed_points.push_back(apply(transform, nested_points));
    }

    return transformed_points;
}

// Static method to apply transformation to a nested array of points
template <typename T, std::size_t N>
std::array<T, N> Xform::apply(const Matrix& transform, const std::array<T, N>& points) {
    std::array<T, N> transformed_points;

    for (std::size_t i = 0; i < N; ++i) {
        transformed_points[i] = apply(transform, points[i]);
    }

    return transformed_points;
}

// Explicit template instantiation
template std::array<geo::Point, 2> Xform::apply(const Matrix& transform,
                                                const std::array<geo::Point, 2>& points);
template std::array<std::array<geo::Point, 2>, 2> Xform::apply(
    const Matrix& transform, const std::array<std::array<geo::Point, 2>, 2>& points);
template std::vector<std::vector<geo::Point>> Xform::apply(
    const Matrix& transform, const std::vector<std::vector<geo::Point>>& points);

}  // namespace geo