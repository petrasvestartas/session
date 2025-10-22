#include "catch_amalgamated.hpp"
#include "cylinder.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Cylinder JSON roundtrip", "[cylinder]") {
    Line line(0.0, 0.0, 0.0, 0.0, 0.0, 8.0);
    Cylinder original(line, 1.0);
    original.name = "test_cylinder";
    
    
    encoders::json_dump(original, "test_cylinder.json");
    Cylinder loaded = encoders::json_load<Cylinder>("test_cylinder.json");

    encoders::json_dump(original, "test_cylinder.json");
    
    REQUIRE(loaded.radius == Catch::Approx(original.radius));
    REQUIRE(loaded.name == original.name);}
