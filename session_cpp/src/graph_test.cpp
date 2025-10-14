#include "catch_amalgamated.hpp"
#include "graph.h"
#include "encoders.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Graph JSON roundtrip", "[graph]") {
    Graph original("test_graph");
    original.add_node("node1", "Node 1");
    original.add_node("node2", "Node 2");
    original.add_edge("node1", "node2", "edge1");
    
    
    encoders::json_dump(original, "test_graph.json");
    Graph loaded = encoders::json_load<Graph>("test_graph.json");

    encoders::json_dump(original, "test_graph.json");
    
    REQUIRE(loaded.number_of_vertices() == 2);
    REQUIRE(loaded.number_of_edges() == 1);
    REQUIRE(loaded.has_edge(std::make_tuple("node1", "node2")));}
