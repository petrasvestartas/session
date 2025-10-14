"""Tests for BVH (Boundary Volume Hierarchy)."""

from .bvh import BVH, BVHNode, calculate_morton_code, expand_bits
from .boundingbox import BoundingBox
from .point import Point
from .vector import Vector


def test_expand_bits():
    """Test bit expansion for Morton codes."""
    assert expand_bits(0) == 0
    assert expand_bits(1) == 1
    assert expand_bits(2) == 8
    assert expand_bits(3) == 9
    # 1023 in binary is 0b1111111111 (10 bits)
    # After expansion, should have pattern with zeros inserted
    result = expand_bits(1023)
    assert result > 0  # Should be non-zero


def test_morton_code_origin():
    """Test Morton code at world origin."""
    code = calculate_morton_code(0.0, 0.0, 0.0, 100.0)
    assert code >= 0
    assert code < (1 << 30)  # 30-bit code


def test_morton_code_corners():
    """Test Morton codes at world corners."""
    world_size = 100.0
    
    # Corner at (-50, -50, -50) should give code 0
    code_min = calculate_morton_code(-50.0, -50.0, -50.0, world_size)
    assert code_min == 0
    
    # Corner at (50, 50, 50) should give maximum code
    code_max = calculate_morton_code(50.0, 50.0, 50.0, world_size)
    assert code_max == 0x3FFFFFFF  # Maximum 30-bit value


def test_morton_code_spatial_locality():
    """Test that nearby points have similar Morton codes."""
    # Two nearby points should have similar codes
    code1 = calculate_morton_code(10.0, 10.0, 10.0)
    code2 = calculate_morton_code(10.1, 10.1, 10.1)
    
    # Two far apart points should have different codes
    code3 = calculate_morton_code(-40.0, -40.0, -40.0)
    
    # Nearby points should be closer in Morton space
    diff_nearby = abs(code1 - code2)
    diff_far = abs(code1 - code3)
    assert diff_nearby < diff_far


def test_bvh_node_creation():
    """Test BVHNode creation."""
    node = BVHNode()
    assert node.guid is not None
    assert node.left is None
    assert node.right is None
    assert node.object_id == -1
    assert node.aabb is None
    assert not node.is_leaf()


def test_bvh_node_leaf():
    """Test leaf node detection."""
    node = BVHNode()
    assert not node.is_leaf()
    
    node.object_id = 5
    assert node.is_leaf()


def test_bvh_creation():
    """Test BVH creation."""
    bvh = BVH(world_size=100.0)
    assert bvh.guid is not None
    assert bvh.name == "my_bvh"
    assert bvh.root is None
    assert bvh.world_size == 100.0


def test_bvh_build_empty():
    """Test building BVH with empty list."""
    bvh = BVH()
    bvh.build([])
    assert bvh.root is None


def test_bvh_build_single():
    """Test building BVH with single bounding box."""
    bbox = BoundingBox(
        Point(0, 0, 0),
        Vector(1, 0, 0),
        Vector(0, 1, 0),
        Vector(0, 0, 1),
        Vector(1, 1, 1)
    )
    
    bvh = BVH()
    bvh.build([bbox])
    
    assert bvh.root is not None
    assert bvh.root.is_leaf()
    assert bvh.root.object_id == 0
    assert bvh.root.aabb == bbox


def test_bvh_build_multiple():
    """Test building BVH with multiple bounding boxes."""
    bboxes = [
        BoundingBox(
            Point(-10, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1)
        ),
        BoundingBox(
            Point(10, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1)
        ),
        BoundingBox(
            Point(0, 10, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1)
        ),
    ]
    
    bvh = BVH()
    bvh.build(bboxes)
    
    assert bvh.root is not None
    assert not bvh.root.is_leaf()
    assert bvh.root.left is not None
    assert bvh.root.right is not None


