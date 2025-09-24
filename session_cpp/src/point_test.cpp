#include "catch_amalgamated.hpp"
#include "color.h"
#include "point.h"
#include <filesystem>
#include <fstream>

namespace session_cpp {

TEST_CASE("Point constructor.") {
  Point point(1.0, 2.0, 3.0);
  REQUIRE(point.x == 1.0);
  REQUIRE(point.y == 2.0);
  REQUIRE(point.z == 3.0);
  REQUIRE(point.name == "my_point");
  REQUIRE(point.width == 1.0);
  REQUIRE(point.pointcolor == Color::white());
  REQUIRE(!point.guid.empty());
}

TEST_CASE("Point equality.") {
  Point p1(1.0, 2.0, 3.0);
  Point p2(1.0, 2.0, 3.0);
  REQUIRE(p1 == p2);
  REQUIRE_FALSE(p1 != p2);
  Point p3(1.0, 2.0, 3.0);
  Point p4(1.1, 2.0, 3.0);
  REQUIRE_FALSE(p3 == p4);
  REQUIRE(p3 != p4);
}

TEST_CASE("Point to_json_data") {
  Point point(15.5, 25.7, 35.9);
  point.name = "survey_point_A";
  point.width = 2.5;
  point.pointcolor = Color(255, 128, 64, 255);
  auto data = point.to_json_data();
  REQUIRE(data["type"] == "Point");
  REQUIRE(data["name"] == "survey_point_A");
  REQUIRE(data["x"] == 15.5);
  REQUIRE(data["y"] == 25.7);
  REQUIRE(data["z"] == 35.9);
  REQUIRE(data["width"] == 2.5);
  REQUIRE(data["pointcolor"]["r"] == 255);
  REQUIRE(data["pointcolor"]["g"] == 128);
  REQUIRE(data["pointcolor"]["b"] == 64);
  REQUIRE(data["pointcolor"]["a"] == 255);
}

TEST_CASE("Point from_json_data") {
  Point original(42.1, 84.2, 126.3);
  original.name = "control_point_B";
  original.width = 3.0;
  original.pointcolor = Color(200, 100, 50, 255);
  auto data = original.to_json_data();
  Point restored = Point::from_json_data(data);
  REQUIRE(restored.x == 42.1);
  REQUIRE(restored.y == 84.2);
  REQUIRE(restored.z == 126.3);
  REQUIRE(restored.name == "control_point_B");
  REQUIRE(restored.width == 3.0);
  REQUIRE(restored.pointcolor.r == 200);
  REQUIRE(restored.pointcolor.g == 100);
  REQUIRE(restored.pointcolor.b == 50);
  REQUIRE(restored.pointcolor.a == 255);
  REQUIRE(restored.guid == original.guid);
}

TEST_CASE("Point to_json from_json") {
  Point original(123.45, 678.90, 999.11);
  original.name = "file_test_point";
  original.width = 4.5;
  original.pointcolor = Color(0, 255, 128, 255);
  std::string filename = "test_point.json";
  original.to_json(filename);
  Point loaded = Point::from_json(filename);
  REQUIRE(loaded.x == original.x);
  REQUIRE(loaded.y == original.y);
  REQUIRE(loaded.z == original.z);
  REQUIRE(loaded.name == original.name);
  REQUIRE(loaded.width == original.width);
  REQUIRE(loaded.pointcolor == original.pointcolor);
  REQUIRE(loaded.guid == original.guid);
}

} // namespace session_cpp
