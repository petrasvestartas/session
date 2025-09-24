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

} // namespace session_cpp