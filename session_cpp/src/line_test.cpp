#include "catch_amalgamated.hpp"
#include "line.h"
#include "point.h"
#include "vector.h"
#include "color.h"
#include <cmath>

namespace session_cpp {

TEST_CASE("test_line_default_constructor") {
    Line line;
    REQUIRE(line.z1() == 1.0f);
    REQUIRE(line.name == "my_line");
}

TEST_CASE("test_line_constructor") {
    Line line(1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
    REQUIRE(line.x0() == 1.0f);
    REQUIRE(line.z1() == 6.0f);
}

TEST_CASE("test_line_from_points") {
    Point p1(1.0f, 2.0f, 3.0f);
    Point p2(4.0f, 5.0f, 6.0f);
    Line line = Line::from_points(p1, p2);
    REQUIRE(line.y0() == 2.0f);
    REQUIRE(line.y1() == 5.0f);
}

TEST_CASE("test_line_with_name") {
    Line line = Line::with_name("custom", 0.0f, 0.0f, 0.0f, 1.0f, 1.0f, 1.0f);
    REQUIRE(line.name == "custom");
}

TEST_CASE("test_line_to_string") {
    Line line(1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
    std::string s = line.to_string();
    REQUIRE(s.find("1") != std::string::npos);
}

TEST_CASE("test_line_operator_subscript") {
    Line line(1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
    REQUIRE(line[0] == 1.0f);
    REQUIRE(line[5] == 6.0f);
}

TEST_CASE("test_line_operator_subscript_mutable") {
    Line line;
    line[0] = 10.0f;
    REQUIRE(line.x0() == 10.0f);
}

TEST_CASE("test_line_operator_add_assign") {
    Line line(0.0f, 0.0f, 0.0f, 1.0f, 1.0f, 1.0f);
    Vector v(1.0f, 2.0f, 3.0f);
    line += v;
    REQUIRE(line.x0() == 1.0f);
    REQUIRE(line.z1() == 4.0f);
}

TEST_CASE("test_line_operator_sub_assign") {
    Line line(1.0f, 2.0f, 3.0f, 2.0f, 3.0f, 4.0f);
    Vector v(1.0f, 2.0f, 3.0f);
    line -= v;
    REQUIRE(line.x0() == 0.0f);
}

TEST_CASE("test_line_operator_mul_assign") {
    Line line(1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
    line *= 2.0f;
    REQUIRE(line.x0() == 2.0f);
    REQUIRE(line.z1() == 12.0f);
}

TEST_CASE("test_line_operator_div_assign") {
    Line line(2.0f, 4.0f, 6.0f, 8.0f, 10.0f, 12.0f);
    line /= 2.0f;
    REQUIRE(line.x0() == 1.0f);
    REQUIRE(line.z1() == 6.0f);
}

TEST_CASE("test_line_operator_add") {
    Line line(0.0f, 0.0f, 0.0f, 1.0f, 1.0f, 1.0f);
    Vector v(1.0f, 2.0f, 3.0f);
    Line result = line + v;
    REQUIRE(result.y0() == 2.0f);
}

TEST_CASE("test_line_operator_sub") {
    Line line(1.0f, 2.0f, 3.0f, 2.0f, 3.0f, 4.0f);
    Vector v(1.0f, 2.0f, 3.0f);
    Line result = line - v;
    REQUIRE(result.x0() == 0.0f);
}

TEST_CASE("test_line_operator_mul") {
    Line line(1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
    Line result = line * 2.0f;
    REQUIRE(result.x0() == 2.0f);
}

TEST_CASE("test_line_operator_div") {
    Line line(2.0f, 4.0f, 6.0f, 8.0f, 10.0f, 12.0f);
    Line result = line / 2.0f;
    REQUIRE(result.z1() == 6.0f);
}

TEST_CASE("test_line_to_vector") {
    Line line(1.0f, 2.0f, 3.0f, 4.0f, 6.0f, 9.0f);
    Vector v = line.to_vector();
    REQUIRE(v.x() == 3.0f);
    REQUIRE(v.z() == 6.0f);
}

TEST_CASE("test_line_length") {
    Line line(0.0f, 0.0f, 0.0f, 3.0f, 4.0f, 0.0f);
    REQUIRE(std::abs(line.length() - 5.0f) < 1e-5f);
}

TEST_CASE("test_line_squared_length") {
    Line line(0.0f, 0.0f, 0.0f, 3.0f, 4.0f, 0.0f);
    REQUIRE(std::abs(line.squared_length() - 25.0f) < 1e-5f);
}

TEST_CASE("test_line_point_at") {
    Line line(0.0f, 0.0f, 0.0f, 10.0f, 10.0f, 10.0f);
    Point p = line.point_at(0.5f);
    REQUIRE(std::abs(p.x() - 5.0f) < 1e-5f);
}

TEST_CASE("test_line_start") {
    Line line(1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
    Point p = line.start();
    REQUIRE(p.x() == 1.0f);
}

TEST_CASE("test_line_end") {
    Line line(1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
    Point p = line.end();
    REQUIRE(p.x() == 4.0f);
}

TEST_CASE("test_line_to_json_data") {
    Line line(1.5f, 2.5f, 3.5f, 4.5f, 5.5f, 6.5f);
    line.name = "test";
    auto data = line.to_json_data();
    REQUIRE(data["type"] == "Line");
    REQUIRE(data["name"] == "test");
}

TEST_CASE("test_line_from_json_data") {
    Line orig(1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
    orig.name = "loaded";
    auto data = orig.to_json_data();
    Line restored = Line::from_json_data(data);
    REQUIRE(restored.name == "loaded");
    REQUIRE(restored.x0() == 1.0f);
}

TEST_CASE("test_line_to_json_from_json") {
    Line orig(1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f);
    orig.name = "serialized";
    std::string filepath = "../test_line.json";
    orig.to_json(filepath);
    Line loaded = Line::from_json(filepath);
    REQUIRE(loaded.name == orig.name);
    REQUIRE(loaded.x0() == orig.x0());
    REQUIRE(loaded.z1() == orig.z1());
}

}  // namespace session_cpp
