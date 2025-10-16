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

/**
 * @brief A node in the Bounding Volume Hierarchy tree
 * 
 * Represents a single node in the BVH structure, containing either
 * child nodes or a reference to a geometry object.
 */
class BVHNode {
public:
    std::string guid;
    std::shared_ptr<BVHNode> left;
    std::shared_ptr<BVHNode> right;
    int object_id;
    BoundingBox aabb;

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
    std::shared_ptr<BVHNode> root;
    float world_size;
    std::vector<std::string> object_guids;  // Parallel array to boxes - maps indices to GUIDs

    BVH(float world_size = 1000.0f);
    
    // Compute world size from bounding boxes
    static float compute_world_size(const std::vector<BoundingBox>& bounding_boxes);
    
    // Build BVH from bounding boxes with GUIDs (auto-computes world size)
    void build_with_guids(const std::vector<std::pair<BoundingBox, std::string>>& boxes_with_guids);
    
    // Check all collisions and return GUID pairs directly
    std::vector<std::pair<std::string, std::string>> check_all_collisions_guids(const std::vector<BoundingBox>& bounding_boxes);
    
    static BVH from_boxes(const std::vector<BoundingBox>& bounding_boxes, float world_size);
    
    void build(const std::vector<BoundingBox>& bounding_boxes);
    std::tuple<std::vector<std::pair<int, int>>, std::vector<int>, int> check_all_collisions(const std::vector<BoundingBox>& bounding_boxes);
    std::pair<std::vector<int>, int> find_collisions(int object_id, const BoundingBox& query_bbox, const std::vector<BoundingBox>& bounding_boxes);

    // Public helper methods for testing
    BoundingBox merge_aabb(const BoundingBox& aabb1, const BoundingBox& aabb2);
    bool aabb_intersect(const BoundingBox& aabb1, const BoundingBox& aabb2);

private:
    struct ObjectInfo {
        int id;
        uint32_t morton_code;
        BoundingBox bbox;
    };

    std::shared_ptr<BVHNode> create_subtree(std::vector<ObjectInfo>& objects, int begin, int end);
    void find_collisions_recursive(int object_id, const BoundingBox& query_bbox, std::shared_ptr<BVHNode> node, 
                                   const std::vector<BoundingBox>& bounding_boxes, std::vector<int>& collisions, int& check_count);
};

// Morton code functions
uint32_t expand_bits(uint32_t v);
uint32_t calculate_morton_code(float x, float y, float z, float world_size = 100.0f);

}
