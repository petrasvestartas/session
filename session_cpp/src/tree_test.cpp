#include "catch_amalgamated.hpp"
#include "tree.h"
#include "encoders.h"
#include "point.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("Tree JSON roundtrip", "[tree]") {
    Tree original("test_tree");
    auto point1 = std::make_shared<Point>(1.0, 2.0, 3.0);
    auto node1 = std::make_shared<TreeNode>(point1->guid);
    original.add(node1, nullptr);
    
    
    encoders::json_dump(original, "test_tree.json");
    Tree loaded = encoders::json_load<Tree>("test_tree.json");

    encoders::json_dump(original, "test_tree.json");
    
    REQUIRE(loaded.name == original.name);
    REQUIRE(loaded.nodes().size() == original.nodes().size());}
