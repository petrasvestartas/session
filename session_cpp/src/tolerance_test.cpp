#include "tolerance.h"
#include "point.h"
#include <catch_amalgamated.hpp>
#include <sstream>

using namespace session_cpp;

TEST_CASE("Tolerance default tolerance", "[tolerance]") {
    REQUIRE(TOL.precision() == Tolerance::PRECISION);
    REQUIRE(TOL.precision() == 3);
}

TEST_CASE("Tolerance format number", "[tolerance]") {
    REQUIRE(TOL.format_number(0, 3) == "0.000");
    REQUIRE(TOL.format_number(0.5, 3) == "0.500");
    REQUIRE(TOL.format_number(0.0, 3) == "0.000");
}

TEST_CASE("Tolerance format number with default precision", "[tolerance]") {
    REQUIRE(TOL.format_number(0) == "0.000");
    REQUIRE(TOL.format_number(0.5) == "0.500");
    REQUIRE(TOL.format_number(0.0) == "0.000");
}

TEST_CASE("Tolerance format point", "[tolerance]") {
    Point point(0, 0, 0);
    std::ostringstream oss;
    oss << point;
    REQUIRE(oss.str() == "Point(x=0.000, y=0.000, z=0.000)");
}

TEST_CASE("Tolerance change values", "[tolerance]") {
    // Create a mutable tolerance instance
    Tolerance tol("M");

    // Test default values
    REQUIRE(tol.precision() == Tolerance::PRECISION);
    REQUIRE(tol.absolute() == Tolerance::ABSOLUTE);

    // Change precision and test formatting
    tol.set_precision(2);
    REQUIRE(tol.precision() == 2);
    REQUIRE(tol.format_number(1.23456) == "1.23");

    // Change absolute tolerance and test zero checking
    tol.set_absolute(1e-5);
    REQUIRE(tol.absolute() == 1e-5);
    REQUIRE(tol.is_zero(1e-6) == true);  // Should be true with new tolerance
    REQUIRE(tol.is_zero(1e-4) == false); // Should be false

    // Reset to defaults and verify
    tol.reset();
    REQUIRE(tol.precision() == Tolerance::PRECISION);
    REQUIRE(tol.absolute() == Tolerance::ABSOLUTE);
    REQUIRE(tol.format_number(1.23456) == "1.235"); // Back to 3 decimal places

    // Verify absolute tolerance is back to default
    REQUIRE(tol.is_zero(1e-6) == false); // Should be false with default tolerance
}

TEST_CASE("Tolerance is zero", "[tolerance]") {
    Tolerance tol;
    REQUIRE(tol.is_zero(1e-10) == true);
    REQUIRE(tol.is_zero(1e-5) == false);
}

TEST_CASE("Tolerance is close", "[tolerance]") {
    Tolerance tol;
    REQUIRE(tol.is_close(1.0, 1.0 + 1e-5) == false);
    REQUIRE(tol.is_close(1.0, 1.0 + 1e-6) == true);
    REQUIRE(tol.is_close(0.0, 0.0 + 1e-9) == true);
}

TEST_CASE("Tolerance geometric key", "[tolerance]") {
    Tolerance tol;
    REQUIRE(tol.geometric_key(1.0, 2.0, 3.0) == "1.000,2.000,3.000");
    REQUIRE(tol.geometric_key(1.05725, 2.0195, 3.001, 3) == "1.057,2.019,3.001");
    REQUIRE(tol.geometric_key(1.0, 2.0, 3.0, -1) == "1,2,3");
}

TEST_CASE("Tolerance is positive", "[tolerance]") {
    Tolerance tol;
    REQUIRE(tol.is_positive(1e-7) == true);
    REQUIRE(tol.is_positive(1e-10) == false);
}

TEST_CASE("Tolerance is negative", "[tolerance]") {
    Tolerance tol;
    REQUIRE(tol.is_negative(-1e-7) == true);
    REQUIRE(tol.is_negative(-1e-10) == false);
}

TEST_CASE("Tolerance is between", "[tolerance]") {
    Tolerance tol;
    REQUIRE(tol.is_between(0.5, 0.0, 1.0) == true);
    REQUIRE(tol.is_between(1.5, 0.0, 1.0) == false);
}
