#include "bvh.h"
#include <algorithm>
#include <set>
#include <cmath>

namespace session_cpp {

// BVHNode implementation
BVHNode::BVHNode() : guid(::guid()), left(nullptr), right(nullptr), object_id(-1) {}

bool BVHNode::is_leaf() const {
    return object_id != -1;
}

// Morton code functions
uint32_t expand_bits(uint32_t v) {
    v = (v * 0x00010001u) & 0xFF0000FFu;
    v = (v * 0x00000101u) & 0x0F00F00Fu;
    v = (v * 0x00000011u) & 0xC30C30C3u;
    v = (v * 0x00000005u) & 0x49249249u;
    return v;
}

uint32_t calculate_morton_code(float x, float y, float z, float world_size) {
    // Normalize coordinates to [0,1] range
    float nx = (x + world_size / 2.0f) / world_size;
    float ny = (y + world_size / 2.0f) / world_size;
    float nz = (z + world_size / 2.0f) / world_size;

    // Clamp to [0,1]
    nx = std::max(0.0f, std::min(1.0f, nx));
    ny = std::max(0.0f, std::min(1.0f, ny));
    nz = std::max(0.0f, std::min(1.0f, nz));

    // Scale to [0, 1023] for 10-bit encoding
    uint32_t ix = std::min(static_cast<uint32_t>(nx * 1023), 1023u);
    uint32_t iy = std::min(static_cast<uint32_t>(ny * 1023), 1023u);
    uint32_t iz = std::min(static_cast<uint32_t>(nz * 1023), 1023u);

    // Expand bits and interleave
    uint32_t xx = expand_bits(ix);
    uint32_t yy = expand_bits(iy);
    uint32_t zz = expand_bits(iz);

    return xx | (yy << 1) | (zz << 2);
}

// BVH implementation
BVH::BVH(float world_size) : guid(::guid()), name("my_bvh"), root(nullptr), world_size(world_size) {}

BVH BVH::from_boxes(const std::vector<BoundingBox>& bounding_boxes, float world_size) {
    BVH bvh(world_size);
    bvh.build(bounding_boxes);
    return bvh;
}

void BVH::build(const std::vector<BoundingBox>& bounding_boxes) {
    if (bounding_boxes.empty()) {
        root = nullptr;
        return;
    }

    // Create list of objects with their Morton codes
    std::vector<ObjectInfo> objects;
    for (size_t i = 0; i < bounding_boxes.size(); ++i) {
        const auto& bbox = bounding_boxes[i];
        uint32_t morton_code = calculate_morton_code(
            bbox.center.x(), bbox.center.y(), bbox.center.z(), world_size
        );
        objects.push_back({static_cast<int>(i), morton_code, bbox});
    }

    // Sort by Morton code for spatial locality
    std::sort(objects.begin(), objects.end(), 
              [](const ObjectInfo& a, const ObjectInfo& b) {
                  return a.morton_code < b.morton_code;
              });

    // Build tree recursively
    root = create_subtree(objects, 0, static_cast<int>(objects.size()) - 1);
}

std::shared_ptr<BVHNode> BVH::create_subtree(std::vector<ObjectInfo>& objects, int begin, int end) {
    if (begin == end) {
        // Create leaf node
        auto node = std::make_shared<BVHNode>();
        node->object_id = objects[begin].id;
        node->aabb = objects[begin].bbox;
        return node;
    } else {
        // Create internal node
        int mid = (begin + end) / 2;
        auto node = std::make_shared<BVHNode>();

        // Recursively create children
        node->left = create_subtree(objects, begin, mid);
        node->right = create_subtree(objects, mid + 1, end);

        // Merge children's AABBs
        node->aabb = merge_aabb(node->left->aabb, node->right->aabb);

        return node;
    }
}

BoundingBox BVH::merge_aabb(const BoundingBox& aabb1, const BoundingBox& aabb2) {
    // Calculate min and max corners
    float min_x = std::min(aabb1.center.x() - aabb1.half_size.x(), aabb2.center.x() - aabb2.half_size.x());
    float min_y = std::min(aabb1.center.y() - aabb1.half_size.y(), aabb2.center.y() - aabb2.half_size.y());
    float min_z = std::min(aabb1.center.z() - aabb1.half_size.z(), aabb2.center.z() - aabb2.half_size.z());

    float max_x = std::max(aabb1.center.x() + aabb1.half_size.x(), aabb2.center.x() + aabb2.half_size.x());
    float max_y = std::max(aabb1.center.y() + aabb1.half_size.y(), aabb2.center.y() + aabb2.half_size.y());
    float max_z = std::max(aabb1.center.z() + aabb1.half_size.z(), aabb2.center.z() + aabb2.half_size.z());

    // Calculate new center and half_size
    Point center((min_x + max_x) / 2.0f, (min_y + max_y) / 2.0f, (min_z + max_z) / 2.0f);
    Vector half_size((max_x - min_x) / 2.0f, (max_y - min_y) / 2.0f, (max_z - min_z) / 2.0f);

    return BoundingBox(center, Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), half_size);
}

