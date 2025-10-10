#include "arrow.h"
#include "line.h"
#include <iostream>
#include <iomanip>

int main() {
    using namespace session_cpp;
    
    Line line(0.0f, 0.0f, 0.0f, 10.0f, 0.0f, 10.0f);
    Arrow arrow(line, 1.0f);
    
    auto [vertices, faces] = arrow.mesh.to_vertices_and_faces();
    
    std::cout << "C++ - Vertices: " << vertices.size() << ", Faces: " << faces.size() << std::endl;
    std::cout << std::fixed << std::setprecision(6);
    std::cout << "First vertex: " << vertices[0].x() << " " << vertices[0].y() << " " << vertices[0].z() << std::endl;
    std::cout << "Tip vertex (index 20): " << vertices[20].x() << " " << vertices[20].y() << " " << vertices[20].z() << std::endl;
    std::cout << "Cone base vertex (index 21): " << vertices[21].x() << " " << vertices[21].y() << " " << vertices[21].z() << std::endl;
    
    return 0;
}
