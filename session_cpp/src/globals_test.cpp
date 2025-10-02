#include "catch/include/catch_amalgamated.hpp"
#include "globals.h"

using namespace geo;

TEST_CASE("test_globals_initial_values") {
    REQUIRE(GLOBALS::SCALE == 1e6);
    REQUIRE(GLOBALS::PI == 3.14159265358979323846);
    REQUIRE(GLOBALS::ANGLE == 0.11);
    REQUIRE(GLOBALS::TOLERANCE == 1e-3);
}

TEST_CASE("test_globals_modification") {
    double original_scale = GLOBALS::SCALE;
    GLOBALS::SCALE = 2000.0;
    REQUIRE(GLOBALS::SCALE == 2000.0);
    GLOBALS::SCALE = original_scale;
}
