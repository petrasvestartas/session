#include "catch_amalgamated.hpp"
#include "arrow.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Arrow JSON roundtrip", "[arrow]") {
    Line line(0.0, 0.0, 0.0, 0.0, 0.0, 8.0);
    Arrow original(line, 1.0);
    original.name = "test_arrow";
    
    
    encoders::json_dump(original, "test_arrow.json");
    Arrow loaded = encoders::json_load<Arrow>("test_arrow.json");

    encoders::json_dump(original, "test_arrow.json");
    
    REQUIRE(loaded.radius == original.radius);
    REQUIRE(loaded.name == original.name);
    REQUIRE(loaded.mesh.number_of_vertices() == 29);
    REQUIRE(loaded.mesh.number_of_faces() == 28);}
