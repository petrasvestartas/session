use crate::{BoundingBox, Point, Vector};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BVHNode {
    pub guid: String,
    pub left: Option<Box<BVHNode>>,
    pub right: Option<Box<BVHNode>>,
    pub object_id: i32,
    pub aabb: Option<BoundingBox>,
}

impl BVHNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_leaf(&self) -> bool {
        self.object_id >= 0
    }
}

impl Default for BVHNode {
    fn default() -> Self {
        BVHNode {
            guid: Uuid::new_v4().to_string(),
            left: None,
            right: None,
            object_id: -1,
            aabb: None,
        }
    }
}

// Lightweight AABB for arena nodes (6 doubles, no axes)
#[derive(Clone, Copy, Default, Debug)]
struct BvhAABB {
    cx: f64,
    cy: f64,
    cz: f64,
    hx: f64,
    hy: f64,
    hz: f64,
}

impl BvhAABB {
    #[inline(always)]
    fn from_bbox(b: &BoundingBox) -> Self {
        BvhAABB {
            cx: b.center.x(),
            cy: b.center.y(),
            cz: b.center.z(),
            hx: b.half_size.x(),
            hy: b.half_size.y(),
            hz: b.half_size.z(),
        }
    }

    #[inline(always)]
    fn merge(a: BvhAABB, b: BvhAABB) -> BvhAABB {
        let min_x = (a.cx - a.hx).min(b.cx - b.hx);
        let min_y = (a.cy - a.hy).min(b.cy - b.hy);
        let min_z = (a.cz - a.hz).min(b.cz - b.hz);
        let max_x = (a.cx + a.hx).max(b.cx + b.hx);
        let max_y = (a.cy + a.hy).max(b.cy + b.hy);
        let max_z = (a.cz + a.hz).max(b.cz + b.hz);
        BvhAABB {
            cx: (min_x + max_x) * 0.5,
            cy: (min_y + max_y) * 0.5,
            cz: (min_z + max_z) * 0.5,
            hx: (max_x - min_x) * 0.5,
            hy: (max_y - min_y) * 0.5,
            hz: (max_z - min_z) * 0.5,
        }
    }

    #[inline(always)]
    fn intersects(&self, other: &BvhAABB) -> bool {
        let min1_x = self.cx - self.hx;
        let max1_x = self.cx + self.hx;
        let min1_y = self.cy - self.hy;
        let max1_y = self.cy + self.hy;
        let min1_z = self.cz - self.hz;
        let max1_z = self.cz + self.hz;

        let min2_x = other.cx - other.hx;
        let max2_x = other.cx + other.hx;
        let min2_y = other.cy - other.hy;
        let max2_y = other.cy + other.hy;
        let min2_z = other.cz - other.hz;
        let max2_z = other.cz + other.hz;

        min1_x <= max2_x
            && max1_x >= min2_x
            && min1_y <= max2_y
            && max1_y >= min2_y
            && min1_z <= max2_z
            && max1_z >= min2_z
    }
}

// Flat node for arena-based traversal (cache-friendly)
#[derive(Clone, Copy, Debug)]
struct FlatNode {
    left: i32,      // -1 if leaf
    right: i32,     // -1 if leaf
    object_id: i32, // >= 0 for leaf, -1 for internal
    aabb: BvhAABB,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BVH {
    pub guid: String,
    pub name: String,
    pub root: Option<Box<BVHNode>>,
    pub world_size: f64,
    #[serde(skip)]
    pub object_guids: Vec<String>, // Parallel array to boxes - maps indices to GUIDs
    #[serde(skip)]
    arena: Vec<FlatNode>, // Flat node arena for fast queries
    #[serde(skip)]
    arena_root: i32, // Root index in arena (-1 if empty)
}

#[derive(Debug, Clone)]
struct ObjectInfo {
    id: usize,
    morton_code: u32,
}

impl Default for BVH {
    fn default() -> Self {
        Self::new()
    }
}

impl BVH {
    pub fn new() -> Self {
        BVH {
            guid: Uuid::new_v4().to_string(),
            name: "my_bvh".to_string(),
            root: None,
            world_size: 1000.0, // Default, will be computed from boxes
            object_guids: Vec::new(),
            arena: Vec::new(),
            arena_root: -1,
        }
    }

