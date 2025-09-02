#pragma once
#include <iostream>
#include <filesystem>
#include "../src/point.hpp"

void test_point_constructor() {
    session_cpp::Point p(1.0, 2.0, 3.0);
    my_assert(p.x == 1.0 && p.y == 2.0 && p.z == 3.0);
}

void test_point_to_string() {
    session_cpp::Point p(1.0, 2.0, 3.0);
    std::string str = p.to_string();
    my_assert(str.find("Point(1, 2, 3") != std::string::npos);
}

void test_point_to_json_data() {
    session_cpp::Point p(1.0, 2.0, 3.0);
    auto json = p.to_json_data();
    my_assert(json["x"] == 1.0 && json["y"] == 2.0 && json["z"] == 3.0);
    my_assert(json["dtype"] == "Point");
}

void test_point_json_roundtrip() {
    session_cpp::Point original(1.0, 2.0, 3.0);
    auto json_data = original.to_json_data();
    session_cpp::Point restored = session_cpp::Point::from_json_data(json_data);
    my_assert(restored.x == original.x && restored.y == original.y && restored.z == original.z);
}

void test_point_json_file() {
    session_cpp::Point p(1.0, 2.0, 3.0);
    std::string filepath = "test_point.json";
    p.to_json(filepath);
    session_cpp::Point loaded = session_cpp::Point::from_json(filepath);
    my_assert(loaded.x == 1.0 && loaded.y == 2.0 && loaded.z == 3.0);
    std::filesystem::remove(filepath);
}