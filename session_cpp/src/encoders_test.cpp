#include "catch_amalgamated.hpp"
#include "encoders.h"
#include "point.h"
#include "vector.h"
#include "line.h"
#include <filesystem>

using namespace session_cpp;
using namespace session_cpp::encoders;

TEST_CASE("Encoders json_dump and json_load", "[encoders]") {
    Point original(1.5, 2.5, 3.5);
    original.name = "test_point";
    
    std::string filepath = "test_encoders_point.json";
    json_dump(original, filepath);
    
    Point loaded = json_load<Point>(filepath);
    
    REQUIRE(loaded.x() == original.x());
    REQUIRE(loaded.y() == original.y());
    REQUIRE(loaded.z() == original.z());
    REQUIRE(loaded.name == original.name);
    
    std::filesystem::remove(filepath);
}

TEST_CASE("Encoders json_dumps and json_loads", "[encoders]") {
    Vector original(42.1, 84.2, 126.3);
    original.name = "test_vector";
    
    std::string json_str = json_dumps(original);
    REQUIRE(!json_str.empty());
    REQUIRE(json_str.find("Vector") != std::string::npos);
    
    Vector loaded = json_loads<Vector>(json_str);
    
    REQUIRE(loaded.x() == Catch::Approx(original.x()));
    REQUIRE(loaded.y() == Catch::Approx(original.y()));
    REQUIRE(loaded.z() == Catch::Approx(original.z()));
    REQUIRE(loaded.name == original.name);
}

TEST_CASE("Encoders encode_collection with values", "[encoders]") {
    std::vector<Point> points;
    points.push_back(Point(1.0, 2.0, 3.0));
    points.push_back(Point(4.0, 5.0, 6.0));
    points.push_back(Point(7.0, 8.0, 9.0));
    
    auto json_arr = encode_collection(points);
    
    REQUIRE(json_arr.is_array());
    REQUIRE(json_arr.size() == 3);
    REQUIRE(json_arr[0]["type"] == "Point");
    REQUIRE(json_arr[1]["x"] == 4.0);
    REQUIRE(json_arr[2]["z"] == 9.0);
}

TEST_CASE("Encoders encode_collection with shared_ptr", "[encoders]") {
    std::vector<std::shared_ptr<Line>> lines;
    lines.push_back(std::make_shared<Line>(0.0, 0.0, 0.0, 1.0, 0.0, 0.0));
    lines.push_back(std::make_shared<Line>(0.0, 0.0, 0.0, 0.0, 1.0, 0.0));
    
    auto json_arr = encode_collection(lines);
    
    REQUIRE(json_arr.is_array());
    REQUIRE(json_arr.size() == 2);
    REQUIRE(json_arr[0]["type"] == "Line");
    REQUIRE(json_arr[1]["type"] == "Line");
}

TEST_CASE("Encoders decode_collection", "[encoders]") {
    std::vector<Point> original_points;
    original_points.push_back(Point(1.0, 2.0, 3.0));
    original_points.push_back(Point(4.0, 5.0, 6.0));
    
    auto json_arr = encode_collection(original_points);
    auto decoded_points = decode_collection<Point>(json_arr);
    
    REQUIRE(decoded_points.size() == 2);
    REQUIRE(decoded_points[0].x() == 1.0);
    REQUIRE(decoded_points[1].y() == 5.0);
}

TEST_CASE("Encoders decode_collection_ptr", "[encoders]") {
    std::vector<std::shared_ptr<Vector>> original_vectors;
    original_vectors.push_back(std::make_shared<Vector>(1.0, 0.0, 0.0));
    original_vectors.push_back(std::make_shared<Vector>(0.0, 1.0, 0.0));
    
    auto json_arr = encode_collection(original_vectors);
    auto decoded_vectors = decode_collection_ptr<Vector>(json_arr);
    
    REQUIRE(decoded_vectors.size() == 2);
    REQUIRE(decoded_vectors[0]->x() == Catch::Approx(1.0));
    REQUIRE(decoded_vectors[1]->y() == Catch::Approx(1.0));
}

