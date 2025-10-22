#include "xform.h"
#include "point.h"

namespace session_cpp {

Xform::Xform() {
    m = {0.0, 0.0, 0.0, 0.0,
         0.0, 0.0, 0.0, 0.0,
         0.0, 0.0, 0.0, 0.0,
         0.0, 0.0, 0.0, 0.0};
    m[0] = 1.0;
    m[5] = 1.0;
    m[10] = 1.0;
    m[15] = 1.0;
}

Xform::Xform(const std::array<double, 16>& matrix) : m(matrix) {}

Xform Xform::identity() {
    return Xform();
}

Xform Xform::from_matrix(const std::array<double, 16>& matrix) {
    return Xform(matrix);
}

Xform Xform::translation(double x, double y, double z) {
    Xform xform;
    xform.m[12] = x;
    xform.m[13] = y;
    xform.m[14] = z;
    return xform;
}

Xform Xform::scaling(double x, double y, double z) {
    Xform xform;
    xform.m[0] = x;
    xform.m[5] = y;
    xform.m[10] = z;
    return xform;
}

Xform Xform::rotation_x(double angle_radians) {
    Xform xform;
    double cos_angle = std::cos(angle_radians);
    double sin_angle = std::sin(angle_radians);
    xform.m[5] = cos_angle;
    xform.m[6] = sin_angle;
    xform.m[9] = -sin_angle;
    xform.m[10] = cos_angle;
    return xform;
}

Xform Xform::rotation_y(double angle_radians) {
    Xform xform;
    double cos_angle = std::cos(angle_radians);
    double sin_angle = std::sin(angle_radians);
    xform.m[0] = cos_angle;
    xform.m[2] = -sin_angle;
    xform.m[8] = sin_angle;
    xform.m[10] = cos_angle;
    return xform;
}

Xform Xform::rotation_z(double angle_radians) {
    Xform xform;
    double cos_angle = std::cos(angle_radians);
    double sin_angle = std::sin(angle_radians);
    xform.m[0] = cos_angle;
    xform.m[1] = sin_angle;
    xform.m[4] = -sin_angle;
    xform.m[5] = cos_angle;
    return xform;
}

Xform Xform::rotation(Vector& axis, double angle_radians) {
    Xform xform;
    axis.normalize_self();
    
    double cos_angle = std::cos(angle_radians);
    double sin_angle = std::sin(angle_radians);
    double one_minus_cos = 1.0 - cos_angle;

    double xx = axis.x() * axis.x();
    double xy = axis.x() * axis.y();
    double xz = axis.x() * axis.z();
    double yy = axis.y() * axis.y();
    double yz = axis.y() * axis.z();
    double zz = axis.z() * axis.z();

    xform.m[0] = cos_angle + xx * one_minus_cos;
    xform.m[1] = xy * one_minus_cos + axis.z() * sin_angle;
    xform.m[2] = xz * one_minus_cos - axis.y() * sin_angle;

    xform.m[4] = xy * one_minus_cos - axis.z() * sin_angle;
    xform.m[5] = cos_angle + yy * one_minus_cos;
    xform.m[6] = yz * one_minus_cos + axis.x() * sin_angle;

    xform.m[8] = xz * one_minus_cos + axis.y() * sin_angle;
    xform.m[9] = yz * one_minus_cos - axis.x() * sin_angle;
    xform.m[10] = cos_angle + zz * one_minus_cos;

    return xform;
}

Xform Xform::change_basis(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis) {
    Xform xform;

    x_axis.normalize_self();
    y_axis.normalize_self();
    z_axis.normalize_self();

    xform.m[0] = x_axis.x();
    xform.m[1] = x_axis.y();
    xform.m[2] = x_axis.z();

    xform.m[4] = y_axis.x();
    xform.m[5] = y_axis.y();
    xform.m[6] = y_axis.z();

    xform.m[8] = z_axis.x();
    xform.m[9] = z_axis.y();
    xform.m[10] = z_axis.z();

    xform.m[12] = origin.x();
    xform.m[13] = origin.y();
    xform.m[14] = origin.z();

    return xform;
}

Xform Xform::change_basis_alt(Point& origin_1, Vector& x_axis_1, Vector& y_axis_1, Vector& z_axis_1,
                                Point& origin_0, Vector& x_axis_0, Vector& y_axis_0, Vector& z_axis_0) {
    double a = x_axis_1.dot(y_axis_1);
    double b = x_axis_1.dot(z_axis_1);
    double c = y_axis_1.dot(z_axis_1);

    double r[3][6] = {
        {x_axis_1.dot(x_axis_1), a, b, x_axis_1.dot(x_axis_0), x_axis_1.dot(y_axis_0), x_axis_1.dot(z_axis_0)},
        {a, y_axis_1.dot(y_axis_1), c, y_axis_1.dot(x_axis_0), y_axis_1.dot(y_axis_0), y_axis_1.dot(z_axis_0)},
        {b, c, z_axis_1.dot(z_axis_1), z_axis_1.dot(x_axis_0), z_axis_1.dot(y_axis_0), z_axis_1.dot(z_axis_0)}
    };

    int i0 = (r[0][0] >= r[1][1]) ? 0 : 1;
    if (r[2][2] > r[i0][i0]) i0 = 2;
    int i1 = (i0 + 1) % 3;
    int i2 = (i1 + 1) % 3;

    if (r[i0][i0] == 0.0) return Xform::identity();

    double d = 1.0 / r[i0][i0];
    for (int j = 0; j < 6; j++) r[i0][j] *= d;
    r[i0][i0] = 1.0;

    if (r[i1][i0] != 0.0) {
        d = -r[i1][i0];
        for (int j = 0; j < 6; j++) r[i1][j] += d * r[i0][j];
        r[i1][i0] = 0.0;
    }
    if (r[i2][i0] != 0.0) {
        d = -r[i2][i0];
        for (int j = 0; j < 6; j++) r[i2][j] += d * r[i0][j];
        r[i2][i0] = 0.0;
    }

    if (std::abs(r[i1][i1]) < std::abs(r[i2][i2])) {
        std::swap(i1, i2);
    }
    if (r[i1][i1] == 0.0) return Xform::identity();

    d = 1.0 / r[i1][i1];
    for (int j = 0; j < 6; j++) r[i1][j] *= d;
    r[i1][i1] = 1.0;

    if (r[i0][i1] != 0.0) {
        d = -r[i0][i1];
        for (int j = 0; j < 6; j++) r[i0][j] += d * r[i1][j];
        r[i0][i1] = 0.0;
    }
    if (r[i2][i1] != 0.0) {
        d = -r[i2][i1];
        for (int j = 0; j < 6; j++) r[i2][j] += d * r[i1][j];
        r[i2][i1] = 0.0;
    }

    if (r[i2][i2] == 0.0) return Xform::identity();

    d = 1.0 / r[i2][i2];
    for (int j = 0; j < 6; j++) r[i2][j] *= d;
    r[i2][i2] = 1.0;

    if (r[i0][i2] != 0.0) {
        d = -r[i0][i2];
        for (int j = 0; j < 6; j++) r[i0][j] += d * r[i2][j];
        r[i0][i2] = 0.0;
    }
    if (r[i1][i2] != 0.0) {
        d = -r[i1][i2];
        for (int j = 0; j < 6; j++) r[i1][j] += d * r[i2][j];
        r[i1][i2] = 0.0;
    }

    Xform m_xform;
    m_xform.m[0] = static_cast<double>(r[0][3]);
    m_xform.m[4] = static_cast<double>(r[0][4]);
    m_xform.m[8] = static_cast<double>(r[0][5]);
    m_xform.m[1] = static_cast<double>(r[1][3]);
    m_xform.m[5] = static_cast<double>(r[1][4]);
    m_xform.m[9] = static_cast<double>(r[1][5]);
    m_xform.m[2] = static_cast<double>(r[2][3]);
    m_xform.m[6] = static_cast<double>(r[2][4]);
    m_xform.m[10] = static_cast<double>(r[2][5]);

    Xform t0 = translation(-origin_1.x(), -origin_1.y(), -origin_1.z());
    Xform t2 = translation(origin_0.x(), origin_0.y(), origin_0.z());
    return t2 * (m_xform * t0);
}

Xform Xform::plane_to_plane(Point& origin_0, Vector& x_axis_0, Vector& y_axis_0, Vector& z_axis_0,
                             Point& origin_1, Vector& x_axis_1, Vector& y_axis_1, Vector& z_axis_1) {
    Vector x0 = x_axis_0, y0 = y_axis_0, z0 = z_axis_0;
    Vector x1 = x_axis_1, y1 = y_axis_1, z1 = z_axis_1;
    x0.normalize_self(); y0.normalize_self(); z0.normalize_self();
    x1.normalize_self(); y1.normalize_self(); z1.normalize_self();

    Xform t0 = translation(-origin_0.x(), -origin_0.y(), -origin_0.z());

    Xform f0;
    f0.m[0] = x0.x(); f0.m[1] = x0.y(); f0.m[2] = x0.z();
    f0.m[4] = y0.x(); f0.m[5] = y0.y(); f0.m[6] = y0.z();
    f0.m[8] = z0.x(); f0.m[9] = z0.y(); f0.m[10] = z0.z();

    Xform f1;
    f1.m[0] = x1.x(); f1.m[4] = x1.y(); f1.m[8] = x1.z();
    f1.m[1] = y1.x(); f1.m[5] = y1.y(); f1.m[9] = y1.z();
    f1.m[2] = z1.x(); f1.m[6] = z1.y(); f1.m[10] = z1.z();

    Xform r = f1 * f0;
    Xform t1 = translation(origin_1.x(), origin_1.y(), origin_1.z());
    return t1 * (r * t0);
}

Xform Xform::plane_to_xy(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis) {
    Vector x = x_axis, y = y_axis, z = z_axis;
    x.normalize_self(); y.normalize_self(); z.normalize_self();

    Xform t = translation(-origin.x(), -origin.y(), -origin.z());
    Xform f;
    f.m[0] = x.x(); f.m[1] = x.y(); f.m[2] = x.z();
    f.m[4] = y.x(); f.m[5] = y.y(); f.m[6] = y.z();
    f.m[8] = z.x(); f.m[9] = z.y(); f.m[10] = z.z();
    return f * t;
}

Xform Xform::xy_to_plane(Point& origin, Vector& x_axis, Vector& y_axis, Vector& z_axis) {
    Vector x = x_axis, y = y_axis, z = z_axis;
    x.normalize_self(); y.normalize_self(); z.normalize_self();

    Xform f;
    f.m[0] = x.x(); f.m[4] = y.x(); f.m[8] = z.x();
    f.m[1] = x.y(); f.m[5] = y.y(); f.m[9] = z.y();
    f.m[2] = x.z(); f.m[6] = y.z(); f.m[10] = z.z();

    Xform t = translation(origin.x(), origin.y(), origin.z());
    return t * f;
}

Xform Xform::scale_xyz(double scale_x, double scale_y, double scale_z) {
    Xform xform;
    xform.m[0] = scale_x;
    xform.m[5] = scale_y;
    xform.m[10] = scale_z;
    return xform;
}

Xform Xform::scale_uniform(Point& origin, double scale_value) {
    Xform t0 = translation(-origin.x(), -origin.y(), -origin.z());
    Xform t1 = scaling(scale_value, scale_value, scale_value);
    Xform t2 = translation(origin.x(), origin.y(), origin.z());
    return t2 * (t1 * t0);
}

Xform Xform::scale_non_uniform(Point& origin, double scale_x, double scale_y, double scale_z) {
    Xform t0 = translation(-origin.x(), -origin.y(), -origin.z());
    Xform t1 = scale_xyz(scale_x, scale_y, scale_z);
    Xform t2 = translation(origin.x(), origin.y(), origin.z());
    return t2 * (t1 * t0);
}

Xform Xform::axis_rotation(double angle, Vector& axis) {
    double c = std::cos(angle);
    double s = std::sin(angle);
    double ux = axis.x();
    double uy = axis.y();
    double uz = axis.z();
    double t = 1.0 - c;

    Xform xform;
    xform.m[0] = t * ux * ux + c;
    xform.m[4] = t * ux * uy - uz * s;
    xform.m[8] = t * ux * uz + uy * s;

    xform.m[1] = t * ux * uy + uz * s;
    xform.m[5] = t * uy * uy + c;
    xform.m[9] = t * uy * uz - ux * s;

    xform.m[2] = t * ux * uz - uy * s;
    xform.m[6] = t * uy * uz + ux * s;
    xform.m[10] = t * uz * uz + c;

    return xform;
}

Xform Xform::look_at_rh(const Point& eye, const Point& target, const Vector& up) {
    Vector f = target - eye;
    f.normalize_self();
    Vector up_copy = up;
    up_copy.normalize_self();
    Vector s = f.cross(up_copy);
    s.normalize_self();
    Vector u = s.cross(f);
    
    Xform xform;
    xform.m[0] = s.x();
    xform.m[4] = s.y();
    xform.m[8] = s.z();
    
    xform.m[1] = u.x();
    xform.m[5] = u.y();
    xform.m[9] = u.z();
    
    xform.m[2] = -f.x();
    xform.m[6] = -f.y();
    xform.m[10] = -f.z();
    
    Vector eye_vec(eye.x(), eye.y(), eye.z());
    xform.m[12] = -s.dot(eye_vec);
    xform.m[13] = -u.dot(eye_vec);
    xform.m[14] = f.dot(eye_vec);
    
    return xform;
}

std::optional<Xform> Xform::inverse() const {
    double a00 = m[0], a01 = m[4], a02 = m[8];
    double a10 = m[1], a11 = m[5], a12 = m[9];
    double a20 = m[2], a21 = m[6], a22 = m[10];

    double det = a00 * (a11 * a22 - a12 * a21) 
              - a01 * (a10 * a22 - a12 * a20)
              + a02 * (a10 * a21 - a11 * a20);
    
    if (std::abs(det) < 1e-12) {
        return std::nullopt;
    }

    double inv_det = 1.0 / det;

    double m00 = (a11 * a22 - a12 * a21) * inv_det;
    double m01 = (a02 * a21 - a01 * a22) * inv_det;
    double m02 = (a01 * a12 - a02 * a11) * inv_det;
    double m10 = (a12 * a20 - a10 * a22) * inv_det;
    double m11 = (a00 * a22 - a02 * a20) * inv_det;
    double m12 = (a02 * a10 - a00 * a12) * inv_det;
    double m20 = (a10 * a21 - a11 * a20) * inv_det;
    double m21 = (a01 * a20 - a00 * a21) * inv_det;
    double m22 = (a00 * a11 - a01 * a10) * inv_det;

    double tx = m[12], ty = m[13], tz = m[14];
    double itx = -(m00 * tx + m01 * ty + m02 * tz);
    double ity = -(m10 * tx + m11 * ty + m12 * tz);
    double itz = -(m20 * tx + m21 * ty + m22 * tz);

    Xform res;
    res.guid = "";
    res.name = "";
    res.m[0] = m00;  res.m[4] = m01;  res.m[8] = m02;   res.m[12] = itx;
    res.m[1] = m10;  res.m[5] = m11;  res.m[9] = m12;   res.m[13] = ity;
    res.m[2] = m20;  res.m[6] = m21;  res.m[10] = m22;  res.m[14] = itz;
    
    return res;
}

bool Xform::is_identity() const {
    Xform identity;
    for (int i = 0; i < 16; i++) {
        if (std::abs(m[i] - identity.m[i]) > 1e-10) {
            return false;
        }
    }
    return true;
}

Point Xform::transformed_point(const Point& point) const {
    double x = point.x();
    double y = point.y();
    double z = point.z();
    double w = m[3] * x + m[7] * y + m[11] * z + m[15];
    double w_inv = (std::abs(w) > 1e-10) ? 1.0 / w : 1.0;

    return Point(
        (m[0] * x + m[4] * y + m[8] * z + m[12]) * w_inv,
        (m[1] * x + m[5] * y + m[9] * z + m[13]) * w_inv,
        (m[2] * x + m[6] * y + m[10] * z + m[14]) * w_inv
    );
}

Vector Xform::transformed_vector(const Vector& vector) const {
    double x = vector.x();
    double y = vector.y();
    double z = vector.z();

    return Vector(
        m[0] * x + m[4] * y + m[8] * z,
        m[1] * x + m[5] * y + m[9] * z,
        m[2] * x + m[6] * y + m[10] * z
    );
}

void Xform::transform_point(Point& point) const {
    double x = point[0];
    double y = point[1];
    double z = point[2];
    double w = m[3] * x + m[7] * y + m[11] * z + m[15];
    double w_inv = (std::abs(w) > 1e-10) ? 1.0 / w : 1.0;

    point[0] = (m[0] * x + m[4] * y + m[8] * z + m[12]) * w_inv;
    point[1] = (m[1] * x + m[5] * y + m[9] * z + m[13]) * w_inv;
    point[2] = (m[2] * x + m[6] * y + m[10] * z + m[14]) * w_inv;
}

void Xform::transform_vector(Vector& vector) const {
    double x = vector[0];
    double y = vector[1];
    double z = vector[2];

    vector[0] = m[0] * x + m[4] * y + m[8] * z;
    vector[1] = m[1] * x + m[5] * y + m[9] * z;
    vector[2] = m[2] * x + m[6] * y + m[10] * z;
}

nlohmann::ordered_json Xform::jsondump() const {
    return nlohmann::ordered_json{
        {"type", "Xform"},
        {"guid", guid},
        {"name", name},
        {"m", m}
    };
}

Xform Xform::jsonload(const nlohmann::json& data) {
    Xform xform;
    xform.guid = data["guid"].get<std::string>();
    xform.name = data["name"].get<std::string>();
    xform.m = data["m"].get<std::array<double, 16>>();
    return xform;
}

Xform Xform::operator*(const Xform& other) const {
    Xform result;
    result.m = {0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0};

    for (int i = 0; i < 4; i++) {
        for (int j = 0; j < 4; j++) {
            double sum = 0.0;
            for (int k = 0; k < 4; k++) {
                sum += m[k * 4 + i] * other.m[j * 4 + k];
            }
            result.m[j * 4 + i] = sum;
        }
    }

    return result;
}

Xform& Xform::operator*=(const Xform& other) {
    *this = *this * other;
    return *this;
}

double& Xform::operator()(int row, int col) {
    if (row < 0 || row >= 4 || col < 0 || col >= 4) {
        throw std::out_of_range("Index out of bounds: (" + std::to_string(row) + ", " + std::to_string(col) + ")");
    }
    return m[col * 4 + row];
}

const double& Xform::operator()(int row, int col) const {
    if (row < 0 || row >= 4 || col < 0 || col >= 4) {
        throw std::out_of_range("Index out of bounds: (" + std::to_string(row) + ", " + std::to_string(col) + ")");
    }
    return m[col * 4 + row];
}

} // namespace session_cpp
