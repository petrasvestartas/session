#include "bvh.h"
#include <algorithm>
#include <set>
#include <cmath>
#include <limits>

namespace session_cpp {

static bool ray_aabb_intersect(const Point& origin,
                               const Vector& direction,
                               const BoundingBox& box,
                               float& tmin_out,
                               float& tmax_out)
{
    // AABB from center/half_size
    float min_x = box.center.x() - box.half_size.x();
    float max_x = box.center.x() + box.half_size.x();
    float min_y = box.center.y() - box.half_size.y();
    float max_y = box.center.y() + box.half_size.y();
    float min_z = box.center.z() - box.half_size.z();
    float max_z = box.center.z() + box.half_size.z();

    auto inv = [](float v) {
        return (v != 0.0f) ? (1.0f / v) : std::numeric_limits<float>::infinity();
    };

    float invx = inv(direction.x());
    float invy = inv(direction.y());
    float invz = inv(direction.z());

    float tx1 = (min_x - origin.x()) * invx;
    float tx2 = (max_x - origin.x()) * invx;
    float tmin = std::min(tx1, tx2);
    float tmax = std::max(tx1, tx2);

    float ty1 = (min_y - origin.y()) * invy;
    float ty2 = (max_y - origin.y()) * invy;
    tmin = std::max(tmin, std::min(ty1, ty2));
    tmax = std::min(tmax, std::max(ty1, ty2));

    float tz1 = (min_z - origin.z()) * invz;
    float tz2 = (max_z - origin.z()) * invz;
    tmin = std::max(tmin, std::min(tz1, tz2));
    tmax = std::min(tmax, std::max(tz1, tz2));

    tmin_out = tmin;
    tmax_out = tmax;
    return tmax >= tmin;
}

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

float BVH::compute_world_size(const std::vector<BoundingBox>& bounding_boxes) {
    if (bounding_boxes.empty()) {
        return 1000.0f;
    }
    
    float max_extent = 0.0f;
    for (const auto& bbox : bounding_boxes) {
        // Find maximum absolute coordinate in any dimension
        float x_extent = std::max(std::abs(bbox.center.x() + bbox.half_size.x()), 
                                   std::abs(bbox.center.x() - bbox.half_size.x()));
        float y_extent = std::max(std::abs(bbox.center.y() + bbox.half_size.y()), 
                                   std::abs(bbox.center.y() - bbox.half_size.y()));
        float z_extent = std::max(std::abs(bbox.center.z() + bbox.half_size.z()), 
                                   std::abs(bbox.center.z() - bbox.half_size.z()));
        
        max_extent = std::max({max_extent, x_extent, y_extent, z_extent});
    }
    
    // World size should be at least 2x the maximum extent, plus padding
    return std::max(max_extent * 2.2f, 10.0f);
}

void BVH::build_with_guids(const std::vector<std::pair<BoundingBox, std::string>>& boxes_with_guids) {
    if (boxes_with_guids.empty()) {
        root = nullptr;
        object_guids.clear();
        return;
    }
    
    // Extract boxes and GUIDs
    std::vector<BoundingBox> bounding_boxes;
    object_guids.clear();
    for (const auto& [bbox, guid] : boxes_with_guids) {
        bounding_boxes.push_back(bbox);
        object_guids.push_back(guid);
    }
    
    // Auto-compute world size from bounding boxes
    world_size = compute_world_size(bounding_boxes);
    
    // Build the tree
    build(bounding_boxes);
}

std::vector<std::pair<std::string, std::string>> BVH::check_all_collisions_guids(const std::vector<BoundingBox>& bounding_boxes) {
    auto [collision_pairs, colliding_indices, check_count] = check_all_collisions(bounding_boxes);
    (void)colliding_indices;  // Unused
    (void)check_count;  // Unused
    
    // Convert indices to GUIDs
    std::vector<std::pair<std::string, std::string>> guid_collisions;
    for (const auto& [i, j] : collision_pairs) {
        if (i >= 0 && j >= 0 && 
            static_cast<size_t>(i) < object_guids.size() && 
            static_cast<size_t>(j) < object_guids.size()) {
            guid_collisions.push_back(std::make_pair(object_guids[i], object_guids[j]));
        }
    }
    
    return guid_collisions;
}

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

bool BVH::ray_cast(const Point& origin,
                   const Vector& direction,
                   std::vector<int>& candidate_leaf_ids,
                   bool find_all) const
{
    candidate_leaf_ids.clear();
    if (!root) return false;

    struct StackItem {
        std::shared_ptr<BVHNode> node;
        float tmin;
        float tmax;
    };

    std::vector<StackItem> stack;
    stack.reserve(64);

    float rtmin, rtmax;
    if (!ray_aabb_intersect(origin, direction, root->aabb, rtmin, rtmax)) {
        return false;
    }
    stack.push_back({root, rtmin, rtmax});

    bool any = false;
    while (!stack.empty()) {
        // pop the item with smallest tmin (near first)
        size_t best_i = stack.size() - 1;
        for (size_t i = 0; i + 1 < stack.size(); ++i) {
            if (stack[i].tmin < stack[best_i].tmin) best_i = i;
        }
        StackItem item = stack[best_i];
        stack.erase(stack.begin() + best_i);

        auto node = item.node;
        if (!node) continue;

        if (node->is_leaf()) {
            candidate_leaf_ids.push_back(node->object_id);
            any = true;
            if (!find_all) {
                // Do not early return here; correctness is maintained by exact phase later.
            }
            continue;
        }

        // Intersect children and push
        if (node->left) {
            float cmin, cmax;
            if (ray_aabb_intersect(origin, direction, node->left->aabb, cmin, cmax)) {
                stack.push_back({node->left, cmin, cmax});
            }
        }
        if (node->right) {
            float cmin, cmax;
            if (ray_aabb_intersect(origin, direction, node->right->aabb, cmin, cmax)) {
                stack.push_back({node->right, cmin, cmax});
            }
        }
    }

    return any;
}

}
