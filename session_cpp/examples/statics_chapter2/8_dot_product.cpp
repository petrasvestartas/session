#include "core.hpp"

int main() {

    // Get a dot product of two vectors.
    geo::Vector a(1*10, sqrt(3)*10, 0);
    geo::Vector b(4, 0, 0);
    double dot_product = a.dot(b);

    // Dot product is the same as multiplication of vector lengths and cosine of the angle between them.
    double length_a = a.length();
    double length_b = b.length();
    double angle = 60;
    double dot_product2 = length_a * length_b * cos(angle * geo::GLOBALS::TO_RADIANS);

    geo::log(std::to_string(dot_product));
    geo::log(std::to_string(dot_product2));

    // Dot product is a sum of vector coordinates multiplication.
    double dot_product3 = a[0]*b[0] + a[1]*b[1] + a[2]*b[2];
    geo::log(std::to_string(dot_product3));

    // You can get angle from the dot product using arc-cos.
    double angle0 = acos(dot_product / (a.length()*b.length())) * geo::GLOBALS::TO_DEGREES;
    double angle1 = a.angle(b, false);
    geo::log(std::to_string(angle0));
    geo::log(std::to_string(angle1));

    // You can get a projection of one vector to another.
    geo::Vector u = b.unitized();
    geo::Vector a_projected = a.length() * cos(60* geo::GLOBALS::TO_RADIANS) * u;
    geo::log(a_projected.to_string());
    a_projected = a.dot(u) * u;
    geo::log(a_projected.to_string());
    a_projected = a.projection(b);
    geo::log(a_projected.to_string());

    return 1;
}