    /// Compute world size from bounding boxes
    pub fn compute_world_size(bounding_boxes: &[BoundingBox]) -> f64 {
        if bounding_boxes.is_empty() {
            return 1000.0;
        }

        let mut max_extent = 0.0f64;
        for bbox in bounding_boxes {
            // Find maximum absolute coordinate in any dimension
            let x_extent = (bbox.center.x() + bbox.half_size.x())
                .abs()
                .max((bbox.center.x() - bbox.half_size.x()).abs());
            let y_extent = (bbox.center.y() + bbox.half_size.y())
                .abs()
                .max((bbox.center.y() - bbox.half_size.y()).abs());
            let z_extent = (bbox.center.z() + bbox.half_size.z())
                .abs()
                .max((bbox.center.z() - bbox.half_size.z()).abs());

            max_extent = max_extent.max(x_extent).max(y_extent).max(z_extent);
        }

        // World size should be at least 2x the maximum extent, plus padding
        (max_extent * 2.2).max(10.0)
    }

    /// Build BVH from bounding boxes with GUIDs
    pub fn build_with_guids(&mut self, boxes_with_guids: &[(BoundingBox, String)]) {
        if boxes_with_guids.is_empty() {
            self.root = None;
            self.object_guids.clear();
            return;
        }

        // Extract boxes and GUIDs
        let bounding_boxes: Vec<BoundingBox> = boxes_with_guids
            .iter()
            .map(|(bbox, _)| bbox.clone())
            .collect();
        self.object_guids = boxes_with_guids
            .iter()
            .map(|(_, guid)| guid.clone())
            .collect();

        // Auto-compute world size from bounding boxes
        self.world_size = Self::compute_world_size(&bounding_boxes);

        // Build the tree
        self.build(&bounding_boxes);
    }

    pub fn from_boxes(bounding_boxes: &[BoundingBox], world_size: f64) -> Self {
        let mut bvh = Self::new();
        bvh.world_size = world_size;
        bvh.build(bounding_boxes);
        bvh
    }

