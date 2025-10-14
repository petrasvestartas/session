#include "catch_amalgamated.hpp"
#include "treenode.h"
#include "encoders.h"

using namespace session_cpp;

TEST_CASE("TreeNode JSON roundtrip", "[treenode]") {
    auto original = std::make_shared<TreeNode>("test_node");
    auto child = std::make_shared<TreeNode>("child_node");
    original->add(child);
    
    encoders::json_dump(original->jsondump(), "test_treenode.json");
    auto loaded_json = encoders::json_load_data("test_treenode.json");
    auto loaded = TreeNode::jsonload(loaded_json);
    
    REQUIRE(loaded->name == original->name);
}
