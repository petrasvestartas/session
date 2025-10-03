#include "catch_amalgamated.hpp"
#include "graph.h"
#include "point.h"
#include <algorithm>
#include <filesystem>
#include <fstream>

namespace session_cpp {

// Test Graph constructor.
TEST_CASE("Graph constructor.") {
  Graph graph("my_graph");
  REQUIRE(graph.name == "my_graph");
  REQUIRE(graph.guid != "");
}

// Test Graph add_node method.
TEST_CASE("Graph add_node.") {
  Graph graph;
  auto result = graph.add_node("node1", "attribute_data");
  REQUIRE(result == std::string("node1"));
  REQUIRE(graph.has_node("node1"));
}

// Test Graph add_edge method.
TEST_CASE("Graph add_edge.") {
  Graph graph;
  auto result = graph.add_edge("node1", "node2", "edge_data");
  REQUIRE(std::get<0>(result) == std::string("node1"));
  REQUIRE(std::get<1>(result) == std::string("node2"));
  REQUIRE(graph.has_edge(std::make_tuple("node1", "node2")));
}

// Test Graph has_node method.
TEST_CASE("Graph has_node.") {
  Graph graph;
  graph.add_node("node1");
  REQUIRE(graph.has_node("node1"));
  REQUIRE_FALSE(graph.has_node("node2"));
}

// Test Graph has_edge method.
TEST_CASE("Graph has_edge.") {
  Graph graph;
  graph.add_edge("A", "B", "edge_attr");
  REQUIRE(graph.has_edge(std::make_tuple("A", "B")));
  REQUIRE_FALSE(graph.has_edge(std::make_tuple("C", "D")));
}

// Test Graph remove_node method.
TEST_CASE("Graph remove_node.") {
  Graph graph;
  graph.add_node("node1");
  graph.remove_node("node1");
  REQUIRE_FALSE(graph.has_node("node1"));
}

// Test Graph remove_edge method.
TEST_CASE("Graph remove_edge.") {
  Graph graph;
  graph.add_edge("A", "B", "edge_attr");
  graph.remove_edge(std::make_tuple("A", "B"));
  REQUIRE_FALSE(graph.has_edge(std::make_tuple("A", "B")));
}

// Test Graph vertices method.
TEST_CASE("Graph vertices.") {
  Graph graph;
  graph.add_node("node1", "node_data");
  auto verts = graph.get_vertices();
  REQUIRE(verts.size() == 1);
  REQUIRE(verts[0].name == std::string("node1"));
}

// Test Graph edges method.
TEST_CASE("Graph edges.") {
  Graph graph;
  graph.add_edge("node1", "node2", "edge_data");
  auto e = graph.get_edges();
  REQUIRE(e.size() == 1);
  auto &edge = e[0];
  REQUIRE(((std::get<0>(edge) == std::string("node1") &&
            std::get<1>(edge) == std::string("node2")) ||
           (std::get<0>(edge) == std::string("node2") &&
            std::get<1>(edge) == std::string("node1"))));
}

// Test Graph neighbors method.
TEST_CASE("Graph neighbors.") {
  Graph graph;
  graph.add_edge("A", "B", "edge1");
  graph.add_edge("A", "C", "edge2");
  auto nb = graph.neighbors("A");
  std::sort(nb.begin(), nb.end());
  REQUIRE(nb == std::vector<std::string>{"B", "C"});
}

// Test Graph number_of_vertices method.
TEST_CASE("Graph number_of_vertices.") {
  Graph graph;
  graph.add_node("node1");
  REQUIRE(graph.number_of_vertices() == 1);
}

// Test Graph number_of_edges method.
TEST_CASE("Graph number_of_edges.") {
  Graph graph;
  graph.add_edge("node1", "node2");
  REQUIRE(graph.number_of_edges() == 1);
}

// Test Graph clear method.
TEST_CASE("Graph clear.") {
  Graph graph;
  graph.add_node("node1");
  graph.clear();
  REQUIRE(graph.number_of_vertices() == 0);
  REQUIRE(graph.number_of_edges() == 0);
}

// Test Graph node_attribute method.
TEST_CASE("Graph node_attribute.") {
  Graph graph;
  graph.add_node("node1", "initial_data");
  REQUIRE(graph.node_attribute("node1") == std::string("initial_data"));
  graph.node_attribute("node1", "new_data");
  REQUIRE(graph.node_attribute("node1") == std::string("new_data"));
}

// Test Graph edge_attribute method.
TEST_CASE("Graph edge_attribute.") {
  Graph graph("test_graph");
  graph.add_edge("node1", "node2", "edge_data");
  REQUIRE(graph.edge_attribute("node1", "node2") == std::string("edge_data"));
  graph.edge_attribute("node1", "node2", "new_data");
  REQUIRE(graph.edge_attribute("node1", "node2") == std::string("new_data"));
}

// Test Graph file I/O with to_json and from_json.
TEST_CASE("Graph to_json from_json.") {
  Graph graph("my_graph");
  graph.add_node("A", "vertex_A");
  graph.add_node("B", "vertex_B");
  graph.add_node("C", "vertex_C");
  graph.add_node("D", "vertex_D");
  graph.add_edge("A", "B", "edge_AB");
  graph.add_edge("B", "C", "edge_BC");
  graph.add_edge("C", "D", "edge_CD");
  std::string filename = "../test_graph.json";
  graph.to_json(filename);
  Graph loaded = Graph::from_json(filename);
  REQUIRE(loaded.name == graph.name);
  REQUIRE(loaded.number_of_vertices() == graph.number_of_vertices());
  REQUIRE(loaded.number_of_edges() == graph.number_of_edges());
}

// Test Graph from_json_data method.
TEST_CASE("Graph from_json_data.") {
  nlohmann::json data;
  data["type"] = "Graph";
  data["name"] = "test_graph";
  data["guid"] = "test-guid-123";
  data["vertex_count"] = 3;
  data["edge_count"] = 2;

  // vertices with required fields for Vertex::from_json_data
  data["vertices"] = nlohmann::json::array();
  data["vertices"].push_back({{"type", "Vertex"},
                              {"name", "node1"},
                              {"guid", "v-guid-1"},
                              {"attribute", "type:start"},
                              {"index", 0}});
  data["vertices"].push_back({{"type", "Vertex"},
                              {"name", "node2"},
                              {"guid", "v-guid-2"},
                              {"attribute", "type:middle"},
                              {"index", 1}});
  data["vertices"].push_back({{"type", "Vertex"},
                              {"name", "node3"},
                              {"guid", "v-guid-3"},
                              {"attribute", "type:end"},
                              {"index", 2}});

  // edges with required fields for Edge::from_json_data
  data["edges"] = nlohmann::json::array();
  data["edges"].push_back({{"type", "Edge"},
                           {"name", "my_edge"},
                           {"guid", "e-guid-1"},
                           {"v0", "node1"},
                           {"v1", "node2"},
                           {"attribute", "weight:10"},
                           {"index", 0}});
  data["edges"].push_back({{"type", "Edge"},
                           {"name", "my_edge"},
                           {"guid", "e-guid-2"},
                           {"v0", "node2"},
                           {"v1", "node3"},
                           {"attribute", "weight:20"},
                           {"index", 1}});

  Graph graph = Graph::from_json_data(data);
  REQUIRE(graph.name == std::string("test_graph"));
  REQUIRE(graph.number_of_vertices() == 3);
  REQUIRE(graph.number_of_edges() == 2);
  REQUIRE(graph.has_node("node1"));
  REQUIRE(graph.has_node("node2"));
  REQUIRE(graph.has_node("node3"));
  REQUIRE(graph.has_edge(std::make_tuple("node1", "node2")));
  REQUIRE(graph.has_edge(std::make_tuple("node2", "node3")));
  REQUIRE(graph.node_attribute("node1") == std::string("type:start"));
  REQUIRE(graph.edge_attribute("node1", "node2") == std::string("weight:10"));
}
} // namespace session_cpp