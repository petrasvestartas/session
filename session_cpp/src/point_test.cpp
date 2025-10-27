#include "catch_amalgamated.hpp"
#include "point.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Point JSON roundtrip", "[point]") {
    Point original(42.1, 84.2, 126.3);
    original.name = "test_point";
    original.width = 3.0;
    
    
    encoders::json_dump(original, "test_point.json");
    Point loaded = encoders::json_load<Point>("test_point.json");

    encoders::json_dump(original, "test_point.json");
    
    REQUIRE(std::abs(loaded.x()-original.x()) < 0.0001);
    REQUIRE(std::abs(loaded.y()-original.y()) < 0.0001);
    REQUIRE(std::abs(loaded.z()-original.z()) < 0.0001);
    REQUIRE(loaded.name == original.name);
    REQUIRE(loaded.width == original.width);
}
