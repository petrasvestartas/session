#include <iostream>
#include "fmt/core.h"
#include "src/point.h"

using namespace session_cpp;

int main() {
    fmt::print("=== C++ Point JSON Demo ===\n");
    
    // Create a point
    Point point0(1.5, 2.5, 3.5);  
    Point point1(1.5, 2.5, 3.5);
    fmt::print("Point equality: {}\n", point0 == point1);
    fmt::print("Created point: {}\n", point0);
    
    // Show JSON serialization output
    auto json_data = point0.to_json_data();
    fmt::print("\nSerialized JSON:\n{}\n", json_data.dump(2));
    
    // Test deserialization from JSON data
    Point loaded_point = Point::from_json_data(json_data);
    fmt::print("\nDeserialized point: {}\n", loaded_point);
    
    // Also save to file
    std::string filename = "point_cpp.json";
    point0.to_json(filename);
    fmt::print("\nAlso saved to file: {}\n", filename);
    
    fmt::print("JSON serialization demo completed!\n");
    return 0;
}
