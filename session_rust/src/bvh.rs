use crate::{BoundingBox, Point, Vector};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BVH {
    pub guid: String,
    pub name: String,
    pub root: Option<Box<BVHNode>>,
    pub world_size: f32,
}

#[derive(Debug, Clone)]
struct ObjectInfo {
    id: usize,
    morton_code: u32,
    bbox: BoundingBox,
}

impl BVH {
    pub fn new(world_size: f32) -> Self {
        BVH {
            guid: Uuid::new_v4().to_string(),
            name: "my_bvh".to_string(),
            root: None,
            world_size,
        }
    }

    pub fn from_boxes(bounding_boxes: &[BoundingBox], world_size: f32) -> Self {
        let mut bvh = Self::new(world_size);
        bvh.build(bounding_boxes);
        bvh
    }

    pub fn build(&mut self, bounding_boxes: &[BoundingBox]) {
        if bounding_boxes.is_empty() {
            self.root = None;
            return;
        }

        // Create list of objects with their Morton codes
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
                ObjectInfo {
                    id: i,
                    morton_code,
                    bbox: bbox.clone(),
                }
            })
            .collect();

        // Sort by Morton code for spatial locality
        objects.sort_by_key(|obj| obj.morton_code);

        // Build tree recursively
        let len = objects.len();
        self.root = Some(self.create_subtree(&mut objects, 0, len - 1));
    }

    fn create_subtree(&self, objects: &mut [ObjectInfo], begin: usize, end: usize) -> Box<BVHNode> {
        if begin == end {
            // Create leaf node
            let mut node = BVHNode::new();
            node.object_id = objects[begin].id as i32;
            node.aabb = Some(objects[begin].bbox.clone());
            Box::new(node)
        } else {
            // Create internal node
            let mid = (begin + end) / 2;
            let mut node = BVHNode::new();

            // Recursively create children
            let left = self.create_subtree(objects, begin, mid);
            let right = self.create_subtree(objects, mid + 1, end);

            // Merge children's AABBs
            if let (Some(left_aabb), Some(right_aabb)) = (&left.aabb, &right.aabb) {
                node.aabb = Some(self.merge_aabb(left_aabb, right_aabb));
            }

            node.left = Some(left);
            node.right = Some(right);

            Box::new(node)
        }
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
        if self.root.is_none() {
            return (Vec::new(), 0);
        }

        let mut collisions = Vec::new();
        let mut check_count = 0;

        self.find_collisions_recursive(
            object_id,
            query_bbox,
            self.root.as_ref().unwrap(),
            bounding_boxes,
            &mut collisions,
            &mut check_count,
        );

        (collisions, check_count)
    }

    fn find_collisions_recursive(
        &self,
        object_id: usize,
        query_bbox: &BoundingBox,
        node: &BVHNode,
        bounding_boxes: &[BoundingBox],
        collisions: &mut Vec<usize>,
        check_count: &mut i32,
    ) {
        *check_count += 1;

        // Early exit if query doesn't intersect this node's AABB
        if let Some(node_aabb) = &node.aabb {
            if !self.aabb_intersect(query_bbox, node_aabb) {
                return;
            }
        }

        // If leaf node, check for collision
        if node.is_leaf() {
            let node_object_id = node.object_id as usize;
            // Don't check collision with self
            if node_object_id != object_id
                && self.aabb_intersect(query_bbox, &bounding_boxes[node_object_id])
            {
                collisions.push(node_object_id);
            }
            return;
        }

        // Recurse through children
        if let Some(left) = &node.left {
            self.find_collisions_recursive(
                object_id,
                query_bbox,
                left,
                bounding_boxes,
                collisions,
                check_count,
            );
        }
        if let Some(right) = &node.right {
            self.find_collisions_recursive(
                object_id,
                query_bbox,
                right,
                bounding_boxes,
                collisions,
                check_count,
            );
        }
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
        let mut all_collisions = Vec::new();
        let mut colliding_objects = HashSet::new();
        let mut total_checks = 0;

        for (i, bbox) in bounding_boxes.iter().enumerate() {
            let (collisions, checks) = self.find_collisions(i, bbox, bounding_boxes);
            total_checks += checks;

            // Add unique collision pairs (avoid duplicates)
            for j in collisions {
                if i < j {
                    // Only add each pair once
                    all_collisions.push((i, j));
                    colliding_objects.insert(i);
                    colliding_objects.insert(j);
                }
            }
        }

        let mut colliding_indices: Vec<usize> = colliding_objects.into_iter().collect();
        colliding_indices.sort();

        (all_collisions, colliding_indices, total_checks)
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

pub fn calculate_morton_code(x: f32, y: f32, z: f32, world_size: f32) -> u32 {
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
