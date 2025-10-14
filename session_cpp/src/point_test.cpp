#include "catch_amalgamated.hpp"
#include "point.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Point JSON roundtrip", "[point]") {
    Point original(42.1, 84.2, 126.3);
    original.name = "test_point";
    original.width = 3.0;
    
    
    encoders::json_dump(original, "test_point.json");
    Point loaded = encoders::json_load<Point>("test_point.json");

    encoders::json_dump(original, "test_point.json");
    
    REQUIRE(loaded.x() == original.x());
    REQUIRE(loaded.y() == original.y());
    REQUIRE(loaded.z() == original.z());
    REQUIRE(loaded.name == original.name);
    REQUIRE(loaded.width == original.width);}
