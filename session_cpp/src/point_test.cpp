#include "catch_amalgamated.hpp"
#include "color.h"
#include "point.h"
#include <cmath>
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

///////////////////////////////////////////////////////////////////////////////////////////
// No-copy Operators
///////////////////////////////////////////////////////////////////////////////////////////

TEST_CASE("Point getitem") {
  Point point(1.0, 2.0, 3.0);
  REQUIRE(point[0] == 1.0);
  REQUIRE(point[1] == 2.0);
  REQUIRE(point[2] == 3.0);
}

TEST_CASE("Point setitem") {
  Point point(1.0, 2.0, 3.0);
  point[0] = 4.0;
  point[1] = 5.0;
  point[2] = 6.0;
  REQUIRE(point.x == 4.0);
  REQUIRE(point.y == 5.0);
  REQUIRE(point.z == 6.0);
}

TEST_CASE("Point imul") {
  Point point(1.0, 2.0, 3.0);
  point *= 2.0;
  REQUIRE(point.x == 2.0);
  REQUIRE(point.y == 4.0);
  REQUIRE(point.z == 6.0);
}

TEST_CASE("Point itruediv") {
  Point point(2.0, 4.0, 6.0);
  point /= 2.0;
  REQUIRE(point.x == 1.0);
  REQUIRE(point.y == 2.0);
  REQUIRE(point.z == 3.0);
}

TEST_CASE("Point iadd") {
  Point point(1.0, 2.0, 3.0);
  point += Point(4.0, 5.0, 6.0);
  REQUIRE(point.x == 5.0);
  REQUIRE(point.y == 7.0);
  REQUIRE(point.z == 9.0);
}

TEST_CASE("Point isub") {
  Point point(5.0, 7.0, 9.0);
  point -= Point(4.0, 5.0, 6.0);
  REQUIRE(point.x == 1.0);
  REQUIRE(point.y == 2.0);
  REQUIRE(point.z == 3.0);
}

///////////////////////////////////////////////////////////////////////////////////////////
// Copy Operators
///////////////////////////////////////////////////////////////////////////////////////////

TEST_CASE("Point mul") {
  Point point(1.0, 2.0, 3.0);
  Point result = point * 2.0;
  REQUIRE(result.x == 2.0);
  REQUIRE(result.y == 4.0);
  REQUIRE(result.z == 6.0);
}

TEST_CASE("Point truediv") {
  Point point(2.0, 4.0, 6.0);
  Point result = point / 2.0;
  REQUIRE(result.x == 1.0);
  REQUIRE(result.y == 2.0);
  REQUIRE(result.z == 3.0);
}

TEST_CASE("Point add") {
  Point point(1.0, 2.0, 3.0);
  Point result = point + Point(4.0, 5.0, 6.0);
  REQUIRE(result.x == 5.0);
  REQUIRE(result.y == 7.0);
  REQUIRE(result.z == 9.0);
}

TEST_CASE("Point sub") {
  Point point(5.0, 7.0, 9.0);
  Point result = point - Point(4.0, 5.0, 6.0);
  REQUIRE(result.x == 1.0);
  REQUIRE(result.y == 2.0);
  REQUIRE(result.z == 3.0);
}

///////////////////////////////////////////////////////////////////////////////////////////
// Details
///////////////////////////////////////////////////////////////////////////////////////////

TEST_CASE("Point ccw") {
  Point a(0.0, 0.0, 0.0);
  Point b(1.0, 0.0, 0.0);
  Point c(0.0, 1.0, 0.0);
  REQUIRE(Point::ccw(a, b, c));
  REQUIRE_FALSE(Point::ccw(b, a, c));
}

TEST_CASE("Point mid_point") {
  Point p1(0.0, 0.0, 0.0);
  Point p2(1.0, 0.0, 0.0);
  Point mid = p1.mid_point(p2);
  REQUIRE(std::round(mid.x * 1000000) / 1000000 == 0.5);
  REQUIRE(std::round(mid.y * 1000000) / 1000000 == 0.0);
  REQUIRE(std::round(mid.z * 1000000) / 1000000 == 0.0);
}

TEST_CASE("Point distance") {
  Point p1(0.0, 0.0, 0.0);
  Point p2(1.0, 0.0, 0.0);
  REQUIRE(std::round(p1.distance(p2) * 1000000) / 1000000 == 1.0);
}

TEST_CASE("Point area") {
  std::vector<Point> points = {Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(0.0, 1.0, 0.0)};
  REQUIRE(Point::area(points) == 0.5);
}

TEST_CASE("Point centroid_quad") {
  std::vector<Point> vertices = {Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0), Point(0.0, 1.0, 0.0)};
  Point centroid = Point::centroid_quad(vertices);
  REQUIRE(std::round(centroid.x * 1000000) / 1000000 == 0.5);
  REQUIRE(std::round(centroid.y * 1000000) / 1000000 == 0.5);
  REQUIRE(std::round(centroid.z * 1000000) / 1000000 == 0.0);
}

} // namespace session_cpp
