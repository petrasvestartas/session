#include "catch_amalgamated.hpp"
#include "point.h"
#include "encoders.h"

#ifdef ENABLE_PROTOBUF
#include "point.pb.h"
#include <fstream>
#endif

using namespace session_cpp;

TEST_CASE("Point JSON roundtrip", "[point]") {
    Point original(42.1, 84.2, 126.3);
    original.name = "test_point";
    original.width = 3.0;
    
    
    encoders::json_dump(original, "test_point.json");
    Point loaded = encoders::json_load<Point>("test_point.json");

    encoders::json_dump(original, "test_point.json");
    
    REQUIRE(std::abs(loaded.x()-original.x()) < 0.0001);
    REQUIRE(std::abs(loaded.y()-original.y()) < 0.0001);
    REQUIRE(std::abs(loaded.z()-original.z()) < 0.0001);
    REQUIRE(loaded.name == original.name);
    REQUIRE(loaded.width == original.width);
}

#ifdef ENABLE_PROTOBUF
TEST_CASE("Point protobuf roundtrip", "[point][protobuf]") {
    // Create and serialize to string
    session_proto::Point original;
    original.set_name("test_point");
    original.set_x(1.0);
    original.set_y(2.0);
    original.set_z(3.0);
    
    std::string data;
    original.SerializeToString(&data);
    
    // Deserialize from string and verify
    session_proto::Point loaded;
    loaded.ParseFromString(data);
    
    REQUIRE(loaded.name() == "test_point");
    REQUIRE(loaded.x() == 1.0);
    REQUIRE(loaded.y() == 2.0);
    REQUIRE(loaded.z() == 3.0);
    
    // Save to file
    const std::string filename = "test_point.pb";
    {
        std::ofstream ofs(filename, std::ios::binary);
        original.SerializeToOstream(&ofs);
    }
    
    // Load from file and verify
    session_proto::Point from_file;
    {
        std::ifstream ifs(filename, std::ios::binary);
        from_file.ParseFromIstream(&ifs);
    }
    
    REQUIRE(from_file.name() == "test_point");
    REQUIRE(from_file.x() == 1.0);
    REQUIRE(from_file.y() == 2.0);
    REQUIRE(from_file.z() == 3.0);

}
#endif
