#include "catch_amalgamated.hpp"
#include "arrow.h"

using namespace session_cpp;

TEST_CASE("Arrow creation", "[arrow]") {
    Line line(0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 10.0f);
    Arrow arrow(line, 1.0f);

    REQUIRE(arrow.radius == 1.0f);
    REQUIRE(arrow.mesh.number_of_vertices() == 29);
    REQUIRE(arrow.mesh.number_of_faces() == 28);
    REQUIRE(arrow.name == "my_arrow");
    REQUIRE(!arrow.guid.empty());
}

TEST_CASE("Arrow JSON serialization", "[arrow]") {
    Line line(0.0f, 0.0f, 0.0f, 5.0f, 0.0f, 0.0f);
    Arrow arrow(line, 2.0f);

    auto data = arrow.to_json_data();
    REQUIRE(data["type"] == "Arrow");
    REQUIRE(data["radius"] == 2.0f);
    REQUIRE(data.contains("mesh"));
    REQUIRE(data.contains("line"));
}

TEST_CASE("Arrow JSON round trip", "[arrow]") {
    Line line(1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
    Arrow arrow(line, 0.5f);

    std::string filepath = "test_arrow.json";
    arrow.to_json(filepath);

    Arrow loaded = Arrow::from_json(filepath);
    REQUIRE(loaded.radius == 0.5f);
    REQUIRE(loaded.mesh.number_of_vertices() == 29);
    REQUIRE(loaded.mesh.number_of_faces() == 28);
}

TEST_CASE("Arrow mesh has color collections", "[arrow]") {
    Line line(0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 10.0f);
    Arrow arrow(line, 1.0f);

    REQUIRE(arrow.mesh.pointcolors.size() == 29);
    REQUIRE(arrow.mesh.facecolors.size() == 28);
    REQUIRE(arrow.mesh.linecolors.size() == 56);
    REQUIRE(arrow.mesh.widths.size() == 56);
}
