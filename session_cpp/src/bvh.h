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

class BVH {
public:
    std::string guid;
    std::string name;
    std::shared_ptr<BVHNode> root;
    float world_size;

    BVH(float world_size);
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
