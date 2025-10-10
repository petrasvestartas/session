#include "arrow.h"
#include "line.h"
#include <iostream>
#include <iomanip>

int main() {
    using namespace session_cpp;
    
    Line line(0.0f, 0.0f, 0.0f, 10.0f, 0.0f, 10.0f);
    Arrow arrow(line, 1.0f);
    
    std::cout << "=== Cylinder/Pipe Generation Example ===\n" << std::endl;
    
    auto [v_vertices, v_faces] = arrow.mesh.to_vertices_and_faces();
    
    for (const auto& vertex : v_vertices) {
        std::cout << vertex.x() << " " << vertex.y() << " " << vertex.z() << std::endl;
    }
    
    std::cout << "Faces:" << std::endl;
    for (const auto& face : v_faces) {
        std::cout << face[0] << " " << face[1] << " " << face[2] << std::endl;
    }
    
    return 0;
}