    pub fn build(&mut self, bounding_boxes: &[BoundingBox]) {
        if bounding_boxes.is_empty() {
            self.root = None;
            self.arena.clear();
            self.arena_root = -1;
            return;
        }

        // Create list of objects with their Morton codes (no bbox copies needed later)
        let mut objects: Vec<ObjectInfo> = bounding_boxes
            .iter()
            .enumerate()
            .map(|(i, bbox)| {
                let morton_code = calculate_morton_code(
                    bbox.center.x(),
                    bbox.center.y(),
                    bbox.center.z(),
                    self.world_size,
                );
                ObjectInfo { id: i, morton_code }
            })
            .collect();

        // Radix sort 30-bit Morton codes: 3 passes of 10 bits (RADIX = 1024)
        {
            const RADIX: usize = 1024;
            const PASSES: usize = 3;
            let mut tmp: Vec<ObjectInfo> = vec![objects[0].clone(); objects.len()];
            for pass in 0..PASSES {
                let shift = (pass * 10) as u32;
                let mut count = [0usize; RADIX];
                for e in objects.iter() {
                    let b = ((e.morton_code >> shift) & ((RADIX as u32) - 1)) as usize;
                    count[b] += 1;
                }
                let mut sum = 0usize;
                for c in count.iter_mut() {
                    let old = *c;
                    *c = sum;
                    sum += old;
                }
                for e in objects.iter() {
                    let b = ((e.morton_code >> shift) & ((RADIX as u32) - 1)) as usize;
                    tmp[count[b]] = ObjectInfo {
                        id: e.id,
                        morton_code: e.morton_code,
                    };
                    count[b] += 1;
                }
                std::mem::swap(&mut objects, &mut tmp);
            }
        }

        // LBVH (Karras 2012) construction in O(N) after sort
        let n = objects.len();
        if n == 1 {
            // Single leaf - build arena only
            let id = objects[0].id;
            let aabb = BvhAABB::from_bbox(&bounding_boxes[id]);

            self.arena.clear();
            self.arena.push(FlatNode {
                left: -1,
                right: -1,
                object_id: id as i32,
                aabb,
            });
            self.arena_root = 0;
            self.root = None;
            return;
        }

        // Codes array for prefix computations
        let codes: Vec<u32> = objects.iter().map(|o| o.morton_code).collect();

        #[inline]
        fn clz32(x: u32) -> i32 {
            if x == 0 {
                32
            } else {
                x.leading_zeros() as i32
            }
        }

        let common_prefix = |i: i32, j: i32| -> i32 {
            if j < 0 || j >= n as i32 {
                return -1;
            }
            let ci = codes[i as usize];
            let cj = codes[j as usize];
            if ci != cj {
                return clz32(ci ^ cj);
            }
            let di = i as u32;
            let dj = j as u32;
            32 + clz32(di ^ dj)
        };

        let determine_range = |i: i32| -> (i32, i32) {
            let d = if common_prefix(i, i + 1) - common_prefix(i, i - 1) > 0 {
                1
            } else {
                -1
            };
            let delta_min = common_prefix(i, i - d);
            let mut l = 1i32;
            while common_prefix(i, i + l * d) > delta_min {
                l <<= 1;
            }
            let mut bound = 0i32;
            let mut t = l >> 1;
            while t > 0 {
                if common_prefix(i, i + (bound + t) * d) > delta_min {
                    bound += t;
                }
                t >>= 1;
            }
            let j = i + bound * d;
            (i.min(j), i.max(j))
        };

        let find_split = |first: i32, last: i32| -> i32 {
            let common = common_prefix(first, last);
            let mut split = first;
            let mut step = last - first;
            loop {
                step = (step + 1) >> 1;
                let new_split = split + step;
                if new_split < last {
                    let split_prefix = common_prefix(first, new_split);
                    if split_prefix > common {
                        split = new_split;
                    }
                }
                if step <= 1 {
                    break;
                }
            }
            split
        };

        // Temporary nodes with child indices
        #[derive(Clone)]
        enum TempChild {
            Leaf(usize),
            Internal(usize),
        }
        #[derive(Clone)]
        struct TempNode {
            left: Option<TempChild>,
            right: Option<TempChild>,
            object_id: i32,
            aabb: BvhAABB,
        }

        // Allocate leaves
        let mut leaves: Vec<TempNode> = Vec::with_capacity(n);
        for obj in objects.iter() {
            let id = obj.id;
            let aabb = BvhAABB::from_bbox(&bounding_boxes[id]);
            leaves.push(TempNode {
                left: None,
                right: None,
                object_id: id as i32,
                aabb,
            });
        }

        // Allocate internals
        let mut internals: Vec<TempNode> = Vec::with_capacity(n - 1);
        for _ in 0..(n - 1) {
            internals.push(TempNode {
                left: None,
                right: None,
                object_id: -1,
                aabb: BvhAABB::default(),
            });
        }

        // Build topology
        let mut has_parent: Vec<bool> = vec![false; n - 1];
        for i in 0..(n as i32 - 1) {
            let (first, last) = determine_range(i);
            let split = find_split(first, last);
            if split == first {
                internals[i as usize].left = Some(TempChild::Leaf(split as usize));
            } else {
                internals[i as usize].left = Some(TempChild::Internal(split as usize));
                has_parent[split as usize] = true;
            }
            if split + 1 == last {
                internals[i as usize].right = Some(TempChild::Leaf((split + 1) as usize));
            } else {
                internals[i as usize].right = Some(TempChild::Internal((split + 1) as usize));
                has_parent[(split + 1) as usize] = true;
            }
        }

        // Find root internal index
        let mut root_idx: usize = 0;
        for (idx, hp) in has_parent.iter().enumerate() {
            if !*hp {
                root_idx = idx;
                break;
            }
        }

        // Post-order compute internal AABBs
        fn compute_internal(
            idx: usize,
            internals: &mut [TempNode],
            leaves: &[TempNode],
        ) -> BvhAABB {
            let (left, right) = (
                internals[idx].left.clone().expect("left child"),
                internals[idx].right.clone().expect("right child"),
            );
            let a = match left {
                TempChild::Leaf(li) => leaves[li].aabb,
                TempChild::Internal(ii) => compute_internal(ii, internals, leaves),
            };
            let b = match right {
                TempChild::Leaf(li) => leaves[li].aabb,
                TempChild::Internal(ii) => compute_internal(ii, internals, leaves),
            };
            let merged = BvhAABB::merge(a, b);
            internals[idx].aabb = merged;
            merged
        }
        compute_internal(root_idx, &mut internals, &leaves);

        // Build flat arena for fast queries (cache-friendly, no pointers)
        self.arena.clear();
        self.arena.reserve(n + (n - 1)); // Reserve for all nodes

        // Helper to convert TempNode to FlatNode and add to arena
        fn build_arena_node(
            node_ref: TempChild,
            internals: &[TempNode],
            leaves: &[TempNode],
            arena: &mut Vec<FlatNode>,
        ) -> i32 {
            match node_ref {
                TempChild::Leaf(li) => {
                    let idx = arena.len() as i32;
                    arena.push(FlatNode {
                        left: -1,
                        right: -1,
                        object_id: leaves[li].object_id,
                        aabb: leaves[li].aabb,
                    });
                    idx
                }
                TempChild::Internal(ii) => {
                    let idx = arena.len() as i32;
                    // Reserve slot
                    arena.push(FlatNode {
                        left: -1,
                        right: -1,
                        object_id: -1,
                        aabb: internals[ii].aabb,
                    });
                    // Build children
                    let left_idx = build_arena_node(
                        internals[ii].left.clone().expect("left child"),
                        internals,
                        leaves,
                        arena,
                    );
                    let right_idx = build_arena_node(
                        internals[ii].right.clone().expect("right child"),
                        internals,
                        leaves,
                        arena,
                    );
                    // Update slot
                    arena[idx as usize].left = left_idx;
                    arena[idx as usize].right = right_idx;
                    idx
                }
            }
        }

        self.arena_root = build_arena_node(
            TempChild::Internal(root_idx),
            &internals,
            &leaves,
            &mut self.arena,
        );

        // Leave self.root as None - arena is used for all queries now
        self.root = None;
    }

