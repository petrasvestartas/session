#include "core.hpp"

int main() {

    // Position vector is subtraction of vector coordinates.
    geo::Vector a(1, 1, 3);
    geo::Vector b(3, 3, 9);
    geo::Vector r = b-a;
    geo::log(r.to_string());

    // If the line of action goes through points a and b.
    // Then this force can be expressed by the unit vector multiplied by the magnitude.
    double force = 10;
    geo::Vector f = force * r.unitized();
    geo::log(f.to_string());
    f = force * (r / r.length());
    geo::log(f.to_string());
    f = r.rescaled(force);
    geo::log(f.to_string());

    return 1;
}