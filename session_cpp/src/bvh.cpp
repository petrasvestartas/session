#include "bvh.h"
#include <algorithm>
#include <cmath>
#include <limits>
#include <queue>
#include <array>
#include <functional>

namespace session_cpp {

bool BVH::aabb_intersect(const BvhAABB& aabb1, const BvhAABB& aabb2) {
    double min1_x = aabb1.cx - aabb1.hx;
    double max1_x = aabb1.cx + aabb1.hx;
    double min1_y = aabb1.cy - aabb1.hy;
    double max1_y = aabb1.cy + aabb1.hy;
    double min1_z = aabb1.cz - aabb1.hz;
    double max1_z = aabb1.cz + aabb1.hz;

    double min2_x = aabb2.cx - aabb2.hx;
    double max2_x = aabb2.cx + aabb2.hx;
    double min2_y = aabb2.cy - aabb2.hy;
    double max2_y = aabb2.cy + aabb2.hy;
    double min2_z = aabb2.cz - aabb2.hz;
    double max2_z = aabb2.cz + aabb2.hz;

    return (min1_x <= max2_x && max1_x >= min2_x &&
            min1_y <= max2_y && max1_y >= min2_y &&
            min1_z <= max2_z && max1_z >= min2_z);
}

void BVH::build_from_aabbs(const BvhAABB* aabbs, size_t count, double ws) {
    if (count == 0) {
        root = nullptr;
        node_arena.clear();
        return;
    }

    this->world_size = ws;

    std::vector<ObjectInfo> objects;
    objects.reserve(count);
    for (size_t i = 0; i < count; ++i) {
        const auto& bb = aabbs[i];
        uint32_t morton_code = calculate_morton_code(
            bb.cx, bb.cy, bb.cz, this->world_size
        );
        objects.emplace_back(ObjectInfo{static_cast<int>(i), morton_code});
    }

    // Radix sort 30-bit Morton codes
    {
        auto radix_sort = [&](std::vector<ObjectInfo>& arr) {
            const int RADIX = 1024;
            const int PASSES = 3;
            std::vector<ObjectInfo> tmp(arr.size());
            for (int pass = 0; pass < PASSES; ++pass) {
                std::array<int, RADIX> count{};
                int shift = pass * 10;
                for (const auto& e : arr) {
                    int b = static_cast<int>((e.morton_code >> shift) & (RADIX - 1));
                    count[b]++;
                }
                int sum = 0;
                for (int i = 0; i < RADIX; ++i) {
                    int c = count[i];
                    count[i] = sum;
                    sum += c;
                }
                for (const auto& e : arr) {
                    int b = static_cast<int>((e.morton_code >> shift) & (RADIX - 1));
                    tmp[count[b]++] = e;
                }
                arr.swap(tmp);
            }
        };
        radix_sort(objects);
    }

    node_arena.clear();
    node_arena.reserve(static_cast<size_t>(2 * count - 1));

    const int N = static_cast<int>(count);
    if (N == 1) {
        BVHNode* leaf = alloc_node();
        leaf->object_id = objects[0].id;
        const auto& bb = aabbs[objects[0].id];
        leaf->aabb = BvhAABB{bb.cx, bb.cy, bb.cz, bb.hx, bb.hy, bb.hz};
        leaf->left = leaf->right = nullptr;
        root = leaf;
        return;
    }

    std::vector<uint32_t> codes(N);
    for (int i = 0; i < N; ++i) codes[i] = objects[i].morton_code;

    auto clz32 = [](uint32_t x) -> int {
        if (x == 0) return 32;
#if defined(__clang__) || defined(__GNUC__)
        return __builtin_clz(x);
#else
        int n = 0; while ((x & 0x80000000u) == 0) { n++; x <<= 1; } return n;
#endif
    };

    auto commonPrefix = [&](int i, int j) -> int {
        if (j < 0 || j >= N) return -1;
        uint32_t ci = codes[i];
        uint32_t cj = codes[j];
        if (ci != cj) return clz32(ci ^ cj);
        uint32_t di = static_cast<uint32_t>(i);
        uint32_t dj = static_cast<uint32_t>(j);
        return 32 + clz32(di ^ dj);
    };

    auto determineRange = [&](int i, int& first, int& last) {
        int d = (commonPrefix(i, i + 1) - commonPrefix(i, i - 1)) > 0 ? 1 : -1;
        int deltaMin = commonPrefix(i, i - d);
        int l = 1;
        while (commonPrefix(i, i + l * d) > deltaMin) { l <<= 1; }
        int bound = 0;
        for (int t = l >> 1; t > 0; t >>= 1) {
            if (commonPrefix(i, i + (bound + t) * d) > deltaMin) bound += t;
        }
        int j = i + bound * d;
        first = std::min(i, j);
        last = std::max(i, j);
    };

    auto findSplit = [&](int first, int last) -> int {
        int common = commonPrefix(first, last);
        int split = first;
        int step = last - first;
        do {
            step = (step + 1) >> 1;
            int newSplit = split + step;
            if (newSplit < last) {
                int splitPrefix = commonPrefix(first, newSplit);
                if (splitPrefix > common) split = newSplit;
            }
        } while (step > 1);
        return split;
    };

    std::vector<BVHNode*> leaves(N);
    for (int i = 0; i < N; ++i) {
        BVHNode* leaf = alloc_node();
        leaf->object_id = objects[i].id;
        const auto& bb = aabbs[objects[i].id];
        leaf->aabb = BvhAABB{bb.cx, bb.cy, bb.cz, bb.hx, bb.hy, bb.hz};
        leaf->left = leaf->right = nullptr;
        leaves[i] = leaf;
    }

    std::vector<BVHNode*> internals(N - 1);
    for (int i = 0; i < N - 1; ++i) {
        BVHNode* in = alloc_node();
        in->object_id = -1;
        in->left = in->right = nullptr;
        internals[i] = in;
    }

    std::vector<char> has_parent(N - 1, 0);
    for (int i = 0; i < N - 1; ++i) {
        int first, last;
        determineRange(i, first, last);
        int split = findSplit(first, last);
        if (split == first) { internals[i]->left = leaves[split]; }
        else { internals[i]->left = internals[split]; has_parent[split] = 1; }
        if (split + 1 == last) { internals[i]->right = leaves[split + 1]; }
        else { internals[i]->right = internals[split + 1]; has_parent[split + 1] = 1; }
    }

    int rootIdx = 0;
    for (int i = 0; i < N - 1; ++i) { if (!has_parent[i]) { rootIdx = i; break; } }
    root = internals[rootIdx];

    std::function<void(BVHNode*)> compute = [&](BVHNode* n) {
        if (!n || n->is_leaf()) return;
        compute(n->left);
        compute(n->right);
        const auto& a = n->left->aabb;
        const auto& b = n->right->aabb;
        double min_x = std::min(a.cx - a.hx, b.cx - b.hx);
        double min_y = std::min(a.cy - a.hy, b.cy - b.hy);
        double min_z = std::min(a.cz - a.hz, b.cz - b.hz);
        double max_x = std::max(a.cx + a.hx, b.cx + b.hx);
        double max_y = std::max(a.cy + a.hy, b.cy + b.hy);
        double max_z = std::max(a.cz + a.hz, b.cz + b.hz);
        n->aabb = BvhAABB{(min_x + max_x) * 0.5,
                          (min_y + max_y) * 0.5,
                          (min_z + max_z) * 0.5,
                          (max_x - min_x) * 0.5,
                          (max_y - min_y) * 0.5,
                          (max_z - min_z) * 0.5};
    };
    compute(root);
}

// Overload for lightweight internal BVH AABB
static bool ray_aabb_intersect(const Point& origin,
                               const Vector& direction,
                               const BvhAABB& box,
                               double& tmin_out,
                               double& tmax_out)
{
    double min_x = box.cx - box.hx;
    double max_x = box.cx + box.hx;
    double min_y = box.cy - box.hy;
    double max_y = box.cy + box.hy;
    double min_z = box.cz - box.hz;
    double max_z = box.cz + box.hz;

    auto inv = [](double v) {
        return (v != 0.0) ? (1.0 / v) : std::numeric_limits<double>::infinity();
    };

    double invx = inv(direction.x());
    double invy = inv(direction.y());
    double invz = inv(direction.z());

    double tx1 = (min_x - origin.x()) * invx;
    double tx2 = (max_x - origin.x()) * invx;
    double tmin = std::min(tx1, tx2);
    double tmax = std::max(tx1, tx2);

    double ty1 = (min_y - origin.y()) * invy;
    double ty2 = (max_y - origin.y()) * invy;
    tmin = std::max(tmin, std::min(ty1, ty2));
    tmax = std::min(tmax, std::max(ty1, ty2));

    double tz1 = (min_z - origin.z()) * invz;
    double tz2 = (max_z - origin.z()) * invz;
    tmin = std::max(tmin, std::min(tz1, tz2));
    tmax = std::min(tmax, std::max(tz1, tz2));

    tmin_out = tmin;
    tmax_out = tmax;
    return tmax >= tmin;
}

// BVHNode implementation
BVHNode::BVHNode() : left(nullptr), right(nullptr), object_id(-1) {}

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

uint32_t calculate_morton_code(double x, double y, double z, double world_size) {
    // Normalize coordinates to [0,1] range
    const double inv_world = 1.0 / world_size;
    const double half_world = world_size * 0.5;
    double nx = (x + half_world) * inv_world;
    double ny = (y + half_world) * inv_world;
    double nz = (z + half_world) * inv_world;

    // Clamp to [0,1] and scale to [0, 1023] in one step
    uint32_t ix = std::min(static_cast<uint32_t>(std::max(0.0, std::min(1.0, nx)) * 1023.0), 1023u);
    uint32_t iy = std::min(static_cast<uint32_t>(std::max(0.0, std::min(1.0, ny)) * 1023.0), 1023u);
    uint32_t iz = std::min(static_cast<uint32_t>(std::max(0.0, std::min(1.0, nz)) * 1023.0), 1023u);

    // Inline expand_bits for all three coordinates (avoid 15k function calls)
    ix = (ix * 0x00010001u) & 0xFF0000FFu;
    ix = (ix * 0x00000101u) & 0x0F00F00Fu;
    ix = (ix * 0x00000011u) & 0xC30C30C3u;
    ix = (ix * 0x00000005u) & 0x49249249u;
    
    iy = (iy * 0x00010001u) & 0xFF0000FFu;
    iy = (iy * 0x00000101u) & 0x0F00F00Fu;
    iy = (iy * 0x00000011u) & 0xC30C30C3u;
    iy = (iy * 0x00000005u) & 0x49249249u;
    
    iz = (iz * 0x00010001u) & 0xFF0000FFu;
    iz = (iz * 0x00000101u) & 0x0F00F00Fu;
    iz = (iz * 0x00000011u) & 0xC30C30C3u;
    iz = (iz * 0x00000005u) & 0x49249249u;

    return ix | (iy << 1) | (iz << 2);
}

// BVH implementation
BVH::BVH(double world_size) : guid(::guid()), name("my_bvh"), root(nullptr), world_size(world_size) {}

double BVH::compute_world_size(const std::vector<BoundingBox>& bounding_boxes) {
    if (bounding_boxes.empty()) {
        return 1000.0;
    }
    
    double max_extent = 0.0;
    for (const auto& bbox : bounding_boxes) {
        // Find maximum absolute coordinate in any dimension
        double x_extent = std::max(std::abs(bbox.center.x() + bbox.half_size.x()), 
                                   std::abs(bbox.center.x() - bbox.half_size.x()));
        double y_extent = std::max(std::abs(bbox.center.y() + bbox.half_size.y()), 
                                   std::abs(bbox.center.y() - bbox.half_size.y()));
        double z_extent = std::max(std::abs(bbox.center.z() + bbox.half_size.z()), 
                                   std::abs(bbox.center.z() - bbox.half_size.z()));
        
        max_extent = std::max({max_extent, x_extent, y_extent, z_extent});
    }
    
    // World size should be at least 2x the maximum extent, plus padding
    return std::max(max_extent * 2.2, 10.0);
}

BVH BVH::from_boxes(const std::vector<BoundingBox>& bounding_boxes, double world_size) {
    BVH bvh(world_size);
    bvh.build_from_boxes(bounding_boxes.data(), bounding_boxes.size(), world_size);
    return bvh;
}

void BVH::build(const std::vector<BoundingBox>& bounding_boxes) {
    build_from_boxes(bounding_boxes.data(), bounding_boxes.size(), world_size);
}

void BVH::build_from_boxes(const BoundingBox* boxes, size_t count, double ws) {
    if (count == 0) {
        root = nullptr;
        node_arena.clear();
        return;
    }

    this->world_size = ws;

    // Create list of objects with Morton codes (pre-allocate)
    std::vector<ObjectInfo> objects;
    objects.reserve(count);
    for (size_t i = 0; i < count; ++i) {
        const auto& bbox = boxes[i];
        uint32_t morton_code = calculate_morton_code(
            bbox.center.x(), bbox.center.y(), bbox.center.z(), this->world_size
        );
        objects.emplace_back(ObjectInfo{static_cast<int>(i), morton_code});
    }

    // Radix sort by 30-bit Morton code (3 passes of 10 bits) for speed
    {
        auto radix_sort = [&](std::vector<ObjectInfo>& arr) {
            const int RADIX = 1024; // 2^10
            const int PASSES = 3;   // 30 bits total
            std::vector<ObjectInfo> tmp(arr.size());
            for (int pass = 0; pass < PASSES; ++pass) {
                std::array<int, RADIX> count{};
                int shift = pass * 10;
                for (const auto& e : arr) {
                    int b = static_cast<int>((e.morton_code >> shift) & (RADIX - 1));
                    count[b]++;
                }
                // Prefix sums
                int sum = 0;
                for (int i = 0; i < RADIX; ++i) {
                    int c = count[i];
                    count[i] = sum;
                    sum += c;
                }
                // Stable scatter
                for (const auto& e : arr) {
                    int b = static_cast<int>((e.morton_code >> shift) & (RADIX - 1));
                    tmp[count[b]++] = e;
                }
                arr.swap(tmp);
            }
        };
        radix_sort(objects);
    }

    // Prepare node arena: 2*n - 1 nodes total
    node_arena.clear();
    node_arena.reserve(static_cast<size_t>(2 * count - 1));

    // LBVH (Karras 2012) O(N) construction
    const int N = static_cast<int>(count);
    if (N == 1) {
        // Single leaf
        BVHNode* leaf = alloc_node();
        leaf->object_id = objects[0].id;
        const BoundingBox& bb = boxes[objects[0].id];
        leaf->aabb = BvhAABB{bb.center.x(), bb.center.y(), bb.center.z(), bb.half_size.x(), bb.half_size.y(), bb.half_size.z()};
        leaf->left = leaf->right = nullptr;
        root = leaf;
        return;
    }

    std::vector<uint32_t> codes(N);
    for (int i = 0; i < N; ++i) codes[i] = objects[i].morton_code;

    auto clz32 = [](uint32_t x) -> int {
        if (x == 0) return 32;
#if defined(__clang__) || defined(__GNUC__)
        return __builtin_clz(x);
#else
        // Fallback
        int n = 0; while ((x & 0x80000000u) == 0) { n++; x <<= 1; } return n;
#endif
    };

    auto commonPrefix = [&](int i, int j) -> int {
        if (j < 0 || j >= N) return -1; // out of range
        uint32_t ci = codes[i];
        uint32_t cj = codes[j];
        if (ci != cj) return clz32(ci ^ cj);
        // Disambiguate equal Morton codes using index bits
        uint32_t di = static_cast<uint32_t>(i);
        uint32_t dj = static_cast<uint32_t>(j);
        return 32 + clz32(di ^ dj);
    };

    auto determineRange = [&](int i, int& first, int& last) {
        int d = (commonPrefix(i, i + 1) - commonPrefix(i, i - 1)) > 0 ? 1 : -1;
        int deltaMin = commonPrefix(i, i - d);

        // Find lmax by exponential search
        int l = 1;
        while (commonPrefix(i, i + l * d) > deltaMin) {
            l <<= 1;
        }

        // Binary search to find exact bound
        int bound = 0;
        for (int t = l >> 1; t > 0; t >>= 1) {
            if (commonPrefix(i, i + (bound + t) * d) > deltaMin) bound += t;
        }
        int j = i + bound * d;
        first = std::min(i, j);
        last = std::max(i, j);
    };

    auto findSplit = [&](int first, int last) -> int {
        int common = commonPrefix(first, last);
        int split = first;
        int step = last - first;
        do {
            step = (step + 1) >> 1; // ceil div 2
            int newSplit = split + step;
            if (newSplit < last) {
                int splitPrefix = commonPrefix(first, newSplit);
                if (splitPrefix > common) split = newSplit;
            }
        } while (step > 1);
        return split;
    };

    // Allocate leaves
    std::vector<BVHNode*> leaves(N);
    for (int i = 0; i < N; ++i) {
        BVHNode* leaf = alloc_node();
        leaf->object_id = objects[i].id;
        const BoundingBox& bb = boxes[objects[i].id];
        leaf->aabb = BvhAABB{bb.center.x(), bb.center.y(), bb.center.z(), bb.half_size.x(), bb.half_size.y(), bb.half_size.z()};
        leaf->left = leaf->right = nullptr;
        leaves[i] = leaf;
    }

    // Allocate internal nodes
    std::vector<BVHNode*> internals(N - 1);
    for (int i = 0; i < N - 1; ++i) {
        BVHNode* in = alloc_node();
        in->object_id = -1;
        in->left = in->right = nullptr;
        internals[i] = in;
    }

    // Build topology
    std::vector<char> has_parent(N - 1, 0);
    for (int i = 0; i < N - 1; ++i) {
        int first, last;
        determineRange(i, first, last);
        int split = findSplit(first, last);

        // Left child
        if (split == first) {
            internals[i]->left = leaves[split];
        } else {
            internals[i]->left = internals[split];
            has_parent[split] = 1;
        }

        // Right child
        if (split + 1 == last) {
            internals[i]->right = leaves[split + 1];
        } else {
            internals[i]->right = internals[split + 1];
            has_parent[split + 1] = 1;
        }
    }

    // Find root (internal node without parent)
    int rootIdx = 0;
    for (int i = 0; i < N - 1; ++i) {
        if (!has_parent[i]) { rootIdx = i; break; }
    }
    root = internals[rootIdx];

    // Post-order compute internal AABBs
    std::function<void(BVHNode*)> compute = [&](BVHNode* n) {
        if (!n || n->is_leaf()) return;
        compute(n->left);
        compute(n->right);
        const auto& a = n->left->aabb;
        const auto& b = n->right->aabb;
        double min_x = std::min(a.cx - a.hx, b.cx - b.hx);
        double min_y = std::min(a.cy - a.hy, b.cy - b.hy);
        double min_z = std::min(a.cz - a.hz, b.cz - b.hz);
        double max_x = std::max(a.cx + a.hx, b.cx + b.hx);
        double max_y = std::max(a.cy + a.hy, b.cy + b.hy);
        double max_z = std::max(a.cz + a.hz, b.cz + b.hz);
        n->aabb = BvhAABB{(min_x + max_x) * 0.5,
                          (min_y + max_y) * 0.5,
                          (min_z + max_z) * 0.5,
                          (max_x - min_x) * 0.5,
                          (max_y - min_y) * 0.5,
                          (max_z - min_z) * 0.5};
    };
    compute(root);
}

BVHNode* BVH::alloc_node() {
    node_arena.emplace_back();
    return &node_arena.back();
}

BVHNode* BVH::create_subtree(std::vector<ObjectInfo>& objects, int begin, int end, const BoundingBox* boxes) {
    if (begin == end) {
        // Create leaf node
        BVHNode* node = alloc_node();
        node->object_id = objects[begin].id;
        const BoundingBox& bb = boxes[objects[begin].id];
        node->aabb = BvhAABB{bb.center.x(), bb.center.y(), bb.center.z(),
                             bb.half_size.x(), bb.half_size.y(), bb.half_size.z()};
        node->left = nullptr;
        node->right = nullptr;
        return node;
    } else {
        // Create internal node with simple midpoint split
        int mid = (begin + end) / 2;
        BVHNode* node = alloc_node();

        // Recursively create children
        node->left = create_subtree(objects, begin, mid, boxes);
        node->right = create_subtree(objects, mid + 1, end, boxes);

        // Inline AABB merging (avoid function call overhead)
        const auto& aabb1 = node->left->aabb;
        const auto& aabb2 = node->right->aabb;
        
        // Cache component values
        double c1x = aabb1.cx, c1y = aabb1.cy, c1z = aabb1.cz;
        double h1x = aabb1.hx, h1y = aabb1.hy, h1z = aabb1.hz;
        double c2x = aabb2.cx, c2y = aabb2.cy, c2z = aabb2.cz;
        double h2x = aabb2.hx, h2y = aabb2.hy, h2z = aabb2.hz;
        
        // Calculate merged AABB directly
        double min_x = std::min(c1x - h1x, c2x - h2x);
        double min_y = std::min(c1y - h1y, c2y - h2y);
        double min_z = std::min(c1z - h1z, c2z - h2z);
        double max_x = std::max(c1x + h1x, c2x + h2x);
        double max_y = std::max(c1y + h1y, c2y + h2y);
        double max_z = std::max(c1z + h1z, c2z + h2z);
        
        node->aabb = BvhAABB{(min_x + max_x) * 0.5,
                              (min_y + max_y) * 0.5,
                              (min_z + max_z) * 0.5,
                              (max_x - min_x) * 0.5,
                              (max_y - min_y) * 0.5,
                              (max_z - min_z) * 0.5};

        node->object_id = -1;
        return node;
    }
}

BoundingBox BVH::merge_aabb(const BoundingBox& aabb1, const BoundingBox& aabb2) {
    // Cache center and half_size components (avoid repeated method calls)
    double c1x = aabb1.center.x(), c1y = aabb1.center.y(), c1z = aabb1.center.z();
    double h1x = aabb1.half_size.x(), h1y = aabb1.half_size.y(), h1z = aabb1.half_size.z();
    double c2x = aabb2.center.x(), c2y = aabb2.center.y(), c2z = aabb2.center.z();
    double h2x = aabb2.half_size.x(), h2y = aabb2.half_size.y(), h2z = aabb2.half_size.z();
    
    // Calculate min and max corners
    double min_x = std::min(c1x - h1x, c2x - h2x);
    double min_y = std::min(c1y - h1y, c2y - h2y);
    double min_z = std::min(c1z - h1z, c2z - h2z);
    
    double max_x = std::max(c1x + h1x, c2x + h2x);
    double max_y = std::max(c1y + h1y, c2y + h2y);
    double max_z = std::max(c1z + h1z, c2z + h2z);

    // Calculate new center and half_size in one go
    return BoundingBox(
        Point((min_x + max_x) * 0.5, (min_y + max_y) * 0.5, (min_z + max_z) * 0.5),
        Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1),
        Vector((max_x - min_x) * 0.5, (max_y - min_y) * 0.5, (max_z - min_z) * 0.5)
    );
}