    pub fn merge_aabb(&self, aabb1: &BoundingBox, aabb2: &BoundingBox) -> BoundingBox {
        // Calculate min and max corners
        let min_x =
            (aabb1.center.x() - aabb1.half_size.x()).min(aabb2.center.x() - aabb2.half_size.x());
        let min_y =
            (aabb1.center.y() - aabb1.half_size.y()).min(aabb2.center.y() - aabb2.half_size.y());
        let min_z =
            (aabb1.center.z() - aabb1.half_size.z()).min(aabb2.center.z() - aabb2.half_size.z());

        let max_x =
            (aabb1.center.x() + aabb1.half_size.x()).max(aabb2.center.x() + aabb2.half_size.x());
        let max_y =
            (aabb1.center.y() + aabb1.half_size.y()).max(aabb2.center.y() + aabb2.half_size.y());
        let max_z =
            (aabb1.center.z() + aabb1.half_size.z()).max(aabb2.center.z() + aabb2.half_size.z());

        // Calculate new center and half_size
        let center = Point::new(
            (min_x + max_x) / 2.0,
            (min_y + max_y) / 2.0,
            (min_z + max_z) / 2.0,
        );
        let half_size = Vector::new(
            (max_x - min_x) / 2.0,
            (max_y - min_y) / 2.0,
            (max_z - min_z) / 2.0,
        );

        BoundingBox::new(
            center,
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            half_size,
        )
    }

    pub fn find_collisions(
        &self,
        object_id: usize,
        query_bbox: &BoundingBox,
        bounding_boxes: &[BoundingBox],
    ) -> (Vec<usize>, i32) {
        let mut collisions = Vec::new();
        let mut check_count = 0;

        // Use arena for fast traversal
        if self.arena_root < 0 || self.arena.is_empty() {
            return (collisions, check_count);
        }

        let query_aabb = BvhAABB::from_bbox(query_bbox);
        let mut stack: Vec<i32> = Vec::with_capacity(64);
        stack.push(self.arena_root);

        while let Some(node_idx) = stack.pop() {
            check_count += 1;
            let node = &self.arena[node_idx as usize];

            // Early exit if query doesn't intersect this node's AABB
            if !query_aabb.intersects(&node.aabb) {
                continue;
            }

            // If leaf node, check for collision
            if node.object_id >= 0 {
                let node_object_id = node.object_id as usize;
                // Don't check collision with self
                if node_object_id != object_id
                    && node_object_id < bounding_boxes.len()
                    && self.aabb_intersect(query_bbox, &bounding_boxes[node_object_id])
                {
                    collisions.push(node_object_id);
                }
                continue;
            }

            // Internal node: push children
            if node.left >= 0 {
                stack.push(node.left);
            }
            if node.right >= 0 {
                stack.push(node.right);
            }
        }

        (collisions, check_count)
    }

