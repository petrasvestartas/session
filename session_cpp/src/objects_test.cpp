#include "catch_amalgamated.hpp"
#include "objects.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Objects JSON roundtrip", "[objects]") {
    Objects original;
    auto point1 = std::make_shared<Point>(1.0, 2.0, 3.0);
    auto point2 = std::make_shared<Point>(4.0, 5.0, 6.0);
    original.points->push_back(point1);
    original.points->push_back(point2);
    
    encoders::json_dump(original, "test_objects.json");
    Objects loaded = encoders::json_load<Objects>("test_objects.json");

    encoders::json_dump(original, "test_objects.json");
    
    REQUIRE(loaded.points->size() == original.points->size());}
