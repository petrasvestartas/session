#include "catch_amalgamated.hpp"
#include "obj.h"
#include "mesh.h"

using namespace session_cpp;

TEST_CASE("Read Bunny OBJ File", "[obj]") {
    Mesh mesh = obj::read_obj("../../data/bunny.obj");
    
    // Test vertex and face counts
    REQUIRE(mesh.number_of_vertices() == 2503);
    REQUIRE(mesh.number_of_faces() == 4968);
    
    auto [vertices, faces] = mesh.to_vertices_and_faces();
    REQUIRE(vertices.size() == 2503);
    REQUIRE(faces.size() == 4968);
    
    // Check that vertices are valid (not all zeros)
    bool has_non_zero = false;
    for (const auto& v : vertices) {
        if (v.x() != 0.0f || v.y() != 0.0f || v.z() != 0.0f) {
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
    auto v0 = original_mesh.add_vertex(Point(0.0f, 0.0f, 0.0f));
    auto v1 = original_mesh.add_vertex(Point(1.0f, 0.0f, 0.0f));
    auto v2 = original_mesh.add_vertex(Point(0.0f, 1.0f, 0.0f));
    auto v3 = original_mesh.add_vertex(Point(0.0f, 0.0f, 1.0f));
    
    original_mesh.add_face({v0, v1, v2});
    original_mesh.add_face({v0, v1, v3});
    
    // Write to file
    std::string temp_file = "../../data/test_temp.obj";
    obj::write_obj(original_mesh, temp_file);
    
    // Read back
    Mesh loaded_mesh = obj::read_obj(temp_file);
    
    // Verify counts match
    REQUIRE(loaded_mesh.number_of_vertices() == original_mesh.number_of_vertices());
    REQUIRE(loaded_mesh.number_of_faces() == original_mesh.number_of_faces());
    
    // Clean up
    std::remove(temp_file.c_str());
}
