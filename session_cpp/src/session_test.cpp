#include "catch_amalgamated.hpp"
#include "point.h"
#include "session.h"
#include <filesystem>

namespace session_cpp {

TEST_CASE("Session constructor.") {
  Session session;
  REQUIRE(session.name == "my_session");
  REQUIRE(!session.guid.empty());
  // Objects, tree, and graph are initialized by default constructor
}

TEST_CASE("Session to_json_data.") {
  Session session;
  auto point1 = std::make_shared<Point>(1.0, 2.0, 3.0);
  auto point2 = std::make_shared<Point>(4.0, 5.0, 6.0);
  session.add_point(point1);
  session.add_point(point2);
  session.add_edge(point1->guid, point2->guid, "connection");

  auto data = session.to_json_data();
  REQUIRE(data["name"] == "my_session");
  REQUIRE(data.contains("guid"));
  REQUIRE(data["objects"]["points"].size() == 2);
  REQUIRE(data["graph"]["vertices"].size() == 2);
  REQUIRE(data["graph"]["edges"].size() == 1);
}

TEST_CASE("Session from_json_data.") {
  Session session;
  auto point1 = std::make_shared<Point>(1.0, 2.0, 3.0);
  auto point2 = std::make_shared<Point>(4.0, 5.0, 6.0);
  session.add_point(point1);
  session.add_point(point2);
  session.add_edge(point1->guid, point2->guid, "connection");

  auto data = session.to_json_data();
  Session session2 = Session::from_json_data(data);
  REQUIRE(session2.name == "my_session");
  REQUIRE(session2.lookup.size() == 2);
  REQUIRE(session2.graph.number_of_vertices() == 2);
}

TEST_CASE("Session to_json and from_json file I/O.") {
  Session session;
  auto point1 = std::make_shared<Point>(1.0, 2.0, 3.0);
  auto point2 = std::make_shared<Point>(4.0, 5.0, 6.0);
  session.add_point(point1);
  session.add_point(point2);
  session.add_edge(point1->guid, point2->guid, "connection");
  std::string filename = "test_session.json";

  session.to_json(filename);
  Session loaded_session = Session::from_json(filename);

  REQUIRE(loaded_session.name == session.name);
  REQUIRE(loaded_session.lookup.size() == session.lookup.size());
  REQUIRE(loaded_session.graph.number_of_vertices() ==
          session.graph.number_of_vertices());

  std::filesystem::remove(filename);
}

TEST_CASE("Session add_point.") {
  Session session;
  auto point = std::make_shared<Point>(1.0, 2.0, 3.0);
  session.add_point(point);

  REQUIRE(session.objects.points->size() == 1);
  REQUIRE(session.lookup.count(point->guid) == 1);
  REQUIRE(session.graph.has_node(point->guid));
}

TEST_CASE("Session add_edge.") {
  Session session;
  auto point1 = std::make_shared<Point>(1.0, 2.0, 3.0);
  auto point2 = std::make_shared<Point>(4.0, 5.0, 6.0);
  session.add_point(point1);
  session.add_point(point2);
  session.add_edge(point1->guid, point2->guid, "connection");

  REQUIRE(session.graph.has_edge({point1->guid, point2->guid}));
}

TEST_CASE("Session get_object.") {
  Session session;
  auto point = std::make_shared<Point>(1.0, 2.0, 3.0);
  session.add_point(point);

  auto retrieved = session.get_object<Point>(point->guid);
  REQUIRE(retrieved != nullptr);
  REQUIRE(retrieved->guid == point->guid);
}

TEST_CASE("Session to_json_file.") {
  Session session("test_session");
  auto point1 = std::make_shared<Point>(1.0, 2.0, 3.0);
  auto point2 = std::make_shared<Point>(4.0, 5.0, 6.0);
  session.add_point(point1);
  session.add_point(point2);
  session.add_edge(point1->guid, point2->guid, "test_connection");
  std::string filename = "test_session.json";

  session.to_json(filename);
  Session loaded_session = Session::from_json(filename);

  REQUIRE(loaded_session.name == session.name);
  REQUIRE(loaded_session.objects.points->size() ==
          session.objects.points->size());
  REQUIRE(loaded_session.graph.number_of_vertices() ==
          session.graph.number_of_vertices());
  REQUIRE(loaded_session.graph.number_of_edges() ==
          session.graph.number_of_edges());

  // std::filesystem::remove(filename);
}

} // namespace session_cpp