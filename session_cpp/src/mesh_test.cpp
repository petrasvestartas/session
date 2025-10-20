#include "catch_amalgamated.hpp"
#include "mesh.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Mesh JSON roundtrip", "[mesh]") {
    Mesh original;
    auto v0 = original.add_vertex(Point(0.0, 0.0, 0.0), std::nullopt);
    auto v1 = original.add_vertex(Point(1.0, 0.0, 0.0), std::nullopt);
    auto v2 = original.add_vertex(Point(0.0, 1.0, 0.0), std::nullopt);
    original.add_face({v0, v1, v2}, std::nullopt);
    
    encoders::json_dump(original, "test_mesh.json");
    Mesh loaded = encoders::json_load<Mesh>("test_mesh.json");

    encoders::json_dump(original, "test_mesh.json");
    
    REQUIRE(loaded.number_of_vertices() == original.number_of_vertices());
    REQUIRE(loaded.number_of_faces() == original.number_of_faces());}
