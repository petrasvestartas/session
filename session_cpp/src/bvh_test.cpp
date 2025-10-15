#include "catch_amalgamated.hpp"
#include "bvh.h"
#include <random>
#include <chrono>
#include <cmath>

using namespace session_cpp;

TEST_CASE("Expand bits for Morton codes", "[bvh]") {
    // Test bit expansion for Morton codes
    REQUIRE(expand_bits(0) == 0);
    REQUIRE(expand_bits(1) == 1);
    REQUIRE(expand_bits(2) == 8);
    REQUIRE(expand_bits(3) == 9);
    
    // 1023 in binary is 0b1111111111 (10 bits)
    // After expansion, should have pattern with zeros inserted
    uint32_t result = expand_bits(1023);
    REQUIRE(result > 0);  // Should be non-zero
}

TEST_CASE("Morton code at origin", "[bvh]") {
    uint32_t code = calculate_morton_code(0.0f, 0.0f, 0.0f, 100.0f);
    REQUIRE(code < (1u << 30)); // 30-bit code
}

TEST_CASE("Morton codes at corners", "[bvh]") {
    float world_size = 100.0f;

    // Corner at (-50, -50, -50) should give code 0
    uint32_t code_min = calculate_morton_code(-50.0f, -50.0f, -50.0f, world_size);
    REQUIRE(code_min == 0);

    // Corner at (50, 50, 50) should give maximum code
    uint32_t code_max = calculate_morton_code(50.0f, 50.0f, 50.0f, world_size);
    REQUIRE(code_max == 0x3FFFFFFF);  // Maximum 30-bit value
}

TEST_CASE("Morton code spatial locality", "[bvh]") {
    // Two nearby points should have similar codes
    uint32_t code1 = calculate_morton_code(10.0f, 10.0f, 10.0f);
    uint32_t code2 = calculate_morton_code(10.1f, 10.1f, 10.1f);

    // Two far apart points should have different codes
    uint32_t code3 = calculate_morton_code(-40.0f, -40.0f, -40.0f);

    // Nearby points should be closer in Morton space
    uint32_t diff_nearby = (code1 > code2) ? (code1 - code2) : (code2 - code1);
    uint32_t diff_far = (code1 > code3) ? (code1 - code3) : (code3 - code1);
    REQUIRE(diff_nearby < diff_far);
}

TEST_CASE("BVH node creation", "[bvh]") {
    BVHNode node;
    REQUIRE(!node.guid.empty());
    REQUIRE(node.left == nullptr);
    REQUIRE(node.right == nullptr);
    REQUIRE(node.object_id == -1);
    REQUIRE(!node.is_leaf());
}

TEST_CASE("BVH node leaf", "[bvh]") {
    BVHNode node;
    REQUIRE(!node.is_leaf());

    node.object_id = 5;
    REQUIRE(node.is_leaf());
}

TEST_CASE("BVH creation", "[bvh]") {
    BVH bvh(100.0f);
    REQUIRE(!bvh.guid.empty());
    REQUIRE(bvh.name == "my_bvh");
    REQUIRE(bvh.root == nullptr);
    REQUIRE(bvh.world_size == 100.0f);
}

TEST_CASE("BVH build empty", "[bvh]") {
    std::vector<BoundingBox> boxes;
    BVH bvh = BVH::from_boxes(boxes, 100.0f);
    REQUIRE(bvh.root == nullptr);
}

TEST_CASE("BVH build single", "[bvh]") {
    BoundingBox bbox(Point(0, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1));
    std::vector<BoundingBox> boxes = {bbox};
    
    BVH bvh = BVH::from_boxes(boxes, 100.0f);
    
    REQUIRE(bvh.root != nullptr);
    REQUIRE(bvh.root->is_leaf());
    REQUIRE(bvh.root->object_id == 0);
}

TEST_CASE("BVH build multiple", "[bvh]") {
    std::vector<BoundingBox> bboxes = {
        BoundingBox(Point(-10, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1)),
        BoundingBox(Point(10, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1)),
        BoundingBox(Point(0, 10, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1))
    };

    BVH bvh = BVH::from_boxes(bboxes, 100.0f);

    REQUIRE(bvh.root != nullptr);
    REQUIRE(!bvh.root->is_leaf());
    REQUIRE(bvh.root->left != nullptr);
    REQUIRE(bvh.root->right != nullptr);
}

