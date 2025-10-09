#include "catch_amalgamated.hpp"
#include "cylinder.h"
#include "line.h"

using namespace session_cpp;

TEST_CASE("Cylinder: new", "[cylinder]") {
    Line line(0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 10.0f);
    Cylinder cylinder(line, 1.0f);
    
    REQUIRE(cylinder.radius == 1.0f);
    REQUIRE(cylinder.mesh.number_of_vertices() == 20);
    REQUIRE(cylinder.mesh.number_of_faces() == 20);
    REQUIRE(!cylinder.guid.empty());
    REQUIRE(cylinder.name == "my_cylinder");
}

TEST_CASE("Cylinder: JSON serialization", "[cylinder]") {
    Line line(0.0f, 0.0f, 0.0f, 5.0f, 0.0f, 0.0f);
    Cylinder cylinder(line, 2.0f);
    
    auto json = cylinder.to_json_data();
    Cylinder deserialized = Cylinder::from_json_data(json);
    
    REQUIRE(deserialized.radius == 2.0f);
    REQUIRE(deserialized.mesh.number_of_vertices() == 20);
    REQUIRE(deserialized.mesh.number_of_faces() == 20);
}

TEST_CASE("Cylinder: to_json_data", "[cylinder]") {
    Line line(0.0f, 0.0f, 0.0f, 10.0f, 0.0f, 0.0f);
    Cylinder cylinder(line, 1.5f);
    
    auto json_data = cylinder.to_json_data();
    REQUIRE(json_data["type"] == "Cylinder");
    REQUIRE(json_data.contains("radius"));
    REQUIRE(json_data["radius"] == 1.5f);
}

TEST_CASE("Cylinder: from_json_data", "[cylinder]") {
    Line line(1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
    Cylinder cylinder(line, 0.5f);
    
    auto json_data = cylinder.to_json_data();
    Cylinder deserialized = Cylinder::from_json_data(json_data);
    
    REQUIRE(deserialized.radius == 0.5f);
    REQUIRE(deserialized.mesh.number_of_vertices() == 20);
    REQUIRE(deserialized.mesh.number_of_faces() == 20);
}

TEST_CASE("Cylinder: to_json and from_json", "[cylinder]") {
    Line line(0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 8.0f);
    Cylinder cylinder(line, 1.0f);
    
    std::string filepath = "../test_cylinder.json";
    cylinder.to_json(filepath);
    
    Cylinder loaded = Cylinder::from_json(filepath);
    REQUIRE(loaded.radius == 1.0f);
    REQUIRE(loaded.mesh.number_of_vertices() == 20);
    REQUIRE(loaded.mesh.number_of_faces() == 20);

}
