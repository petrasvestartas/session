use crate::boundingbox::BoundingBox;
/// Tests for BVH (Boundary Volume Hierarchy).
/// These tests match the Python test suite in bvh_test.py
use crate::bvh::*;
use crate::point::Point;
use crate::vector::Vector;
use rand::prelude::*;
use std::time::Instant;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_bits() {
        // Test bit expansion for Morton codes
        assert_eq!(expand_bits(0), 0);
        assert_eq!(expand_bits(1), 1);
        assert_eq!(expand_bits(2), 8);
        assert_eq!(expand_bits(3), 9);

        // 1023 in binary is 0b1111111111 (10 bits)
        // After expansion, should have pattern with zeros inserted
        let result = expand_bits(1023);
        assert!(result > 0); // Should be non-zero
    }

    #[test]
    fn test_morton_code_origin() {
        // Test Morton code at world origin.
        let code = calculate_morton_code(0.0, 0.0, 0.0, 100.0);
        assert!(code < (1u32 << 30)); // 30-bit code
    }

    #[test]
    fn test_morton_code_corners() {
        // Test Morton codes at world corners.
        let world_size = 100.0;

        // Corner at (-50, -50, -50) should give code 0
        let code_min = calculate_morton_code(-50.0, -50.0, -50.0, world_size);
        assert_eq!(code_min, 0);

        // Corner at (50, 50, 50) should give maximum code
        let code_max = calculate_morton_code(50.0, 50.0, 50.0, world_size);
        assert_eq!(code_max, 0x3FFFFFFF); // Maximum 30-bit value
    }

    #[test]
    fn test_morton_code_spatial_locality() {
        // Test that nearby points have similar Morton codes.
        // Two nearby points should have similar codes
        let code1 = calculate_morton_code(10.0, 10.0, 10.0, 100.0);
        let code2 = calculate_morton_code(10.1, 10.1, 10.1, 100.0);

        // Two far apart points should have different codes
        let code3 = calculate_morton_code(-40.0, -40.0, -40.0, 100.0);

        // Nearby points should be closer in Morton space
        let diff_nearby = code1.abs_diff(code2);
        let diff_far = code1.abs_diff(code3);
        assert!(diff_nearby < diff_far);
    }

    #[test]
    fn test_bvh_node_creation() {
        // Test BVHNode creation.
        let node = BVHNode::new();
        assert!(!node.guid.is_empty());
        assert!(node.left.is_none());
        assert!(node.right.is_none());
        assert_eq!(node.object_id, -1);
        assert!(node.aabb.is_none());
        assert!(!node.is_leaf());
    }

    #[test]
    fn test_bvh_node_leaf() {
        // Test leaf node detection.
        let mut node = BVHNode::new();
        assert!(!node.is_leaf());

        node.object_id = 5;
        assert!(node.is_leaf());
    }

    #[test]
    fn test_bvh_creation() {
        // Test BVH creation.
        let bvh = BVH::new();
        assert!(!bvh.guid.is_empty());
        assert_eq!(bvh.name, "my_bvh");
        assert!(bvh.root.is_none());
        assert_eq!(bvh.world_size, 1000.0); // Default value
    }

    #[test]
    fn test_bvh_build_empty() {
        // Test building BVH with empty list.
        let boxes: Vec<BoundingBox> = vec![];
        let bvh = BVH::from_boxes(&boxes, 100.0);
        assert!(bvh.root.is_none());
    }

    #[test]
    fn test_bvh_build_single() {
        // Test building BVH with single bounding box.
        let bbox = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 1.0, 1.0),
        );
        let boxes = vec![bbox.clone()];

        let bvh = BVH::from_boxes(&boxes, 100.0);

        // Verify BVH works by querying it
        let (collisions, _checks) = bvh.find_collisions(0, &bbox, &boxes);
        assert_eq!(collisions.len(), 0); // Should not collide with itself
    }

    #[test]
    fn test_bvh_build_multiple() {
        // Test building BVH with multiple bounding boxes.
        let bboxes = vec![
            BoundingBox::new(
                Point::new(-10.0, 0.0, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                Vector::new(1.0, 1.0, 1.0),
            ),
            BoundingBox::new(
                Point::new(10.0, 0.0, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                Vector::new(1.0, 1.0, 1.0),
            ),
            BoundingBox::new(
                Point::new(0.0, 10.0, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                Vector::new(1.0, 1.0, 1.0),
            ),
        ];

        let bvh = BVH::from_boxes(&bboxes, 100.0);

        // Verify BVH works by performing pairwise collision detection
        let (pairs, _indices, checks) = bvh.check_all_collisions(&bboxes);
        assert_eq!(pairs.len(), 0); // These boxes don't overlap
        assert!(checks > 0); // But we should have checked some nodes
    }

    #[test]
    fn test_bvh_aabb_intersect() {
        // Test AABB intersection detection.
        let bvh = BVH::new();

        // Overlapping boxes
        let bbox1 = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 1.0, 1.0),
        );
        let bbox2 = BoundingBox::new(
            Point::new(0.5, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 1.0, 1.0),
        );
        assert!(bvh.aabb_intersect(&bbox1, &bbox2));

        // Non-overlapping boxes
        let bbox3 = BoundingBox::new(
            Point::new(10.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 1.0, 1.0),
        );
        assert!(!bvh.aabb_intersect(&bbox1, &bbox3));
    }

    #[test]
    fn test_bvh_find_collisions_no_collision() {
        // Test collision detection with no collisions.
        let bboxes = vec![
            BoundingBox::new(
                Point::new(-10.0, 0.0, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                Vector::new(1.0, 1.0, 1.0),
            ),
            BoundingBox::new(
                Point::new(10.0, 0.0, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                Vector::new(1.0, 1.0, 1.0),
            ),
        ];

        let bvh = BVH::from_boxes(&bboxes, 100.0);

        let (collisions, checks) = bvh.find_collisions(0, &bboxes[0], &bboxes);
        assert_eq!(collisions.len(), 0);
        assert!(checks > 0);
    }

    #[test]
    fn test_bvh_find_collisions_with_collision() {
        // Test collision detection with overlapping boxes.
        let bboxes = vec![
            BoundingBox::new(
                Point::new(0.0, 0.0, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                Vector::new(2.0, 2.0, 2.0),
            ),
            BoundingBox::new(
                Point::new(1.0, 0.0, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                Vector::new(2.0, 2.0, 2.0),
            ),
        ];

        let bvh = BVH::from_boxes(&bboxes, 100.0);

        let (collisions, _checks) = bvh.find_collisions(0, &bboxes[0], &bboxes);
        assert_eq!(collisions.len(), 1);
        assert!(collisions.contains(&1));
    }

    #[test]
    fn test_bvh_check_all_collisions() {
        // Test checking all pairwise collisions.
        let bboxes = vec![
            BoundingBox::new(
                Point::new(0.0, 0.0, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                Vector::new(1.0, 1.0, 1.0),
            ),
            BoundingBox::new(
                Point::new(0.5, 0.0, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                Vector::new(1.0, 1.0, 1.0),
            ),
            BoundingBox::new(
                Point::new(10.0, 0.0, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                Vector::new(1.0, 1.0, 1.0),
            ),
        ];

        let bvh = BVH::from_boxes(&bboxes, 100.0);

        let (collisions, colliding_indices, checks) = bvh.check_all_collisions(&bboxes);

        // Boxes 0 and 1 should collide
        assert_eq!(collisions.len(), 1);
        assert!(collisions.contains(&(0, 1)));
        assert_eq!(colliding_indices, vec![0, 1]);
        assert!(checks > 0);
    }

    #[test]
    fn test_bvh_merge_aabb() {
        // Test AABB merging.
        let bvh = BVH::new();

        let bbox1 = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 1.0, 1.0),
        );
        let bbox2 = BoundingBox::new(
            Point::new(5.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 1.0, 1.0),
        );

        let merged = bvh.merge_aabb(&bbox1, &bbox2);

        // Merged box should encompass both
        assert!((merged.center.x() - 2.5).abs() < 0.001); // Midpoint between 0 and 5
        assert!((merged.half_size.x() - 3.5).abs() < 0.001); // Half of distance from -1 to 6
    }

    #[test]
    fn test_bvh_performance_many_boxes() {
        // Test BVH performance with many boxes.
        let mut rng = StdRng::seed_from_u64(42);

        // Create 100 random boxes
        let mut bboxes = Vec::new();
        for _i in 0..100 {
            let center = Point::new(
                rng.gen_range(-40.0..40.0),
                rng.gen_range(-40.0..40.0),
                rng.gen_range(-40.0..40.0),
            );
            let half_size = Vector::new(
                rng.gen_range(0.5..2.0),
                rng.gen_range(0.5..2.0),
                rng.gen_range(0.5..2.0),
            );
            let bbox = BoundingBox::new(
                center,
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 0.0, 1.0),
                half_size,
            );
            bboxes.push(bbox);
        }

        // Build BVH
        let bvh = BVH::from_boxes(&bboxes, 100.0);

        // Check collisions
        let (_collisions, _colliding_indices, checks) = bvh.check_all_collisions(&bboxes);

        // BVH should perform fewer checks than naive O(n²)
        let naive_checks = (bboxes.len() * (bboxes.len() - 1)) / 2;
        assert!((checks as usize) < naive_checks);
    }

    #[test]
    fn test_bvh_performance_1000_boxes() {
        // Test BVH performance with 1000 random boxes.
        println!("Testing BVH with 1000 random boxes...");

        // Create 1000 random boxes with seed 42 for reproducibility
        let mut rng = StdRng::seed_from_u64(42);

        let mut boxes = Vec::new();
        for _i in 0..1000 {
            // Random center position
            let center = Point::new(
                rng.gen_range(-50.0..50.0),
                rng.gen_range(-50.0..50.0),
                rng.gen_range(-50.0..50.0),
            );
            // Random half-size (dimensions)
            let half_size = Vector::new(
                rng.gen_range(0.5..3.0),
                rng.gen_range(0.5..3.0),
                rng.gen_range(0.5..3.0),
            );
            // Create axis-aligned bounding box
            let bbox = BoundingBox::new(
                center,
                Vector::new(1.0, 0.0, 0.0), // X axis
                Vector::new(0.0, 1.0, 0.0), // Y axis
                Vector::new(0.0, 0.0, 1.0), // Z axis
                half_size,
            );
            boxes.push(bbox);
        }

        // Print min/max corners using new API for first few boxes
        for (i, bbox) in boxes.iter().take(5).enumerate() {
            let min_corner = bbox.min_point();
            let max_corner = bbox.max_point();
            println!(
                "Box {} - Min: ({}, {}, {}), Max: ({}, {}, {})",
                i + 1,
                min_corner.x(),
                min_corner.y(),
                min_corner.z(),
                max_corner.x(),
                max_corner.y(),
                max_corner.z()
            );
        }

        // Build BVH and measure time
        let start = Instant::now();
        let bvh = BVH::from_boxes(&boxes, 100.0);
        let build_time = start.elapsed();

        // Check collisions and measure time
        let start = Instant::now();
        let (collisions, colliding_indices, checks) = bvh.check_all_collisions(&boxes);
        let collision_time = start.elapsed();

        // BVH should perform fewer checks than naive O(n²)
        let naive_checks = (boxes.len() * (boxes.len() - 1)) / 2;
        assert!((checks as usize) < naive_checks);

        println!("Collisions found: {}", collisions.len());
        println!("Colliding objects: {}", colliding_indices.len());
        println!("BVH build time: {build_time:?}");
        println!("BVH collision time: {collision_time:?} ({checks} checks)");
        println!("Naive would need: {naive_checks} checks");
        println!(
            "Check reduction: {:.1}%",
            100.0 * (1.0 - checks as f64 / naive_checks as f64)
        );

        // Should find some collisions
        assert!(!collisions.is_empty());
        assert!(!colliding_indices.is_empty());
    }
}