TEST_CASE("Encoders nested_collections", "[encoders]") {
    std::vector<Line> lines;
    lines.push_back(Line(0.0, 0.0, 0.0, 1.0, 0.0, 0.0));
    lines.push_back(Line(0.0, 0.0, 0.0, 0.0, 1.0, 0.0));
    
    auto json_arr = encode_collection(lines);
    std::string json_str = json_arr.dump();
    REQUIRE(!json_str.empty());
    
    auto loaded_json = nlohmann::json::parse(json_str);
    auto loaded = decode_collection<Line>(loaded_json);
    
    REQUIRE(loaded.size() == 2);
    REQUIRE(loaded[0].end().x() == Catch::Approx(1.0));
    REQUIRE(loaded[1].end().y() == Catch::Approx(1.0));
}

TEST_CASE("Encoders roundtrip with file I/O", "[encoders]") {
    std::vector<Vector> vectors;
    vectors.push_back(Vector(1.0, 0.0, 0.0));
    vectors.push_back(Vector(0.0, 1.0, 0.0));
    vectors.push_back(Vector(0.0, 0.0, 1.0));
    
    std::string filepath = "test_encoders_collection.json";
    auto json_arr = encode_collection(vectors);
    json_dump(json_arr, filepath);
    
    auto loaded_json = json_load_data(filepath);
    auto decoded_vectors = decode_collection<Vector>(loaded_json);
    
    REQUIRE(decoded_vectors.size() == 3);
    REQUIRE(decoded_vectors[0].x() == Catch::Approx(1.0));
    REQUIRE(decoded_vectors[1].y() == Catch::Approx(1.0));
    REQUIRE(decoded_vectors[2].z() == Catch::Approx(1.0));
    
    std::filesystem::remove(filepath);
}

TEST_CASE("Encoders pretty vs compact", "[encoders]") {
    Point point(1.0, 2.0, 3.0);
    
    std::string pretty = json_dumps(point, true);
    std::string compact = json_dumps(point, false);
    
    REQUIRE(pretty.length() > compact.length());
    REQUIRE(pretty.find("\n") != std::string::npos);
    REQUIRE(compact.find("\n") == std::string::npos);
    
    Point loaded_pretty = json_loads<Point>(pretty);
    Point loaded_compact = json_loads<Point>(compact);
    
    REQUIRE(loaded_pretty.x() == 1.0);
    REQUIRE(loaded_compact.x() == 1.0);
}

TEST_CASE("Encoders decode primitives", "[encoders]") {
    // Test that primitives serialize/deserialize correctly
    nlohmann::json num = 42;
    std::string json_str = num.dump();
    auto loaded = nlohmann::json::parse(json_str);
    REQUIRE(loaded.get<int>() == 42);
    
    nlohmann::json float_val = 3.14;
    json_str = float_val.dump();
    loaded = nlohmann::json::parse(json_str);
    REQUIRE(loaded.get<double>() == Catch::Approx(3.14));
    
    nlohmann::json text = "hello";
    json_str = text.dump();
    loaded = nlohmann::json::parse(json_str);
    REQUIRE(loaded.get<std::string>() == "hello");
    
    nlohmann::json flag = true;
    json_str = flag.dump();
    loaded = nlohmann::json::parse(json_str);
    REQUIRE(loaded.get<bool>() == true);
}

TEST_CASE("Encoders decode list", "[encoders]") {
    // Test list serialization
    std::vector<int> data = {1, 2, 3};
    nlohmann::json j = data;
    std::string json_str = j.dump();
    auto loaded = nlohmann::json::parse(json_str);
    auto loaded_vec = loaded.get<std::vector<int>>();
    REQUIRE(loaded_vec.size() == 3);
    REQUIRE(loaded_vec[0] == 1);
    REQUIRE(loaded_vec[2] == 3);
    
    // List with geometry objects
    std::vector<Point> points;
    points.push_back(Point(1.0, 2.0, 3.0));
    points.push_back(Point(4.0, 5.0, 6.0));
    
    auto json_arr = encode_collection(points);
    auto decoded = decode_collection<Point>(json_arr);
    REQUIRE(decoded.size() == 2);
    REQUIRE(decoded[0].x() == 1.0);
    REQUIRE(decoded[1].x() == 4.0);
}

