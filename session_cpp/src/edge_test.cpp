#include "catch_amalgamated.hpp"
#include "edge.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Edge JSON roundtrip", "[edge]") {
    Edge original("test_edge", "v0", "v1");
    
    
    encoders::json_dump(original, "test_edge.json");
    Edge loaded = encoders::json_load<Edge>("test_edge.json");

    encoders::json_dump(original, "test_edge.json");
    
    REQUIRE(loaded.name == original.name);
    REQUIRE(loaded.v0 == original.v0);
    REQUIRE(loaded.v1 == original.v1);}
