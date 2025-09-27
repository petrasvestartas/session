#include "catch_amalgamated.hpp"
#include "color.h"
#include <filesystem>
#include <fstream>

namespace session_cpp {

TEST_CASE("Color constructor.") {
  Color color(255, 255, 100, 50);
  REQUIRE(color.r == 255);
  REQUIRE(color.g == 255);
  REQUIRE(color.b == 100);
  REQUIRE(color.a == 50);
  REQUIRE(!color.guid.empty());
}

TEST_CASE("Color equality.") {
  Color c1(0, 100, 50, 200);
  Color c2(0, 100, 50, 200);
  REQUIRE(c1 == c2);
  REQUIRE_FALSE(c1 != c2);
  Color c3(0, 100, 50, 200);
  Color c4(1, 100, 50, 200);
  REQUIRE_FALSE(c3 == c4);
  REQUIRE(c3 != c4);
}

TEST_CASE("Color to_json_data") {
  Color color(255, 128, 64, 255);
  auto data = color.to_json_data();
  REQUIRE(data["type"] == "Color");
  REQUIRE(data["name"] == "my_color");
  REQUIRE(data["r"] == 255);
  REQUIRE(data["g"] == 128);
  REQUIRE(data["b"] == 64);
  REQUIRE(data["a"] == 255);
}

TEST_CASE("Color from_json_data") {
  Color original(255, 128, 64, 255);
  auto data = original.to_json_data();
  Color restored = Color::from_json_data(data);
  REQUIRE(restored.r == 255);
  REQUIRE(restored.g == 128);
  REQUIRE(restored.b == 64);
  REQUIRE(restored.name == "my_color");
  REQUIRE(restored.a == 255);
  REQUIRE(restored.guid == original.guid);
}

TEST_CASE("Color to_json from_json") {
  Color original(123, 678, 999, 255);
  original.name = "file_test_color";
  std::string filename = "test_color.json";
  original.to_json(filename);
  Color loaded = Color::from_json(filename);
  REQUIRE(loaded.r == original.r);
  REQUIRE(loaded.g == original.g);
  REQUIRE(loaded.b == original.b);
  REQUIRE(loaded.a == original.a);
  // Note: File is kept for inspection (no std::filesystem::remove)
  REQUIRE(loaded.name == original.name);
  REQUIRE(loaded.guid == original.guid);
}

TEST_CASE("Color white") {
  Color white = Color::white();
  REQUIRE(white.name == "white");
  REQUIRE(white.r == 255);
  REQUIRE(white.g == 255);
  REQUIRE(white.b == 255);
  REQUIRE(white.a == 255);
}

TEST_CASE("Color black") {
  Color black = Color::black();
  REQUIRE(black.name == "black");
  REQUIRE(black.r == 0);
  REQUIRE(black.g == 0);
  REQUIRE(black.b == 0);
  REQUIRE(black.a == 255);
}

TEST_CASE("Color to_float_array") {
  Color color(255, 128, 64, 255);
  auto float_array = color.to_float_array();
  REQUIRE(float_array[0] == Catch::Approx(1.0));
  REQUIRE(float_array[1] == Catch::Approx(0.5019607843137255));
  REQUIRE(float_array[2] == Catch::Approx(0.25098039215686274));
  REQUIRE(float_array[3] == Catch::Approx(1.0));
}

TEST_CASE("Color from_float") {
  Color color = Color::from_float(1.0, 0.5, 0.25, 1.0);
  REQUIRE(color.r == 255);
  REQUIRE(color.g == 128); // 0.5 * 255 = 127.5, rounded to 128
  REQUIRE(color.b == 64);  // 0.25 * 255 = 63.75, rounded to 64
  REQUIRE(color.a == 255);
}

TEST_CASE("Color red") {
  Color red = Color::red();
  REQUIRE(red.name == "red");
  REQUIRE(red.r == 255);
  REQUIRE(red.g == 0);
  REQUIRE(red.b == 0);
  REQUIRE(red.a == 255);
}

TEST_CASE("Color green") {
  Color green = Color::green();
  REQUIRE(green.name == "green");
  REQUIRE(green.r == 0);
  REQUIRE(green.g == 255);
  REQUIRE(green.b == 0);
  REQUIRE(green.a == 255);
}

TEST_CASE("Color blue") {
  Color blue = Color::blue();
  REQUIRE(blue.name == "blue");
  REQUIRE(blue.r == 0);
  REQUIRE(blue.g == 0);
  REQUIRE(blue.b == 255);
  REQUIRE(blue.a == 255);
}

TEST_CASE("Color grey") {
  Color grey = Color::grey();
  REQUIRE(grey.name == "grey");
  REQUIRE(grey.r == 128);
  REQUIRE(grey.g == 128);
  REQUIRE(grey.b == 128);
  REQUIRE(grey.a == 255);
}

} // namespace session_cpp