#pragma once

#include "point.h"
#include "vector.h"
#include "boundingbox.h"
#include "guid.h"
#include <string>
#include <vector>
#include <memory>
#include <tuple>

namespace session_cpp {

// Lightweight axis-aligned bounding box used internally by BVH
struct BvhAABB {
    double cx, cy, cz; // center
    double hx, hy, hz; // half-size
};

/**
 * @brief A node in the Bounding Volume Hierarchy tree
 * 
 * Represents a single node in the BVH structure, containing either
 * child nodes or a reference to a geometry object.
 */
class BVHNode {
public:
    BVHNode* left;
    BVHNode* right;
    int object_id;
    BvhAABB aabb;

    BVHNode();
    bool is_leaf() const;
};

/**
 * @brief Bounding Volume Hierarchy for efficient spatial queries and collision detection
 * 
 * A spatial data structure that organizes bounding boxes in a binary tree
 * for fast collision detection and spatial queries using Morton codes.
 */
class BVH {
public:
    std::string guid;
    std::string name;
    BVHNode* root;
    double world_size;
    
    // Node arena to store all nodes contiguously (no per-node heap allocations)
    std::vector<BVHNode> node_arena;

    BVH(double world_size = 1000.0);
    
    // Compute world size from bounding boxes
    static double compute_world_size(const std::vector<BoundingBox>& bounding_boxes);
    
    static BVH from_boxes(const std::vector<BoundingBox>& bounding_boxes, double world_size);
    
    // Fast build accepting continuous array of boxes (no copies)
    void build_from_boxes(const BoundingBox* boxes, size_t count, double ws);
    // Fast build accepting continuous array of lightweight AABBs (no BoundingBox construction)
    void build_from_aabbs(const BvhAABB* aabbs, size_t count, double ws);
    
    void build(const std::vector<BoundingBox>& bounding_boxes);
    std::tuple<std::vector<std::pair<int, int>>, std::vector<int>, int> check_all_collisions(const std::vector<BoundingBox>& bounding_boxes);

    // Public helper methods for testing
    BoundingBox merge_aabb(const BoundingBox& aabb1, const BoundingBox& aabb2);
    bool aabb_intersect(const BoundingBox& aabb1, const BoundingBox& aabb2);
    bool aabb_intersect(const BvhAABB& aabb1, const BvhAABB& aabb2);

    // Ray cast traversal over BVH nodes returning candidate leaf indices ordered by AABB entry t.
    bool ray_cast(const Point& origin,
                  const Vector& direction,
                  std::vector<int>& candidate_leaf_ids,
                  bool find_all = false) const;

private:
    struct ObjectInfo {
        int id;
        uint32_t morton_code;
    };

    // Allocates a node in the arena and returns a raw pointer to it
    BVHNode* alloc_node();

    // Subtree creation using sorted object keys and read-only boxes array
    BVHNode* create_subtree(std::vector<ObjectInfo>& objects, int begin, int end, const BoundingBox* boxes);
};

// Morton code functions
uint32_t expand_bits(uint32_t v);
uint32_t calculate_morton_code(double x, double y, double z, double world_size = 100.0);

}
