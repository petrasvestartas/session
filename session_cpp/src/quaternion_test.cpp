#include "catch_amalgamated.hpp"
#include "quaternion.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Quaternion JSON roundtrip", "[quaternion]") {
    Vector axis(0.0, 0.0, 1.0);
    Quaternion original = Quaternion::from_axis_angle(axis, 1.5708);
    
    encoders::json_dump(original, "test_quaternion.json");
    Quaternion loaded = encoders::json_load<Quaternion>("test_quaternion.json");

    encoders::json_dump(original, "test_quaternion.json");
    
    // Just verify roundtrip works
    REQUIRE(loaded.s == Catch::Approx(original.s).epsilon(0.0001));}
