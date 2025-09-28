#include "catch_amalgamated.hpp"
#include "vector.h"
#include <cmath>
#include <filesystem>
#include <fstream>

namespace session_cpp {

TEST_CASE("Vector constructor.") {
  Vector v(1.0, 2.0, 3.0);
  REQUIRE(v.x() == 1.0);
  REQUIRE(v.y() == 2.0);
  REQUIRE(v.z() == 3.0);
  REQUIRE(!v.guid.empty());
}

TEST_CASE("Vector equality.") {
  Vector v1(1.0, 2.0, 3.0), v2(1.0, 2.0, 3.0), v3(1.1, 2.0, 3.0);
  REQUIRE(v1 == v2);
  REQUIRE_FALSE(v1 != v2);
  REQUIRE_FALSE(v1 == v3);
  REQUIRE(v1 != v3);
}

TEST_CASE("Vector to_json_data") {
  Vector v(15.5, 25.7, 35.9);
  v.name = "my_vector";
  auto data = v.to_json_data();
  REQUIRE(data["type"] == "Vector");
  REQUIRE(data["name"] == "my_vector");
  REQUIRE(data["x"] == 15.5);
  REQUIRE(data["y"] == 25.7);
  REQUIRE(data["z"] == 35.9);
}

TEST_CASE("Vector from_json_data") {
  Vector orig(42.1, 84.2, 126.3);
  orig.name = "control_point_B";
  Vector rest = Vector::from_json_data(orig.to_json_data());
  REQUIRE(rest.x() == 42.1);
  REQUIRE(rest.y() == 84.2);
  REQUIRE(rest.z() == 126.3);
  REQUIRE(rest.name == "control_point_B");
  REQUIRE(rest.guid == orig.guid);
}

TEST_CASE("Vector to_json from_json") {
  Vector orig(123.45, 678.90, 999.11);
  orig.name = "file_test_point";
  orig.to_json("test_vector.json");
  Vector load = Vector::from_json("test_vector.json");
  REQUIRE(load.x() == orig.x());
  REQUIRE(load.y() == orig.y());
  REQUIRE(load.z() == orig.z());
  REQUIRE(load.name == orig.name);
  REQUIRE(load.guid == orig.guid);
}

TEST_CASE("Vector default constructor") {
  Vector v;
  REQUIRE(v[0] == 0);
  REQUIRE(v[1] == 0);
  REQUIRE(v[2] == 0);
}

TEST_CASE("Vector constructor") {
  Vector v(0.57, -158.63, 180.890);
  REQUIRE(v[0] == 0.57);
  REQUIRE(v[1] == -158.63);
  REQUIRE(v[2] == 180.890);
}

TEST_CASE("Vector static methods") {
  Vector x = Vector::XAxis(), y = Vector::YAxis(), z = Vector::ZAxis();
  REQUIRE((x[0] == 1 && x[1] == 0 && x[2] == 0));
  REQUIRE((y[0] == 0 && y[1] == 1 && y[2] == 0));
  REQUIRE((z[0] == 0 && z[1] == 0 && z[2] == 1));
}

TEST_CASE("Vector from_start_and_end") {
  Vector v = Vector::from_start_and_end(Vector(8.7, 5.7, -1.87), Vector(1, 1.57, 2));
  REQUIRE(std::abs(v[0] + 7.7) < geo::GLOBALS::ZERO_TOLERANCE);
  REQUIRE(std::abs(v[1] + 4.13) < geo::GLOBALS::ZERO_TOLERANCE);
  REQUIRE(std::abs(v[2] - 3.87) < geo::GLOBALS::ZERO_TOLERANCE);
}

TEST_CASE("Vector operators and compound ops") {
  Vector v1(1, 2, 3);
  Vector v2(4, 5, 6);
  Vector v3 = v1 + v2;
  REQUIRE(v3[0] == 5);
  REQUIRE(v3[1] == 7);
  REQUIRE(v3[2] == 9);
  v3 = v1 - v2;
  REQUIRE(v3[0] == -3);
  REQUIRE(v3[1] == -3);
  REQUIRE(v3[2] == -3);
  v3 = v1 * 2;
  REQUIRE(v3[0] == 2);
  REQUIRE(v3[1] == 4);
  REQUIRE(v3[2] == 6);
  v3 = v1 / 2;
  REQUIRE(v3[0] == 0.5);
  REQUIRE(v3[1] == 1);
  REQUIRE(v3[2] == 1.5);
  v3 = v1;
  v3 += v2;
  REQUIRE(v3[0] == 5);
  REQUIRE(v3[1] == 7);
  REQUIRE(v3[2] == 9);
  v3 -= v2;
  REQUIRE(v3[0] == 1);
  REQUIRE(v3[1] == 2);
  REQUIRE(v3[2] == 3);
  v3 *= 2;
  REQUIRE(v3[0] == 2);
  REQUIRE(v3[1] == 4);
  REQUIRE(v3[2] == 6);
  v3 /= 2;
  REQUIRE(v3[0] == 1);
  REQUIRE(v3[1] == 2);
  REQUIRE(v3[2] == 3);
}

TEST_CASE("Vector reverse") {
  Vector v(1, 2, 3);
  v.reverse();
  REQUIRE((v[0] == -1 && v[1] == -2 && v[2] == -3));
}

TEST_CASE("Vector length") {
  Vector v(5.5697, -9.84, 1.587);
  double length = v.length();
  REQUIRE(length == 11.4177811806848);
}

TEST_CASE("Vector unitize") {
  Vector v(5.5697, -9.84, 1.587);
  REQUIRE(v.unitized().length() == 1);
  v.unitize();
  REQUIRE(v.length() == 1);
}

TEST_CASE("Vector projection") {
  Vector v(1, 1, 1), x(1, 0, 0), y(0, 1, 0), z(0, 0, 1);
  auto [px, lenx, perp_x, plenx] = v.projection(x);
  auto [py, leny, perp_y, pleny] = v.projection(y);
  auto [pz, lenz, perp_z, plenz] = v.projection(z);
  REQUIRE((px[0] == 1 && px[1] == 0 && px[2] == 0));
  REQUIRE((py[0] == 0 && py[1] == 1 && py[2] == 0));
  REQUIRE((pz[0] == 0 && pz[1] == 0 && pz[2] == 1));
}

TEST_CASE("Vector is_parallel_to") {
  Vector v1(0, 0, 1), v2(0, 0, 2), v3(0, 0, -1), v4(0, 1, -1);
  REQUIRE(v1.is_parallel_to(v2) == 1);
  REQUIRE(v1.is_parallel_to(v3) == -1);
  REQUIRE(v1.is_parallel_to(v4) == 0);
}

TEST_CASE("Vector dot") {
  Vector v1(1, 0, 0), v2(0, 1, 0), v3(-1, 0, 0);
  REQUIRE(v1.dot(v2) == 0);
  REQUIRE(v1.dot(v3) == -1);
  REQUIRE(v1.dot(v1) == 1);
  
  double dot = v1.dot(v2), mag = v1.length() * v2.length();
  if (mag > 0.0) {
    double angle_deg = std::acos(dot / mag) * geo::GLOBALS::TO_DEGREES;
    REQUIRE(angle_deg == 90);
  }
}

TEST_CASE("Vector cross") {
  Vector v1(1, 0, 0), v2(0, 1, 0);
  Vector v3 = v1.cross(v2);
  REQUIRE((v3[0] == 0 && v3[1] == 0 && v3[2] == 1));
}

TEST_CASE("Vector angle") {
  Vector v1(1, 1, 0), v2(0, 1, 0);
  REQUIRE(std::abs(v1.angle(v2, false) - 45) < geo::GLOBALS::ZERO_TOLERANCE);
  REQUIRE(std::abs(Vector(-1, 1, 0).angle(v2, true) + 45) < geo::GLOBALS::ZERO_TOLERANCE);
}

TEST_CASE("Vector get_leveled_vector") {
  double scale = 1.0;
  Vector lev = Vector(1, 1, 1).get_leveled_vector(scale);
  REQUIRE(std::abs(lev.length() - 4.1684325329666283) < geo::GLOBALS::ZERO_TOLERANCE);
}

TEST_CASE("Vector cosine_law") {
  double a = 100, b = 150, angle = 115;
  double c = Vector::cosine_law(a, b, angle, true);
  REQUIRE(std::round(c * 100) / 100 == 212.55);
}

TEST_CASE("Vector sine_law_angle") {
  double a = 212.55, angle_a = 115, b = 150;
  double angle_b = Vector::sine_law_angle(a, angle_a, b);
  REQUIRE(std::round(angle_b * 100) / 100 == 39.76);
}

TEST_CASE("Vector sine_law_length") {
  double a = 212.55, angle_a = 115, angle_b = 39.761714;
  double len_b = Vector::sine_law_length(a, angle_a, angle_b);
  REQUIRE(std::round(len_b * 100) / 100 == 150);
}

TEST_CASE("Vector angle_between_vector_xy_components") {
  Vector v1(std::sqrt(3), 1, 0), v2(1, std::sqrt(3), 0);
  REQUIRE(std::round(Vector::angle_between_vector_xy_components(v1) * 100) / 100 == 30);
  REQUIRE(std::round(Vector::angle_between_vector_xy_components(v2) * 100) / 100 == 60);
}

TEST_CASE("Vector sum_of_vectors") {
  std::vector<Vector> vecs = {Vector(1, 1, 1), Vector(2, 2, 2), Vector(3, 3, 3)};
  Vector sum = Vector::sum_of_vectors(vecs);
  REQUIRE((sum[0] == 6 && sum[1] == 6 && sum[2] == 6));
}

TEST_CASE("Vector coordinate_direction_angles") {
  auto abg = Vector(35.4, 35.4, 86.6).coordinate_direction_3angles(true);
  REQUIRE(std::abs(abg[0] - 69.274204) < 1e-6);
  REQUIRE(std::abs(abg[1] - 69.274204) < 1e-6);
  REQUIRE(std::abs(abg[2] - 30.032058) < 1e-6);

  auto pt = Vector(1, 1, std::sqrt(2)).coordinate_direction_2angles(true);
  REQUIRE((std::abs(pt[0] - 45) < 1e-6 && std::abs(pt[1] - 45) < 1e-6));
}

TEST_CASE("Vector scale methods") {
  Vector vs(1, 1, 1);
  vs.scale(2);
  REQUIRE((vs[0] == 2 && vs[1] == 2 && vs[2] == 2));

  Vector vsu(1, 1, 1);
  vsu.scale_up();
  REQUIRE((vsu[0] == geo::GLOBALS::SCALE && vsu[1] == geo::GLOBALS::SCALE && vsu[2] == geo::GLOBALS::SCALE));

  Vector vsd(geo::GLOBALS::SCALE, geo::GLOBALS::SCALE, geo::GLOBALS::SCALE);
  vsd.scale_down();
  REQUIRE((vsd[0] == 1 && vsd[1] == 1 && vsd[2] == 1));
}

} // namespace session_cpp