std::pair<std::vector<int>, int> BVH::find_collisions(int object_id, const BoundingBox& query_bbox, 
                                                      const std::vector<BoundingBox>& bounding_boxes) {
    if (!root) {
        return {std::vector<int>(), 0};
    }

    std::vector<int> collisions;
    int check_count = 0;

    find_collisions_recursive(object_id, query_bbox, root, bounding_boxes, collisions, check_count);

    return {collisions, check_count};
}

void BVH::find_collisions_recursive(int object_id, const BoundingBox& query_bbox, std::shared_ptr<BVHNode> node,
                                    const std::vector<BoundingBox>& bounding_boxes, std::vector<int>& collisions, int& check_count) {
    check_count++;

    // Early exit if query doesn't intersect this node's AABB
    if (!aabb_intersect(query_bbox, node->aabb)) {
        return;
    }

    // If leaf node, check for collision
    if (node->is_leaf()) {
        // Don't check collision with self
        if (node->object_id != object_id) {
            if (aabb_intersect(query_bbox, bounding_boxes[node->object_id])) {
                collisions.push_back(node->object_id);
            }
        }
        return;
    }

    // Recurse through children
    if (node->left) {
        find_collisions_recursive(object_id, query_bbox, node->left, bounding_boxes, collisions, check_count);
    }
    if (node->right) {
        find_collisions_recursive(object_id, query_bbox, node->right, bounding_boxes, collisions, check_count);
    }
}

bool BVH::aabb_intersect(const BoundingBox& aabb1, const BoundingBox& aabb2) {
    // Calculate min/max for both boxes
    float min1_x = aabb1.center.x() - aabb1.half_size.x();
    float max1_x = aabb1.center.x() + aabb1.half_size.x();
    float min1_y = aabb1.center.y() - aabb1.half_size.y();
    float max1_y = aabb1.center.y() + aabb1.half_size.y();
    float min1_z = aabb1.center.z() - aabb1.half_size.z();
    float max1_z = aabb1.center.z() + aabb1.half_size.z();

    float min2_x = aabb2.center.x() - aabb2.half_size.x();
    float max2_x = aabb2.center.x() + aabb2.half_size.x();
    float min2_y = aabb2.center.y() - aabb2.half_size.y();
    float max2_y = aabb2.center.y() + aabb2.half_size.y();
    float min2_z = aabb2.center.z() - aabb2.half_size.z();
    float max2_z = aabb2.center.z() + aabb2.half_size.z();

    // Check for overlap on all three axes
    return (min1_x <= max2_x && max1_x >= min2_x &&
            min1_y <= max2_y && max1_y >= min2_y &&
            min1_z <= max2_z && max1_z >= min2_z);
}

std::tuple<std::vector<std::pair<int, int>>, std::vector<int>, int> BVH::check_all_collisions(const std::vector<BoundingBox>& bounding_boxes) {
    std::vector<std::pair<int, int>> all_collisions;
    std::set<int> colliding_objects;
    int total_checks = 0;

    for (size_t i = 0; i < bounding_boxes.size(); ++i) {
        auto [collisions, checks] = find_collisions(static_cast<int>(i), bounding_boxes[i], bounding_boxes);
        total_checks += checks;

        // Add unique collision pairs (avoid duplicates)
        for (int j : collisions) {
            if (static_cast<int>(i) < j) {  // Only add each pair once
                all_collisions.push_back({static_cast<int>(i), j});
                colliding_objects.insert(static_cast<int>(i));
                colliding_objects.insert(j);
            }
        }
    }

    std::vector<int> colliding_indices(colliding_objects.begin(), colliding_objects.end());
    return {all_collisions, colliding_indices, total_checks};
}

}
