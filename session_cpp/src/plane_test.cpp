#include "catch_amalgamated.hpp"
#include "plane.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Plane JSON roundtrip", "[plane]") {
    Plane original = Plane::xy_plane();
    original.name = "test_plane";
    
    encoders::json_dump(original, "test_plane.json");
    Plane loaded = encoders::json_load<Plane>("test_plane.json");

    encoders::json_dump(original, "test_plane.json");
    
    REQUIRE(loaded.name == original.name);}