    pub fn aabb_intersect(&self, aabb1: &BoundingBox, aabb2: &BoundingBox) -> bool {
        // Calculate min/max for both boxes
        let min1_x = aabb1.center.x() - aabb1.half_size.x();
        let max1_x = aabb1.center.x() + aabb1.half_size.x();
        let min1_y = aabb1.center.y() - aabb1.half_size.y();
        let max1_y = aabb1.center.y() + aabb1.half_size.y();
        let min1_z = aabb1.center.z() - aabb1.half_size.z();
        let max1_z = aabb1.center.z() + aabb1.half_size.z();

        let min2_x = aabb2.center.x() - aabb2.half_size.x();
        let max2_x = aabb2.center.x() + aabb2.half_size.x();
        let min2_y = aabb2.center.y() - aabb2.half_size.y();
        let max2_y = aabb2.center.y() + aabb2.half_size.y();
        let min2_z = aabb2.center.z() - aabb2.half_size.z();
        let max2_z = aabb2.center.z() + aabb2.half_size.z();

        // Check for overlap on all three axes
        min1_x <= max2_x
            && max1_x >= min2_x
            && min1_y <= max2_y
            && max1_y >= min2_y
            && min1_z <= max2_z
            && max1_z >= min2_z
    }

    pub fn check_all_collisions(
        &self,
        bounding_boxes: &[BoundingBox],
    ) -> (Vec<(usize, usize)>, Vec<usize>, i32) {
        let mut all_collisions: Vec<(usize, usize)> = Vec::new();
        let mut total_checks: i32 = 0;

        // Early out if arena is empty
        if self.arena_root < 0 || self.arena.is_empty() {
            return (all_collisions, Vec::new(), total_checks);
        }

        // Track which object indices participate in any collision
        let mut visited: Vec<bool> = vec![false; bounding_boxes.len()];

        // Stack of node index pairs for pairwise BVH traversal (cache-friendly)
        let mut stack: Vec<(i32, i32)> = Vec::with_capacity(256);
        stack.push((self.arena_root, self.arena_root));

        while let Some((a_idx, b_idx)) = stack.pop() {
            let a = &self.arena[a_idx as usize];
            let b = &self.arena[b_idx as usize];

            // AABB overlap test (inline for speed)
            if !a.aabb.intersects(&b.aabb) {
                continue;
            }

            // Count only when node AABBs overlap (matches C++ metric)
            total_checks += 1;

            let a_leaf = a.object_id >= 0;
            let b_leaf = b.object_id >= 0;

            if a_leaf && b_leaf {
                let i = a.object_id as usize;
                let j = b.object_id as usize;
                if i < j && i < visited.len() && j < visited.len() {
                    all_collisions.push((i, j));
                    visited[i] = true;
                    visited[j] = true;
                }
                continue;
            }

            // Expand children (index-based, no pointer chasing)
            if a_idx == b_idx {
                // Same node: expand unique child pairs without symmetry duplicates
                if a.left >= 0 {
                    stack.push((a.left, a.left));
                    if a.right >= 0 {
                        stack.push((a.left, a.right));
                        stack.push((a.right, a.right));
                    }
                }
                continue;
            }

            if !a_leaf && !b_leaf {
                // Both internal
                if a.left >= 0 && b.left >= 0 {
                    stack.push((a.left, b.left));
                }
                if a.left >= 0 && b.right >= 0 {
                    stack.push((a.left, b.right));
                }
                if a.right >= 0 && b.left >= 0 {
                    stack.push((a.right, b.left));
                }
                if a.right >= 0 && b.right >= 0 {
                    stack.push((a.right, b.right));
                }
            } else if a_leaf && !b_leaf {
                // a is leaf, b is internal
                if b.left >= 0 {
                    stack.push((a_idx, b.left));
                }
                if b.right >= 0 {
                    stack.push((a_idx, b.right));
                }
            } else if !a_leaf && b_leaf {
                // a is internal, b is leaf
                if a.left >= 0 {
                    stack.push((a.left, b_idx));
                }
                if a.right >= 0 {
                    stack.push((a.right, b_idx));
                }
            }
        }

        let mut colliding_indices: Vec<usize> = visited
            .iter()
            .enumerate()
            .filter_map(|(idx, v)| if *v { Some(idx) } else { None })
            .collect();
        colliding_indices.sort_unstable();

        (all_collisions, colliding_indices, total_checks)
    }

