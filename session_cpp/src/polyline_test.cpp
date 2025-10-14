#include "catch_amalgamated.hpp"
#include "polyline.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Polyline JSON roundtrip", "[polyline]") {
    std::vector<Point> points = {Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0)};
    Polyline original(points);
    original.name = "test_polyline";
    
    encoders::json_dump(original, "test_polyline.json");
    Polyline loaded = encoders::json_load<Polyline>("test_polyline.json");

    encoders::json_dump(original, "test_polyline.json");
    
    REQUIRE(loaded.len() == original.len());
    REQUIRE(loaded.name == original.name);}
