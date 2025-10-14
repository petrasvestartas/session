#include "catch_amalgamated.hpp"
#include "color.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Color JSON roundtrip", "[color]") {
    Color original(100, 150, 200, 255);
    original.name = "test_color";
    
    
    encoders::json_dump(original, "test_color.json");
    Color loaded = encoders::json_load<Color>("test_color.json");

    encoders::json_dump(original, "test_color.json");
    
    REQUIRE(loaded.r == original.r);
    REQUIRE(loaded.g == original.g);
    REQUIRE(loaded.b == original.b);
    REQUIRE(loaded.a == original.a);
    REQUIRE(loaded.name == original.name);}
