#include "core.hpp"

int main() {
    geo::Vector a(1, 0, 0);
    geo::Vector b(1.5, 0, 0);
    geo::Vector r = a + b;
    geo::log(r.to_string());
    geo::Vector f1(-2, 2.5, 0);
    geo::Vector f2(4, 0, 0);
    r = f1 + f2;
    geo::log(r.to_string());
    return 1;
}