#include "catch_amalgamated.hpp"
#include "vector.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Vector JSON roundtrip", "[vector]") {
    Vector original(42.1, 84.2, 126.3);
    original.name = "test_vector";
    
    encoders::json_dump(original, "test_vector.json");
    Vector loaded = encoders::json_load<Vector>("test_vector.json");
    
    REQUIRE(loaded.x() == Catch::Approx(original.x()));
    REQUIRE(loaded.y() == Catch::Approx(original.y()));
    REQUIRE(loaded.z() == Catch::Approx(original.z()));
    REQUIRE(loaded.name == original.name);
}
