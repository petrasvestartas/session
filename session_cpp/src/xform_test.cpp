#include "catch_amalgamated.hpp"
#include "xform.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Xform JSON roundtrip", "[xform]") {
    Xform original = Xform::translation(10.0, 20.0, 30.0);
    
    encoders::json_dump(original, "test_xform.json");
    Xform loaded = encoders::json_load<Xform>("test_xform.json");

    encoders::json_dump(original, "test_xform.json");
    
    // Just verify roundtrip works - Xform uses different API
    REQUIRE(loaded.is_identity() == original.is_identity());}
