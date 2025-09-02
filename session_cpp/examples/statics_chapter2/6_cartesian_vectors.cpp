#include "core.hpp"

int main() {

    // Get a length of a vector:
    geo::Vector f(1, 2, 3);
    double length0 = sqrt(pow(f[0],2)+pow(f[1],2)+pow(f[2],2));
    double length1 = f.length();
    geo::log(std::to_string(length0));
    geo::log(std::to_string(length1));

    // Sum of vector components is a vector:
    geo::Vector fx = f.xc();
    geo::Vector fy = f.yc();
    geo::Vector fz = f.zc();
    geo::Vector sum_fx = fx + fy + fz;
    geo::log(sum_fx.to_string());

    // Unit vector is formed by dividing vector by its length:
    geo::Vector u0 = f / f.length();
    geo::Vector u1 = (fx/f.length()) + (fy/f.length()) + (fz/f.length());
    geo::Vector u2 = f.unitized();
    geo::log(u0.to_string());
    geo::log(u1.to_string());
    geo::log(u2.to_string());

    // Coordinate angles are angles between vector and coordinate axes:
    std::array<double, 3> coordinate_angles = f.coordinate_direction_3angles();

    // These cosine angles forms a unit vector as components
    geo::Vector u3 (cos(coordinate_angles[0]), cos(coordinate_angles[1]), cos(coordinate_angles[2]));
    geo::log(u3.to_string());

    // The sum of square cosines of these angles is 1
    double sum_cosines = pow(cos(coordinate_angles[0]),2) + pow(cos(coordinate_angles[1]),2) + pow(cos(coordinate_angles[2]),2);
    geo::log(std::to_string(sum_cosines));

    return 1;
}