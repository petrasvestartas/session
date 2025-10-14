#include "catch_amalgamated.hpp"
#include "cylinder.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Cylinder JSON roundtrip", "[cylinder]") {
    Line line(0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 8.0f);
    Cylinder original(line, 1.0f);
    original.name = "test_cylinder";
    
    
    encoders::json_dump(original, "test_cylinder.json");
    Cylinder loaded = encoders::json_load<Cylinder>("test_cylinder.json");

    encoders::json_dump(original, "test_cylinder.json");
    
    REQUIRE(loaded.radius == Catch::Approx(original.radius));
    REQUIRE(loaded.name == original.name);}
