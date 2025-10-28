#include "catch_amalgamated.hpp"
#include "session.pb.h"
#include "point.h"
#include <fstream>
#include <string>

using namespace session_cpp;
using namespace session_proto;

TEST_CASE("Protobuf Point serialization", "[protobuf]") {
    SECTION("Create and serialize Point message") {
        // Create a protobuf Point message
        session_proto::Point pb_point;
        pb_point.set_guid("test-guid-123");
        pb_point.set_name("test_point");
        pb_point.set_x(1.5);
        pb_point.set_y(2.5);
        pb_point.set_z(3.5);
        pb_point.set_width(2.0);
        
        // Set color
        auto* color = pb_point.mutable_pointcolor();
        color->set_r(255);
        color->set_g(128);
        color->set_b(64);
        color->set_a(255);
        
        // Verify values
        REQUIRE(pb_point.guid() == "test-guid-123");
        REQUIRE(pb_point.name() == "test_point");
        REQUIRE(pb_point.x() == 1.5);
        REQUIRE(pb_point.y() == 2.5);
        REQUIRE(pb_point.z() == 3.5);
        REQUIRE(pb_point.width() == 2.0);
        REQUIRE(pb_point.pointcolor().r() == 255);
        REQUIRE(pb_point.pointcolor().g() == 128);
        REQUIRE(pb_point.pointcolor().b() == 64);
        REQUIRE(pb_point.pointcolor().a() == 255);
    }
    
    SECTION("Serialize to binary and deserialize") {
        // Create original point
        session_proto::Point original;
        original.set_guid("binary-test");
        original.set_name("binary_point");
        original.set_x(10.0);
        original.set_y(20.0);
        original.set_z(30.0);
        original.set_width(5.0);
        
        // Serialize to string
        std::string serialized;
        REQUIRE(original.SerializeToString(&serialized));
        REQUIRE(serialized.size() > 0);
        
        // Deserialize
        session_proto::Point deserialized;
        REQUIRE(deserialized.ParseFromString(serialized));
        
        // Verify values match
        REQUIRE(deserialized.guid() == original.guid());
        REQUIRE(deserialized.name() == original.name());
        REQUIRE(deserialized.x() == original.x());
        REQUIRE(deserialized.y() == original.y());
        REQUIRE(deserialized.z() == original.z());
        REQUIRE(deserialized.width() == original.width());
    }
    
    SECTION("Write to file and read back") {
        const std::string filename = "test_point.bin";
        
        // Create and write point
        session_proto::Point original;
        original.set_guid("file-test");
        original.set_name("file_point");
        original.set_x(100.0);
        original.set_y(200.0);
        original.set_z(300.0);
        original.set_width(10.0);
        
        {
            std::ofstream ofs(filename, std::ios::binary);
            REQUIRE(original.SerializeToOstream(&ofs));
        }
        
        // Read back
        session_proto::Point loaded;
        {
            std::ifstream ifs(filename, std::ios::binary);
            REQUIRE(loaded.ParseFromIstream(&ifs));
        }
        
        // Verify
        REQUIRE(loaded.guid() == "file-test");
        REQUIRE(loaded.name() == "file_point");
        REQUIRE(loaded.x() == 100.0);
        REQUIRE(loaded.y() == 200.0);
        REQUIRE(loaded.z() == 300.0);
        REQUIRE(loaded.width() == 10.0);
        
        // Cleanup
        std::remove(filename.c_str());
    }
}

TEST_CASE("Protobuf Color serialization", "[protobuf]") {
    SECTION("Create and verify Color") {
        session_proto::Color color;
        color.set_r(100);
        color.set_g(150);
        color.set_b(200);
        color.set_a(255);
        
        REQUIRE(color.r() == 100);
        REQUIRE(color.g() == 150);
        REQUIRE(color.b() == 200);
        REQUIRE(color.a() == 255);
    }
}

TEST_CASE("Protobuf Vector serialization", "[protobuf]") {
    SECTION("Create and verify Vector") {
        session_proto::Vector vec;
        vec.set_x(1.0);
        vec.set_y(2.0);
        vec.set_z(3.0);
        
        REQUIRE(vec.x() == 1.0);
        REQUIRE(vec.y() == 2.0);
        REQUIRE(vec.z() == 3.0);
    }
}

TEST_CASE("Protobuf Xform serialization", "[protobuf]") {
    SECTION("Create identity matrix") {
        session_proto::Xform xform;
        
        // Add 16 values for 4x4 identity matrix (column-major)
        for (int i = 0; i < 16; i++) {
            if (i == 0 || i == 5 || i == 10 || i == 15) {
                xform.add_matrix(1.0);  // Diagonal elements
            } else {
                xform.add_matrix(0.0);  // Off-diagonal elements
            }
        }
        
        REQUIRE(xform.matrix_size() == 16);
        REQUIRE(xform.matrix(0) == 1.0);   // [0,0]
        REQUIRE(xform.matrix(5) == 1.0);   // [1,1]
        REQUIRE(xform.matrix(10) == 1.0);  // [2,2]
        REQUIRE(xform.matrix(15) == 1.0);  // [3,3]
    }
}