bool BVH::aabb_intersect(const BoundingBox& aabb1, const BoundingBox& aabb2) {
    double min1_x = aabb1.center.x() - aabb1.half_size.x();
    double max1_x = aabb1.center.x() + aabb1.half_size.x();
    double min1_y = aabb1.center.y() - aabb1.half_size.y();
    double max1_y = aabb1.center.y() + aabb1.half_size.y();
    double min1_z = aabb1.center.z() - aabb1.half_size.z();
    double max1_z = aabb1.center.z() + aabb1.half_size.z();

    double min2_x = aabb2.center.x() - aabb2.half_size.x();
    double max2_x = aabb2.center.x() + aabb2.half_size.x();
    double min2_y = aabb2.center.y() - aabb2.half_size.y();
    double max2_y = aabb2.center.y() + aabb2.half_size.y();
    double min2_z = aabb2.center.z() - aabb2.half_size.z();
    double max2_z = aabb2.center.z() + aabb2.half_size.z();

    return (min1_x <= max2_x && max1_x >= min2_x &&
            min1_y <= max2_y && max1_y >= min2_y &&
            min1_z <= max2_z && max1_z >= min2_z);
}

std::tuple<std::vector<std::pair<int, int>>, std::vector<int>, int> BVH::check_all_collisions(const std::vector<BoundingBox>& bounding_boxes) {
    std::vector<std::pair<int, int>> all_collisions;
    all_collisions.reserve(bounding_boxes.size());
    std::vector<char> visited(bounding_boxes.size(), 0);
    int total_checks = 0;

    if (!root) {
        return {all_collisions, {}, total_checks};
    }

    // Pair traversal stack (canonicalized by pointer address to avoid duplicate work)
    std::vector<std::pair<BVHNode*, BVHNode*>> stack;
    stack.reserve(64);
    stack.emplace_back(root, root);

    auto push_pair = [&](BVHNode* a, BVHNode* b) {
        if (!a || !b) return;
        // Canonicalize order
        if (a > b) std::swap(a, b);
        stack.emplace_back(a, b);
    };

    while (!stack.empty()) {
        auto [a, b] = stack.back();
        stack.pop_back();
        if (!a || !b) continue;

        // AABB overlap test
        if (!aabb_intersect(a->aabb, b->aabb)) continue;
        total_checks++;

        bool aLeaf = a->is_leaf();
        bool bLeaf = b->is_leaf();

        if (aLeaf && bLeaf) {
            int i = a->object_id;
            int j = b->object_id;
            if (i >= 0 && j >= 0 && i < j) {
                all_collisions.emplace_back(i, j);
                if (static_cast<size_t>(i) < visited.size()) visited[static_cast<size_t>(i)] = 1;
                if (static_cast<size_t>(j) < visited.size()) visited[static_cast<size_t>(j)] = 1;
            }
            continue;
        }

        if (a == b) {
            // Same node: expand unique child pairs without symmetry duplicates
            if (!aLeaf) {
                if (a->left)  push_pair(a->left,  a->left);
                if (a->left && a->right) push_pair(a->left, a->right);
                if (a->right) push_pair(a->right, a->right);
            }
            continue;
        }

        if (!aLeaf && !bLeaf) {
            push_pair(a->left,  b->left);
            push_pair(a->left,  b->right);
            push_pair(a->right, b->left);
            push_pair(a->right, b->right);
        } else if (aLeaf && !bLeaf) {
            push_pair(a, b->left);
            push_pair(a, b->right);
        } else if (!aLeaf && bLeaf) {
            push_pair(a->left,  b);
            push_pair(a->right, b);
        }
    }

    std::vector<int> colliding_indices;
    colliding_indices.reserve(bounding_boxes.size());
    for (size_t idx = 0; idx < visited.size(); ++idx) {
        if (visited[idx]) colliding_indices.push_back(static_cast<int>(idx));
    }
    return {all_collisions, colliding_indices, total_checks};
}

