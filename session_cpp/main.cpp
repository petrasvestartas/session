#include "src/session.h"
#include <iostream>

using namespace session_cpp;

int main() {
    Session session("demo");
    
    // Add geometry with some overlaps, some separated
    session.add_point(std::make_shared<Point>(0, 0, 0));                    // Point 1
    session.add_point(std::make_shared<Point>(0.0005, 0, 0));               // Point 2 - collides with Point 1
    session.add_line(std::make_shared<Line>(0, 0, 0, 0.1, 0.1, 0.1));       // Line 1 - collides with both points
    session.add_line(std::make_shared<Line>(5, 5, 5, 5.1, 5.1, 5.1));       // Line 2 - far away
    session.add_bbox(std::make_shared<BoundingBox>(
        Point(10, 10, 10), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(0.5, 0.5, 0.5)));  // Box - far away
    
    // Detect collisions
    auto collisions = session.get_collisions();
    std::cout << "Objects: " << session.lookup.size() << ", Collisions: " << collisions.size() << std::endl;
    
    // Print graph edges
    std::cout << "\nGraph edges:" << std::endl;
    for (const auto& [node, edges] : session.graph.edges) {
        for (const auto& [neighbor, edge] : edges) {
            std::cout << "  " << node.substr(0, 8) << "... -> " << neighbor.substr(0, 8) 
                      << "... [" << edge.attribute << "]" << std::endl;
        }
    }
    
    return 0;
}
