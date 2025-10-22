#include "catch_amalgamated.hpp"
#include "line.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Line JSON roundtrip", "[line]") {
    Line original(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    original.name = "test_line";
    
    
    encoders::json_dump(original, "test_line.json");
    Line loaded = encoders::json_load<Line>("test_line.json");

    encoders::json_dump(original, "test_line.json");
    
    REQUIRE(loaded.x0() == Catch::Approx(original.x0()));
    REQUIRE(loaded.y0() == Catch::Approx(original.y0()));
    REQUIRE(loaded.z0() == Catch::Approx(original.z0()));
    REQUIRE(loaded.x1() == Catch::Approx(original.x1()));
    REQUIRE(loaded.y1() == Catch::Approx(original.y1()));
    REQUIRE(loaded.z1() == Catch::Approx(original.z1()));
    REQUIRE(loaded.name == original.name);}