    /// Check for all collisions and return GUID pairs directly
    /// Uses the internally stored object_guids from build_with_guids
    pub fn check_all_collisions_guids(
        &self,
        bounding_boxes: &[BoundingBox],
    ) -> Vec<(String, String)> {
        let (collision_pairs, _, _) = self.check_all_collisions(bounding_boxes);

        // Convert indices to GUIDs
        collision_pairs
            .iter()
            .filter_map(|(i, j)| {
                if *i < self.object_guids.len() && *j < self.object_guids.len() {
                    Some((self.object_guids[*i].clone(), self.object_guids[*j].clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    #[inline(always)]
    fn ray_bvhaabb_intersect(
        origin: &Point,
        direction: &Vector,
        aabb: &BvhAABB,
    ) -> Option<(f64, f64)> {
        let min_x = aabb.cx - aabb.hx;
        let max_x = aabb.cx + aabb.hx;
        let min_y = aabb.cy - aabb.hy;
        let max_y = aabb.cy + aabb.hy;
        let min_z = aabb.cz - aabb.hz;
        let max_z = aabb.cz + aabb.hz;

        let invx = if direction.x() == 0.0 {
            f64::INFINITY
        } else {
            1.0 / direction.x()
        };
        let invy = if direction.y() == 0.0 {
            f64::INFINITY
        } else {
            1.0 / direction.y()
        };
        let invz = if direction.z() == 0.0 {
            f64::INFINITY
        } else {
            1.0 / direction.z()
        };

        let tx1 = (min_x - origin.x()) * invx;
        let tx2 = (max_x - origin.x()) * invx;
        let mut tmin = tx1.min(tx2);
        let mut tmax = tx1.max(tx2);

        let ty1 = (min_y - origin.y()) * invy;
        let ty2 = (max_y - origin.y()) * invy;
        tmin = tmin.max(ty1.min(ty2));
        tmax = tmax.min(ty1.max(ty2));

        let tz1 = (min_z - origin.z()) * invz;
        let tz2 = (max_z - origin.z()) * invz;
        tmin = tmin.max(tz1.min(tz2));
        tmax = tmax.min(tz1.max(tz2));

        if tmax >= tmin {
            Some((tmin, tmax))
        } else {
            None
        }
    }

    pub fn ray_cast(
        &self,
        origin: &Point,
        direction: &Vector,
        candidate_leaf_ids: &mut Vec<usize>,
        _find_all: bool,
    ) -> bool {
        candidate_leaf_ids.clear();

        // Use arena for fast index-based traversal
        if self.arena_root < 0 || self.arena.is_empty() {
            return false;
        }

        let root = &self.arena[self.arena_root as usize];
        if let Some((_tmin, tmax)) = Self::ray_bvhaabb_intersect(origin, direction, &root.aabb) {
            if tmax < 0.0 {
                return false;
            }
        } else {
            return false;
        }

        let mut stack: Vec<i32> = Vec::with_capacity(64);
        stack.push(self.arena_root);

        while let Some(node_idx) = stack.pop() {
            let node = &self.arena[node_idx as usize];

            if let Some((_tmin, tmax)) = Self::ray_bvhaabb_intersect(origin, direction, &node.aabb)
            {
                if tmax < 0.0 {
                    continue;
                }
            } else {
                continue;
            }

            if node.object_id >= 0 {
                // Leaf node
                candidate_leaf_ids.push(node.object_id as usize);
                continue;
            }

            // Internal node: push children
            if node.left >= 0 {
                stack.push(node.left);
            }
            if node.right >= 0 {
                stack.push(node.right);
            }
        }

        !candidate_leaf_ids.is_empty()
    }
}

// Morton code functions
pub fn expand_bits(v: u32) -> u32 {
    let mut v = v;
    v = (v.wrapping_mul(0x00010001)) & 0xFF0000FF;
    v = (v.wrapping_mul(0x00000101)) & 0x0F00F00F;
    v = (v.wrapping_mul(0x00000011)) & 0xC30C30C3;
    v = (v.wrapping_mul(0x00000005)) & 0x49249249;
    v
}

pub fn calculate_morton_code(x: f64, y: f64, z: f64, world_size: f64) -> u32 {
    // Normalize coordinates to [0,1] range
    let nx = (x + world_size / 2.0) / world_size;
    let ny = (y + world_size / 2.0) / world_size;
    let nz = (z + world_size / 2.0) / world_size;

    // Clamp to [0,1]
    let nx = nx.clamp(0.0, 1.0);
    let ny = ny.clamp(0.0, 1.0);
    let nz = nz.clamp(0.0, 1.0);

    // Scale to [0, 1023] for 10-bit encoding
    let ix = ((nx * 1023.0) as u32).min(1023);
    let iy = ((ny * 1023.0) as u32).min(1023);
    let iz = ((nz * 1023.0) as u32).min(1023);

    // Expand bits and interleave
    let xx = expand_bits(ix);
    let yy = expand_bits(iy);
    let zz = expand_bits(iz);

    xx | (yy << 1) | (zz << 2)
}

// Tests have been moved to bvh_test.rs for consistency with other modules
// and to match Python's test file structure (bvh_test.py)
