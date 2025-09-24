#include "catch_amalgamated.hpp"
#include "vector.h"
#include <filesystem>
#include <fstream>

namespace session_cpp {

TEST_CASE("Vector constructor.") {
  Vector vector(1.0, 2.0, 3.0);
  REQUIRE(vector.x == 1.0);
  REQUIRE(vector.y == 2.0);
  REQUIRE(vector.z == 3.0);
  REQUIRE(vector.name == "my_vector");
  REQUIRE(!vector.guid.empty());
}

TEST_CASE("Vector equality.") {
  Vector p1(1.0, 2.0, 3.0);
  Vector p2(1.0, 2.0, 3.0);
  REQUIRE(p1 == p2);
  REQUIRE_FALSE(p1 != p2);
  Vector p3(1.0, 2.0, 3.0);
  Vector p4(1.1, 2.0, 3.0);
  REQUIRE_FALSE(p3 == p4);
  REQUIRE(p3 != p4);
}

TEST_CASE("Vector to_json_data") {
  Vector vector(15.5, 25.7, 35.9);
  vector.name = "my_vector";
  auto data = vector.to_json_data();
  REQUIRE(data["type"] == "Vector");
  REQUIRE(data["name"] == "my_vector");
  REQUIRE(data["x"] == 15.5);
  REQUIRE(data["y"] == 25.7);
  REQUIRE(data["z"] == 35.9);
}

TEST_CASE("Vector from_json_data") {
  Vector original(42.1, 84.2, 126.3);
  original.name = "control_point_B";
  auto data = original.to_json_data();
  Vector restored = Vector::from_json_data(data);
  REQUIRE(restored.x == 42.1);
  REQUIRE(restored.y == 84.2);
  REQUIRE(restored.z == 126.3);
  REQUIRE(restored.name == "control_point_B");
  REQUIRE(restored.guid == original.guid);
}

TEST_CASE("Vector to_json from_json") {
  Vector original(123.45, 678.90, 999.11);
  original.name = "file_test_point";
  std::string filename = "test_vector.json";
  original.to_json(filename);
  Vector loaded = Vector::from_json(filename);
  REQUIRE(loaded.x == original.x);
  REQUIRE(loaded.y == original.y);
  REQUIRE(loaded.z == original.z);
  REQUIRE(loaded.name == original.name);
  REQUIRE(loaded.guid == original.guid);
}

} // namespace session_cpp