TEST_CASE("Encoders decode dict", "[encoders]") {
    // Plain dict
    std::map<std::string, int> data;
    data["a"] = 1;
    data["b"] = 2;
    nlohmann::json j = data;
    std::string json_str = j.dump();
    auto loaded = nlohmann::json::parse(json_str);
    REQUIRE(loaded["a"].get<int>() == 1);
    REQUIRE(loaded["b"].get<int>() == 2);
    
    // Geometry object (has type field)
    Vector vec(1.0, 2.0, 3.0);
    std::string vec_json = json_dumps(vec);
    Vector loaded_vec = json_loads<Vector>(vec_json);
    REQUIRE(loaded_vec.x() == Catch::Approx(1.0));
}

TEST_CASE("Encoders list in list in list", "[encoders]") {
    nlohmann::json data = {{{1, 2}, {3, 4}}, {{5, 6}, {7, 8}}};
    std::string json_str = data.dump();
    auto loaded = nlohmann::json::parse(json_str);
    
    REQUIRE(loaded[0][0][0] == 1);
    REQUIRE(loaded[1][1][1] == 8);
    REQUIRE(loaded.size() == 2);
}

TEST_CASE("Encoders dict of lists", "[encoders]") {
    std::vector<Point> points;
    points.push_back(Point(1.0, 0.0, 0.0));
    points.push_back(Point(0.0, 1.0, 0.0));
    
    nlohmann::json data;
    data["numbers"] = {1, 2, 3};
    data["letters"] = {"a", "b", "c"};
    data["points"] = encode_collection(points);
    
    std::string json_str = data.dump();
    auto loaded = nlohmann::json::parse(json_str);
    
    REQUIRE(loaded["numbers"].size() == 3);
    REQUIRE(loaded["letters"][0] == "a");
    auto loaded_points = decode_collection<Point>(loaded["points"]);
    REQUIRE(loaded_points.size() == 2);
    REQUIRE(loaded_points[0].x() == 1.0);
}

TEST_CASE("Encoders list of dict", "[encoders]") {
    Point point(1.0, 2.0, 3.0);
    
    nlohmann::json data = nlohmann::json::array();
    data.push_back({{"name", "point1"}, {"value", 10}});
    data.push_back({{"name", "point2"}, {"value", 20}});
    data.push_back({{"geometry", point.jsondump()}});
    
    std::string json_str = data.dump();
    auto loaded = nlohmann::json::parse(json_str);
    
    REQUIRE(loaded.size() == 3);
    REQUIRE(loaded[0]["name"] == "point1");
    REQUIRE(loaded[1]["value"] == 20);
    Point loaded_point = Point::jsonload(loaded[2]["geometry"]);
    REQUIRE(loaded_point.z() == Catch::Approx(3.0));
}

TEST_CASE("Encoders dict of dicts", "[encoders]") {
    Point point(1.0, 2.0, 3.0);
    Vector vec(0.0, 0.0, 1.0);
    
    nlohmann::json data;
    data["config"]["tolerance"] = 0.001;
    data["config"]["scale"] = 1000;
    data["geometry"]["point"] = point.jsondump();
    data["geometry"]["vector"] = vec.jsondump();
    
    std::string json_str = data.dump();
    auto loaded = nlohmann::json::parse(json_str);
    
    REQUIRE(loaded["config"]["tolerance"] == Catch::Approx(0.001));
    REQUIRE(loaded["config"]["scale"] == 1000);
    Point loaded_point = Point::jsonload(loaded["geometry"]["point"]);
    Vector loaded_vec = Vector::jsonload(loaded["geometry"]["vector"]);
    REQUIRE(loaded_point.x() == Catch::Approx(1.0));
    REQUIRE(loaded_vec.z() == Catch::Approx(1.0));
}
