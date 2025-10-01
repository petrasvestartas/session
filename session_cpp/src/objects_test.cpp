#include "catch_amalgamated.hpp"
#include "objects.h"
#include <algorithm>
#include <filesystem>
#include <fstream>

namespace session_cpp {

TEST_CASE("Objects constructor.") {
  Objects objects;
  REQUIRE(objects.name == "my_objects");
  REQUIRE_FALSE(objects.guid.empty());
  REQUIRE(objects.points);
  REQUIRE(objects.points->empty());
}

TEST_CASE("Objects to_json_data.") {
  Objects objects;
  // add three points
  objects.points->push_back(std::make_shared<Point>(1.0, 2.0, 3.0));
  objects.points->push_back(std::make_shared<Point>(4.0, 5.0, 6.0));
  objects.points->push_back(std::make_shared<Point>(7.0, 8.0, 9.0));

  auto data = objects.to_json_data();
  REQUIRE(data["name"] == "my_objects");
  REQUIRE(data.contains("guid"));
  REQUIRE(data["points"].size() == 3);
  REQUIRE(data["points"][0]["x"] == 1.0);
  REQUIRE(data["points"][1]["y"] == 5.0);
}

TEST_CASE("Objects from_json_data.") {
  // Build JSON similar to to_json_data structure
  nlohmann::ordered_json j;
  j["name"] = "my_objects";
  j["guid"] = ::guid();
  j["type"] = "Objects";
  j["points"] = nlohmann::ordered_json::array();
  j["points"].push_back(Point(1.0, 2.0, 3.0).to_json_data());
  j["points"].push_back(Point(4.0, 5.0, 6.0).to_json_data());
  j["points"].push_back(Point(7.0, 8.0, 9.0).to_json_data());

  auto objects = Objects::from_json_data(j);
  REQUIRE(objects.name == "my_objects");
  REQUIRE(objects.points);
  REQUIRE(objects.points->size() == 3);
  REQUIRE(objects.points->at(0)->x() == 1.0);
  REQUIRE(objects.points->at(1)->y() == 5.0);
}

TEST_CASE("Objects to_json_from_json.") {
  // Prepare an Objects instance
  Objects objects;
  objects.points->push_back(std::make_shared<Point>(1.0, 2.0, 3.0));
  objects.points->push_back(std::make_shared<Point>(4.0, 5.0, 6.0));
  objects.points->push_back(std::make_shared<Point>(7.0, 8.0, 9.0));

  // Save to a temporary file, then load back
  std::string filename = "test_objects.json";
  objects.to_json(filename);
  auto loaded = Objects::from_json(filename);

  REQUIRE(loaded.name == objects.name);
  REQUIRE(loaded.points);
  REQUIRE(loaded.points->size() == 3);
  REQUIRE(loaded.points->at(0)->x() == 1.0);
  REQUIRE(loaded.points->at(1)->y() == 5.0);
}

} // namespace session_cpp