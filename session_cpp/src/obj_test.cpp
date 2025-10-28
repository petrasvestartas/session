#include "catch_amalgamated.hpp"
#include "obj.h"
#include "mesh.h"
#include <fstream>

using namespace session_cpp;

TEST_CASE("Read Bunny OBJ File", "[obj]") {
    // Skip test if data file doesn't exist
    std::ifstream test_file("../data/bunny.obj");
    if (!test_file.good()) {
        SKIP("Test data file ../data/bunny.obj not found");
    }
    test_file.close();
    
    Mesh mesh = obj::read_obj("../data/bunny.obj");
    
    // Test vertex and face counts
    REQUIRE(mesh.number_of_vertices() == 2503);
    REQUIRE(mesh.number_of_faces() == 4968);
    
    auto [vertices, faces] = mesh.to_vertices_and_faces();
    REQUIRE(vertices.size() == 2503);
    REQUIRE(faces.size() == 4968);
    
    // Check that vertices are valid (not all zeros)
    bool has_non_zero = false;
    for (const auto& v : vertices) {
        if (v.x() != 0.0 || v.y() != 0.0 || v.z() != 0.0) {
            has_non_zero = true;
            break;
        }
    }
    REQUIRE(has_non_zero);
    
    // Check that faces have at least 3 vertices
    for (const auto& face : faces) {
        REQUIRE(face.size() >= 3);
    }
}

TEST_CASE("Write and Read OBJ Round-Trip", "[obj]") {
    // Create a simple mesh
    Mesh original_mesh;
    auto v0 = original_mesh.add_vertex(Point(0.0, 0.0, 0.0));
    auto v1 = original_mesh.add_vertex(Point(1.0, 0.0, 0.0));
    auto v2 = original_mesh.add_vertex(Point(0.0, 1.0, 0.0));
    auto v3 = original_mesh.add_vertex(Point(0.0, 0.0, 1.0));
    
    original_mesh.add_face({v0, v1, v2});
    original_mesh.add_face({v0, v1, v3});
    
    REQUIRE(original_mesh.number_of_vertices() == 4);
    REQUIRE(original_mesh.number_of_faces() == 2);
    
    // Write to file in system temp directory for better portability
    std::string temp_file = "test_temp_roundtrip.obj";
    obj::write_obj(original_mesh, temp_file);
    
    // Verify file was created and is readable
    std::ifstream check(temp_file);
    REQUIRE(check.good());
    check.close();
    
    // Read back
    Mesh loaded_mesh = obj::read_obj(temp_file);
    
    // Verify counts match
    REQUIRE(loaded_mesh.number_of_vertices() == original_mesh.number_of_vertices());
    REQUIRE(loaded_mesh.number_of_faces() == original_mesh.number_of_faces());
    
    // Clean up
    std::remove(temp_file.c_str());
}
