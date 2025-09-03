#include <iostream>
#include "src/point.h"

using namespace geo;

int main() {
    std::cout << "=== C++ Point JSON Demo ===" << std::endl;
    
    // Create a point
    Point point(1.5, 2.5, 3.5);
    std::cout << "Created point: " << point << std::endl;
    
    // Show JSON serialization output
    auto json_data = point.to_json_data();
    std::cout << "\nSerialized JSON:" << std::endl;
    std::cout << json_data.dump(2) << std::endl;
    
    // Test deserialization from JSON data
    Point loaded_point = Point::from_json_data(json_data);
    std::cout << "\nDeserialized point: " << loaded_point << std::endl;
    
    // Also save to file
    std::string filename = "point_cpp.json";
    point.to_json(filename);
    std::cout << "\nAlso saved to file: " << filename << std::endl;
    
    std::cout << "JSON serialization demo completed!" << std::endl;
    return 0;
}
