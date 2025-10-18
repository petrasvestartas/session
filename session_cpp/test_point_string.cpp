#include "src/point.h"
#include <iostream>

int main() {
    using namespace session_cpp;
    
    Point p(500.0, 338.9, 484.0);
    
    std::cout << "Using to_string():\n";
    std::cout << p.to_string() << "\n\n";
    
    std::cout << "Using manual format:\n";
    printf("(%.2f, %.2f, %.2f)\n", p.x(), p.y(), p.z());
    
    return 0;
}
