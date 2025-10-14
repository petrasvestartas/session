#include "catch_amalgamated.hpp"
#include "pointcloud.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("PointCloud JSON roundtrip", "[pointcloud]") {
    std::vector<Point> points = {Point(1.0, 2.0, 3.0), Point(4.0, 5.0, 6.0)};
    PointCloud original(points, {}, {});
    original.name = "test_cloud";
    
    encoders::json_dump(original, "test_pointcloud.json");
    PointCloud loaded = encoders::json_load<PointCloud>("test_pointcloud.json");

    encoders::json_dump(original, "test_pointcloud.json");
    
    REQUIRE(loaded.points.size() == original.points.size());
    REQUIRE(loaded.name == original.name);}