bool BVH::ray_cast(const Point& origin,
                   const Vector& direction,
                   std::vector<int>& candidate_leaf_ids,
                   bool find_all) const
{
    candidate_leaf_ids.clear();
    if (!root) return false;

    struct HeapItem {
        BVHNode* node;
        double tmin;
        
        bool operator>(const HeapItem& other) const {
            return tmin > other.tmin; // min-heap (smallest tmin at top)
        }
    };

    std::priority_queue<HeapItem, std::vector<HeapItem>, std::greater<HeapItem>> heap;

    double rtmin, rtmax;
    if (!ray_aabb_intersect(origin, direction, root->aabb, rtmin, rtmax)) {
        return false;
    }
    
    // Prune boxes entirely behind ray origin
    if (rtmax < 0.0) return false;
    
    heap.push({root, rtmin});

    bool any = false;
    while (!heap.empty()) {
        HeapItem item = heap.top();
        heap.pop();

        auto node = item.node;
        if (!node) continue;

        if (node->is_leaf()) {
            candidate_leaf_ids.push_back(node->object_id);
            any = true;
            if (!find_all && candidate_leaf_ids.size() >= 1) {
                // Early exit for find_first, but keep traversing for near-first ordering
            }
            continue;
        }

        // Intersect children and push (prune boxes behind ray)
        if (node->left) {
            double cmin, cmax;
            if (ray_aabb_intersect(origin, direction, node->left->aabb, cmin, cmax) && cmax >= 0.0) {
                heap.push({node->left, cmin});
            }
        }
        if (node->right) {
            double cmin, cmax;
            if (ray_aabb_intersect(origin, direction, node->right->aabb, cmin, cmax) && cmax >= 0.0) {
                heap.push({node->right, cmin});
            }
        }
    }

    return any;
}

}
