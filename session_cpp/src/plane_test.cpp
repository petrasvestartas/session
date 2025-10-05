#include "catch_amalgamated.hpp"
#include "plane.h"
#include "point.h"
#include "vector.h"
#include <cmath>
#include <filesystem>
#include <fstream>

namespace session_cpp {

TEST_CASE("Plane default constructor") {
  Plane plane;
  REQUIRE(plane.origin() == Point(0.0f, 0.0f, 0.0f));
  REQUIRE(plane.x_axis() == Vector::x_axis());
  REQUIRE(plane.y_axis() == Vector::y_axis());
  REQUIRE(plane.z_axis() == Vector::z_axis());
  REQUIRE(plane.a() == 0.0f);
  REQUIRE(plane.b() == 0.0f);
  REQUIRE(plane.c() == 1.0f);
  REQUIRE(plane.d() == 0.0f);
}

TEST_CASE("Plane constructor from origin and axes") {
  Point origin(1.0f, 2.0f, 3.0f);
  Vector x(1.0f, 0.0f, 0.0f);
  Vector y(0.0f, 1.0f, 0.0f);
  Plane plane(origin, x, y, "test_plane");
  REQUIRE(plane.name == "test_plane");
  REQUIRE(plane.origin() == origin);
  REQUIRE(plane.c() == 1.0f);
}

TEST_CASE("Plane from_point_normal") {
  Point p(0.0f, 0.0f, 5.0f);
  Vector n(0.0f, 0.0f, 1.0f);
  Plane plane = Plane::from_point_normal(p, n);
  REQUIRE(plane.origin() == p);
  REQUIRE(std::abs(plane.z_axis().z() - 1.0f) < 1e-5f);
  REQUIRE(std::abs(plane.d() + 5.0f) < 1e-5f);
}

TEST_CASE("Plane from_points") {
  std::vector<Point> points = {
    Point(0.0f, 0.0f, 0.0f),
    Point(1.0f, 0.0f, 0.0f),
    Point(0.0f, 1.0f, 0.0f)
  };
  Plane plane = Plane::from_points(points);
  REQUIRE(std::abs(plane.c() - 1.0f) < 1e-5f);
  REQUIRE(std::abs(plane.d()) < 1e-5f);
}

TEST_CASE("Plane from_two_points") {
  Point p1(0.0f, 0.0f, 0.0f);
  Point p2(1.0f, 0.0f, 0.0f);
  Plane plane = Plane::from_two_points(p1, p2);
  REQUIRE(plane.origin() == p1);
}

TEST_CASE("Plane xy_plane") {
  Plane plane = Plane::xy_plane();
  REQUIRE(plane.name == "xy_plane");
  REQUIRE(plane.a() == 0.0f);
  REQUIRE(plane.b() == 0.0f);
  REQUIRE(plane.c() == 1.0f);
  REQUIRE(plane.d() == 0.0f);
}

TEST_CASE("Plane yz_plane") {
  Plane plane = Plane::yz_plane();
  REQUIRE(plane.name == "yz_plane");
  REQUIRE(plane.a() == 1.0f);
  REQUIRE(plane.b() == 0.0f);
  REQUIRE(plane.c() == 0.0f);
  REQUIRE(plane.d() == 0.0f);
}

TEST_CASE("Plane xz_plane") {
  Plane plane = Plane::xz_plane();
  REQUIRE(plane.name == "xz_plane");
  REQUIRE(plane.a() == 0.0f);
  REQUIRE(plane.b() == 1.0f);
  REQUIRE(plane.c() == 0.0f);
  REQUIRE(plane.d() == 0.0f);
}

TEST_CASE("Plane to_string") {
  Plane plane = Plane::xy_plane();
  std::string str = plane.to_string();
  REQUIRE(str.find("Plane") != std::string::npos);
  REQUIRE(str.find("xy_plane") != std::string::npos);
}

TEST_CASE("Plane operator[]") {
  Plane plane;
  REQUIRE(plane[0] == Vector::x_axis());
  REQUIRE(plane[1] == Vector::y_axis());
  REQUIRE(plane[2] == Vector::z_axis());
}

TEST_CASE("Plane operator+= translation") {
  Plane plane = Plane::xy_plane();
  Vector offset(1.0f, 2.0f, 3.0f);
  plane += offset;
  REQUIRE(plane.origin().x() == 1.0f);
  REQUIRE(plane.origin().y() == 2.0f);
  REQUIRE(plane.origin().z() == 3.0f);
  REQUIRE(std::abs(plane.d() + 3.0f) < 1e-5f);
}

TEST_CASE("Plane operator-= translation") {
  Plane plane = Plane::xy_plane();
  Vector offset(1.0f, 2.0f, 3.0f);
  plane -= offset;
  REQUIRE(plane.origin().x() == -1.0f);
  REQUIRE(plane.origin().y() == -2.0f);
  REQUIRE(plane.origin().z() == -3.0f);
}

TEST_CASE("Plane operator+ translation") {
  Plane plane = Plane::xy_plane();
  Vector offset(1.0f, 2.0f, 3.0f);
  Plane moved = plane + offset;
  REQUIRE(moved.origin().z() == 3.0f);
  REQUIRE(plane.origin().z() == 0.0f);
}

TEST_CASE("Plane operator- translation") {
  Plane plane = Plane::xy_plane();
  Vector offset(1.0f, 2.0f, 3.0f);
  Plane moved = plane - offset;
  REQUIRE(moved.origin().z() == -3.0f);
}

TEST_CASE("Plane to_json_data") {
  Plane plane = Plane::xy_plane();
  auto data = plane.to_json_data();
  REQUIRE(data["type"] == "Plane");
  REQUIRE(data["name"] == "xy_plane");
  REQUIRE(data["a"].get<float>() == 0.0f);
  REQUIRE(data["b"].get<float>() == 0.0f);
  REQUIRE(data["c"].get<float>() == 1.0f);
  REQUIRE(data["d"].get<float>() == 0.0f);
}

TEST_CASE("Plane from_json_data") {
  Plane original = Plane::xy_plane();
  auto data = original.to_json_data();
  Plane loaded = Plane::from_json_data(data);
  REQUIRE(loaded.name == "xy_plane");
  REQUIRE(loaded.c() == 1.0f);
}

TEST_CASE("Plane JSON file round-trip") {
  std::string filepath = "../test_plane.json";
  Plane original = Plane::xy_plane();
  original.to_json(filepath);
  Plane loaded = Plane::from_json(filepath);
  REQUIRE(loaded.name == original.name);
  REQUIRE(loaded.c() == original.c());
}

TEST_CASE("Plane reverse") {
  Plane plane = Plane::xy_plane();
  Vector orig_x = plane.x_axis();
  Vector orig_y = plane.y_axis();
  plane.reverse();
  REQUIRE(plane.x_axis() == orig_y);
  REQUIRE(plane.y_axis() == orig_x);
  REQUIRE(plane.c() == -1.0f);
}

TEST_CASE("Plane rotate") {
  Plane plane = Plane::xy_plane();
  float angle = geo::GLOBALS::PI_F / 2.0f;
  plane.rotate(angle);
  REQUIRE(std::abs(plane.x_axis().y() - 1.0f) < 1e-5f);
}

TEST_CASE("Plane is_same_direction parallel") {
  Plane p1 = Plane::xy_plane();
  Plane p2 = Plane::xy_plane();
  REQUIRE(Plane::is_same_direction(p1, p2, true));
}

TEST_CASE("Plane is_same_direction flipped") {
  Plane p1 = Plane::xy_plane();
  Plane p2 = Plane::xy_plane();
  p2.reverse();
  REQUIRE(Plane::is_same_direction(p1, p2, true));
  REQUIRE_FALSE(Plane::is_same_direction(p1, p2, false));
}

TEST_CASE("Plane is_same_position") {
  Plane p1 = Plane::xy_plane();
  Plane p2 = Plane::xy_plane();
  REQUIRE(Plane::is_same_position(p1, p2));
  p2 += Vector(0.0f, 0.0f, 1.0f);
  REQUIRE_FALSE(Plane::is_same_position(p1, p2));
}

TEST_CASE("Plane is_coplanar") {
  Plane p1 = Plane::xy_plane();
  Plane p2 = Plane::xy_plane();
  REQUIRE(Plane::is_coplanar(p1, p2, true));
  p2.reverse();
  REQUIRE(Plane::is_coplanar(p1, p2, true));
  p2 += Vector(0.0f, 0.0f, 1.0f);
  REQUIRE_FALSE(Plane::is_coplanar(p1, p2, true));
}

TEST_CASE("Plane is_right_hand") {
  Plane plane = Plane::xy_plane();
  REQUIRE(plane.is_right_hand());
  plane = Plane::yz_plane();
  REQUIRE(plane.is_right_hand());
  plane = Plane::xz_plane();
  REQUIRE(plane.is_right_hand());
  plane = Plane();
  REQUIRE(plane.is_right_hand());
  plane.reverse();
  REQUIRE(plane.is_right_hand());
  plane.rotate(geo::GLOBALS::PI_F / 4.0f);
  REQUIRE(plane.is_right_hand());
}

} // namespace session_cpp
