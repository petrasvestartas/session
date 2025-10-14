#include "catch_amalgamated.hpp"
#include "vertex.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Vertex JSON roundtrip", "[vertex]") {
    Vertex original("v0", "test_attribute");
    
    encoders::json_dump(original, "test_vertex.json");
    Vertex loaded = encoders::json_load<Vertex>("test_vertex.json");
    
    REQUIRE(loaded.name == original.name);
    REQUIRE(loaded.attribute == original.attribute);
}
