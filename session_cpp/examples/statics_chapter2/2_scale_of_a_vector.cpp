#include "core.hpp"

int main() {
    geo::Vector v(1, 1, 1);
    geo::log(v.to_string());
    v.rescale(2);
    geo::log(v.to_string());
    v.rescale(-1.5); // unitize and scale the vector
    geo::log(v.to_string());
    v.rescale(-0.5);
    return 1;
}