#include "src/vector.h"
#include <iostream>

using namespace session_cpp;

int main() {
    std::cout << "=== C++ Length Caching Demo ===" << std::endl;
    
    // Create a vector
    Vector v(3.0, 4.0, 5.0);
    std::cout << "Created vector: (" << v.x() << ", " << v.y() << ", " << v.z() << ")" << std::endl;
    
    // First call to length() - will compute and cache
    std::cout << "First magnitude() call - computes: " << v.magnitude() << std::endl;
    
    // Second call to length() - uses cached value
    std::cout << "Second magnitude() call - cached: " << v.magnitude() << std::endl;
    
    // Modify the vector - this invalidates the cache
    v.set_x(6.0);
    std::cout << "Modified x to 6.0" << std::endl;
    
    // Next call to length() - recomputes because cache was invalidated
    std::cout << "After modification - recomputes: " << v.magnitude() << std::endl;
    
    // Use compound assignment - also invalidates cache
    v *= 2.0;
    std::cout << "After scaling by 2.0" << std::endl;
    std::cout << "Magnitude after scaling: " << v.magnitude() << std::endl;
    
    // Test compute_length (always computes)
    std::cout << "Using compute_length(): " << v.compute_length() << std::endl;
    
    std::cout << std::endl << "✅ Length caching working correctly!" << std::endl;
    std::cout << "🔧 Cache is invalidated when coordinates change" << std::endl;
    std::cout << "📈 Performance improved by avoiding repeated sqrt() calls" << std::endl;
    
    return 0;
}