TEST_CASE("BVH AABB intersect", "[bvh]") {
    BVH bvh(100.0f);

    // Overlapping boxes
    BoundingBox bbox1(Point(0, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1));
    BoundingBox bbox2(Point(0.5f, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1));
    REQUIRE(bvh.aabb_intersect(bbox1, bbox2));

    // Non-overlapping boxes
    BoundingBox bbox3(Point(10, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1));
    REQUIRE(!bvh.aabb_intersect(bbox1, bbox3));
}

TEST_CASE("BVH find collisions no collision", "[bvh]") {
    std::vector<BoundingBox> bboxes = {
        BoundingBox(Point(-10, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1)),
        BoundingBox(Point(10, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1))
    };

    BVH bvh = BVH::from_boxes(bboxes, 100.0f);

    auto [collisions, checks] = bvh.find_collisions(0, bboxes[0], bboxes);
    REQUIRE(collisions.size() == 0);
    REQUIRE(checks > 0);
}

TEST_CASE("BVH find collisions with collision", "[bvh]") {
    std::vector<BoundingBox> bboxes = {
        BoundingBox(Point(0, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(2, 2, 2)),
        BoundingBox(Point(1, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(2, 2, 2))
    };

    BVH bvh = BVH::from_boxes(bboxes, 100.0f);

    auto [collisions, checks] = bvh.find_collisions(0, bboxes[0], bboxes);
    REQUIRE(collisions.size() == 1);
    REQUIRE(collisions[0] == 1);
}

TEST_CASE("BVH check all collisions", "[bvh]") {
    std::vector<BoundingBox> bboxes = {
        BoundingBox(Point(0, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1)),
        BoundingBox(Point(0.5f, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1)),
        BoundingBox(Point(10, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1))
    };

    BVH bvh = BVH::from_boxes(bboxes, 100.0f);

    auto [collisions, colliding_indices, checks] = bvh.check_all_collisions(bboxes);

    // Boxes 0 and 1 should collide
    REQUIRE(collisions.size() == 1);
    REQUIRE(collisions[0].first == 0);
    REQUIRE(collisions[0].second == 1);
    REQUIRE(colliding_indices.size() == 2);
    REQUIRE(colliding_indices[0] == 0);
    REQUIRE(colliding_indices[1] == 1);
    REQUIRE(checks > 0);
}

TEST_CASE("BVH merge AABB", "[bvh]") {
    BVH bvh(100.0f);

    BoundingBox bbox1(Point(0, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1));
    BoundingBox bbox2(Point(5, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1));

    BoundingBox merged = bvh.merge_aabb(bbox1, bbox2);

    // Merged box should encompass both
    REQUIRE(std::abs(merged.center.x() - 2.5f) < 0.001f);  // Midpoint between 0 and 5
    REQUIRE(std::abs(merged.half_size.x() - 3.5f) < 0.001f);  // Half of distance from -1 to 6
}

TEST_CASE("BVH performance many boxes", "[bvh]") {
    std::mt19937 gen(42);
    std::uniform_real_distribution<float> pos_dist(-40.0f, 40.0f);
    std::uniform_real_distribution<float> size_dist(0.5f, 2.0f);
    
    std::vector<BoundingBox> bboxes;
    for (int i = 0; i < 100; ++i) {
        Point center(pos_dist(gen), pos_dist(gen), pos_dist(gen));
        Vector half_size(size_dist(gen), size_dist(gen), size_dist(gen));
        BoundingBox bbox(center, Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), half_size);
        bboxes.push_back(bbox);
    }

    BVH bvh = BVH::from_boxes(bboxes, 100.0f);
    auto [collisions, colliding_indices, checks] = bvh.check_all_collisions(bboxes);

    // BVH should perform fewer checks than naive O(n²)
    int naive_checks = static_cast<int>(bboxes.size()) * (static_cast<int>(bboxes.size()) - 1) / 2;
    REQUIRE(checks < naive_checks);
}

// All tests converted to Catch2 TEST_CASE format above
