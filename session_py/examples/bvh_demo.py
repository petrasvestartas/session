"""BVH Collision Detection Demo

This example demonstrates how to use the BVH (Boundary Volume Hierarchy)
for efficient collision detection among many objects.
"""

import random
import time
from session_py import BVH, BoundingBox, Point, Vector


def create_random_boxes(count, world_size=100.0, min_size=0.5, max_size=3.0):
    """Create random bounding boxes in 3D space.

    Parameters
    ----------
    count : int
        Number of boxes to create.
    world_size : float
        Size of the world bounds.
    min_size : float
        Minimum box dimension.
    max_size : float
        Maximum box dimension.

    Returns
    -------
    list of BoundingBox
        List of randomly positioned and sized bounding boxes.
    """
    boxes = []
    for i in range(count):
        # Random center position
        center = Point(
            random.uniform(-world_size / 2, world_size / 2),
            random.uniform(-world_size / 2, world_size / 2),
            random.uniform(-world_size / 2, world_size / 2),
        )

        # Random half-size (dimensions)
        half_size = Vector(
            random.uniform(min_size, max_size) / 2,
            random.uniform(min_size, max_size) / 2,
            random.uniform(min_size, max_size) / 2,
        )

        # Create axis-aligned bounding box
        bbox = BoundingBox(
            center,
            Vector(1, 0, 0),  # X axis
            Vector(0, 1, 0),  # Y axis
            Vector(0, 0, 1),  # Z axis
            half_size,
        )
        boxes.append(bbox)

    return boxes


def naive_collision_detection(boxes):
    """Naive O(n²) collision detection for comparison.

    Parameters
    ----------
    boxes : list of BoundingBox
        List of bounding boxes to check.

    Returns
    -------
    tuple
        (collisions, check_count) where collisions is a list of (i, j) pairs.
    """
    collisions = []
    check_count = 0

    for i in range(len(boxes)):
        for j in range(i + 1, len(boxes)):
            check_count += 1
            # Simple AABB intersection test
            box1 = boxes[i]
            box2 = boxes[j]

            min1_x = box1.center.x - box1.half_size.x
            max1_x = box1.center.x + box1.half_size.x
            min2_x = box2.center.x - box2.half_size.x
            max2_x = box2.center.x + box2.half_size.x

            min1_y = box1.center.y - box1.half_size.y
            max1_y = box1.center.y + box1.half_size.y
            min2_y = box2.center.y - box2.half_size.y
            max2_y = box2.center.y + box2.half_size.y

            min1_z = box1.center.z - box1.half_size.z
            max1_z = box1.center.z + box1.half_size.z
            min2_z = box2.center.z - box2.half_size.z
            max2_z = box2.center.z + box2.half_size.z

            if (
                min1_x <= max2_x
                and max1_x >= min2_x
                and min1_y <= max2_y
                and max1_y >= min2_y
                and min1_z <= max2_z
                and max1_z >= min2_z
            ):
                collisions.append((i, j))

    return collisions, check_count


def main():
    """Run BVH collision detection demo."""
    print("=" * 60)
    print("BVH Collision Detection Demo")
    print("=" * 60)

    # Set random seed for reproducibility
    random.seed(42)

    # Test with different box counts
    box_counts = [10, 50, 100, 500, 1000]

    for count in box_counts:
        print(f"\n--- Testing with {count} boxes ---")

        # Create random boxes
        boxes = create_random_boxes(count, world_size=100.0)

        # Build BVH
        bvh = BVH(world_size=100.0)
        start_time = time.time()
        bvh.build(boxes)
        build_time = (time.time() - start_time) * 1000  # Convert to ms

        # BVH collision detection
        start_time = time.time()
        bvh_collisions, bvh_checks = bvh.check_all_collisions(boxes)
        bvh_time = (time.time() - start_time) * 1000  # Convert to ms

        # Naive collision detection (only for smaller counts)
        if count <= 100:
            start_time = time.time()
            naive_collisions, naive_checks = naive_collision_detection(boxes)
            naive_time = (time.time() - start_time) * 1000

            # Verify results match
            assert len(bvh_collisions) == len(
                naive_collisions
            ), f"Collision count mismatch: BVH={len(bvh_collisions)}, Naive={len(naive_collisions)}"

            speedup = naive_time / bvh_time if bvh_time > 0 else float("inf")
            check_reduction = (
                (1 - bvh_checks / naive_checks) * 100 if naive_checks > 0 else 0
            )

            print(f"  Collisions found: {len(bvh_collisions)}")
            print(f"  BVH build time: {build_time:.2f} ms")
            print(f"  BVH check time: {bvh_time:.2f} ms ({bvh_checks:,} checks)")
            print(f"  Naive check time: {naive_time:.2f} ms ({naive_checks:,} checks)")
            print(f"  Speedup: {speedup:.1f}x")
            print(f"  Check reduction: {check_reduction:.1f}%")
        else:
            print(f"  Collisions found: {len(bvh_collisions)}")
            print(f"  BVH build time: {build_time:.2f} ms")
            print(f"  BVH check time: {bvh_time:.2f} ms ({bvh_checks:,} checks)")
            print(f"  (Naive method skipped for performance)")

    print("\n" + "=" * 60)
    print("Demo complete!")
    print("=" * 60)


if __name__ == "__main__":
    main()