def test_bvh_aabb_intersect():
    """Test AABB intersection detection."""
    bvh = BVH()
    
    # Overlapping boxes
    bbox1 = BoundingBox(
        Point(0, 0, 0),
        Vector(1, 0, 0),
        Vector(0, 1, 0),
        Vector(0, 0, 1),
        Vector(1, 1, 1)
    )
    bbox2 = BoundingBox(
        Point(0.5, 0, 0),
        Vector(1, 0, 0),
        Vector(0, 1, 0),
        Vector(0, 0, 1),
        Vector(1, 1, 1)
    )
    assert bvh._aabb_intersect(bbox1, bbox2)
    
    # Non-overlapping boxes
    bbox3 = BoundingBox(
        Point(10, 0, 0),
        Vector(1, 0, 0),
        Vector(0, 1, 0),
        Vector(0, 0, 1),
        Vector(1, 1, 1)
    )
    assert not bvh._aabb_intersect(bbox1, bbox3)


def test_bvh_find_collisions_no_collision():
    """Test collision detection with no collisions."""
    bboxes = [
        BoundingBox(
            Point(-10, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1)
        ),
        BoundingBox(
            Point(10, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1)
        ),
    ]
    
    bvh = BVH()
    bvh.build(bboxes)
    
    collisions, checks = bvh.find_collisions(0, bboxes[0], bboxes)
    assert len(collisions) == 0
    assert checks > 0


def test_bvh_find_collisions_with_collision():
    """Test collision detection with overlapping boxes."""
    bboxes = [
        BoundingBox(
            Point(0, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(2, 2, 2)
        ),
        BoundingBox(
            Point(1, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(2, 2, 2)
        ),
    ]
    
    bvh = BVH()
    bvh.build(bboxes)
    
    collisions, checks = bvh.find_collisions(0, bboxes[0], bboxes)
    assert len(collisions) == 1
    assert 1 in collisions


def test_bvh_check_all_collisions():
    """Test checking all pairwise collisions."""
    bboxes = [
        BoundingBox(Point(0, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1)),
        BoundingBox(Point(0.5, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1)),
        BoundingBox(Point(10, 0, 0), Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), Vector(1, 1, 1)),
    ]
    
    bvh = BVH()
    bvh.build(bboxes)
    
    collisions, checks = bvh.check_all_collisions(bboxes)
    
    # Boxes 0 and 1 should collide
    assert len(collisions) == 1
    assert (0, 1) in collisions
    assert checks > 0


def test_bvh_merge_aabb():
    """Test AABB merging."""
    bvh = BVH()
    
    bbox1 = BoundingBox(
        Point(0, 0, 0),
        Vector(1, 0, 0),
        Vector(0, 1, 0),
        Vector(0, 0, 1),
        Vector(1, 1, 1)
    )
    bbox2 = BoundingBox(
        Point(5, 0, 0),
        Vector(1, 0, 0),
        Vector(0, 1, 0),
        Vector(0, 0, 1),
        Vector(1, 1, 1)
    )
    
    merged = bvh._merge_aabb(bbox1, bbox2)
    
    # Merged box should encompass both
    assert merged.center.x == 2.5  # Midpoint between 0 and 5
    assert merged.half_size.x == 3.5  # Half of distance from -1 to 6


def test_bvh_performance_many_boxes():
    """Test BVH performance with many boxes."""
    import random
    random.seed(42)
    
    # Create 100 random boxes
    bboxes = []
    for i in range(100):
        center = Point(
            random.uniform(-40, 40),
            random.uniform(-40, 40),
            random.uniform(-40, 40)
        )
        half_size = Vector(
            random.uniform(0.5, 2),
            random.uniform(0.5, 2),
            random.uniform(0.5, 2)
        )
        bbox = BoundingBox(
            center,
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            half_size
        )
        bboxes.append(bbox)
    
    # Build BVH
    bvh = BVH()
    bvh.build(bboxes)
    
    # Check collisions
    collisions, checks = bvh.check_all_collisions(bboxes)
    
    # BVH should perform fewer checks than naive O(n²)
    naive_checks = len(bboxes) * (len(bboxes) - 1) // 2
    assert checks < naive_checks
    
    # Should find some collisions
    assert isinstance(collisions, list)
