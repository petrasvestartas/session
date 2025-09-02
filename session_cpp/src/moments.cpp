#include "moments.hpp"

namespace geo {

std::array<int, 2> get_moment_sign(const Point& p, const Vector& f, bool cw_is_positive) {
    // Split the point and vector into x and y components
    double x = p[0];
    double y = p[1];
    double fx = f[0];
    double fy = f[1];

    // Initialize the result array
    std::array<int, 2> result = {0, 0};

    // horizontal component
    if ((y > 0 && fx > 0) || (y < 0 && fx < 0))
        result[0] = 1;
    else if ((y > 0 && fx < 0) || (y < 0 && fx > 0))
        result[0] = -1;

    // vertical component
    if ((x > 0 && fy < 0) || (x < 0 && fy > 0))
        result[1] = 1;
    else if ((x < 0 && fy < 0) || (x > 0 && fy > 0))
        result[1] = -1;

    if (cw_is_positive == false) {
        result[0] *= -1;
        result[1] *= -1;
    }
    return result;
}

std::array<double, 2> get_moment(const Point& p, const Vector& f) {
    // Get the moment sign
    std::array<int, 2> sign = get_moment_sign(p, f);

    // Split the point and vector into x and y components
    double x = p[0];   // m
    double y = p[1];   // m
    double fx = f[0];  // N
    double fy = f[1];  // N

    double Mfy = fy * x * sign[0];  // Nm
    double Mfx = fx * y * sign[1];  // Nm

    std::array<double, 2> result = {Mfx, Mfy};
    return result;
}

std::array<double, 2> get_moment_with_eccentricity(const Point& p, const Vector& f, const double& e,
                                                   const double& deg) {
    std::array<double, 2> result;
    return result;
}

double global_equilibrium_of_frame(const Vector& F0x, const double& e0, const double& en) {
    return 0;
}

double local_equilibrium_of_block(const Point& p0, const double& e0, const Vector& F0x,
                                  const Vector& F0y, const double& deg0, const Point& pw,
                                  const Vector& W, const Point& p1, const double& deg1) {
    return 0;
}

}  // namespace geo