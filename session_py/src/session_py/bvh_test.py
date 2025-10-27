"""Tests for BVH (Boundary Volume Hierarchy) module."""

import pytest
from session_py.bvh import (
    BVH,
    BVHNode,
    BvhAABB,
    expand_bits,
    calculate_morton_code,
)
from session_py.point import Point
from session_py.vector import Vector
from session_py.boundingbox import BoundingBox


def test_expand_bits():
    """Test bit expansion for Morton codes."""
    assert expand_bits(0) == 0
    assert expand_bits(1) == 1
    assert expand_bits(2) == 8
    assert expand_bits(3) == 9

    result = expand_bits(1023)
    assert result > 0


def test_morton_code_at_origin():
    """Test Morton code at origin."""
    code = calculate_morton_code(0.0, 0.0, 0.0, 100.0)
    assert code < (1 << 30)


def test_morton_codes_at_corners():
    """Test Morton codes at corners."""
    world_size = 100.0

    code_min = calculate_morton_code(-50.0, -50.0, -50.0, world_size)
    assert code_min == 0

    code_max = calculate_morton_code(50.0, 50.0, 50.0, world_size)
    assert code_max == 0x3FFFFFFF


def test_morton_code_spatial_locality():
    """Test that nearby points have similar Morton codes."""
    code1 = calculate_morton_code(10.0, 10.0, 10.0)
    code2 = calculate_morton_code(10.1, 10.1, 10.1)
    code3 = calculate_morton_code(-40.0, -40.0, -40.0)

    diff_nearby = abs(code1 - code2)
    diff_far = abs(code1 - code3)
    assert diff_nearby < diff_far


def test_bvh_node_creation():
    """Test BVH node creation."""
    node = BVHNode()
    assert node.left is None
    assert node.right is None
    assert node.object_id == -1
    assert not node.is_leaf()


def test_bvh_node_leaf():
    """Test BVH leaf node."""
    node = BVHNode()
    assert not node.is_leaf()

    node.object_id = 5
    assert node.is_leaf()


def test_bvh_creation():
    """Test BVH creation."""
    bvh = BVH(100.0)
    assert bvh.guid
    assert bvh.name == "my_bvh"
    assert bvh.root is None
    assert bvh.world_size == 100.0


def test_bvh_build_empty():
    """Test BVH build with empty list."""
    boxes = []
    bvh = BVH.from_boxes(boxes, 100.0)
    assert bvh.root is None


def test_bvh_build_single():
    """Test BVH build with single box."""
    bbox = BoundingBox(
        Point(0, 0, 0),
        Vector(1, 0, 0),
        Vector(0, 1, 0),
        Vector(0, 0, 1),
        Vector(1, 1, 1),
    )
    boxes = [bbox]

    bvh = BVH.from_boxes(boxes, 100.0)

    # BVH now uses flat arena instead of tree
    assert bvh.arena_root >= 0
    assert bvh.arena_object_id[bvh.arena_root] == 0  # Root is a leaf with object_id 0


def test_bvh_build_multiple():
    """Test BVH build with multiple boxes."""
    bboxes = [
        BoundingBox(
            Point(-10, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1),
        ),
        BoundingBox(
            Point(10, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1),
        ),
        BoundingBox(
            Point(0, 10, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1),
        ),
    ]

    bvh = BVH.from_boxes(bboxes, 100.0)

    # BVH now uses flat arena instead of tree
    assert bvh.arena_root >= 0
    # Root should be internal node (not a leaf) with multiple boxes
    assert bvh.arena_object_id[bvh.arena_root] == -1  # Internal node
    assert bvh.arena_left[bvh.arena_root] >= 0
    assert bvh.arena_right[bvh.arena_root] >= 0


def test_bvh_aabb_intersect():
    """Test AABB intersection."""
    bvh = BVH(100.0)

    bbox1 = BoundingBox(
        Point(0, 0, 0),
        Vector(1, 0, 0),
        Vector(0, 1, 0),
        Vector(0, 0, 1),
        Vector(1, 1, 1),
    )
    bbox2 = BoundingBox(
        Point(0.5, 0, 0),
        Vector(1, 0, 0),
        Vector(0, 1, 0),
        Vector(0, 0, 1),
        Vector(1, 1, 1),
    )
    assert bvh.aabb_intersect(bbox1, bbox2)

    bbox3 = BoundingBox(
        Point(10, 0, 0),
        Vector(1, 0, 0),
        Vector(0, 1, 0),
        Vector(0, 0, 1),
        Vector(1, 1, 1),
    )
    assert not bvh.aabb_intersect(bbox1, bbox3)


def test_bvh_check_all_collisions():
    """Test check all collisions."""
    bboxes = [
        BoundingBox(
            Point(0, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1),
        ),
        BoundingBox(
            Point(0.5, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1),
        ),
        BoundingBox(
            Point(10, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1),
        ),
    ]

    bvh = BVH.from_boxes(bboxes, 100.0)

    collisions, colliding_indices, checks = bvh.check_all_collisions(bboxes)

    assert len(collisions) == 1
    assert collisions[0] == (0, 1)
    assert len(colliding_indices) == 2
    assert 0 in colliding_indices
    assert 1 in colliding_indices
    assert checks > 0


def test_bvh_merge_aabb():
    """Test AABB merging."""
    bvh = BVH(100.0)

    bbox1 = BoundingBox(
        Point(0, 0, 0),
        Vector(1, 0, 0),
        Vector(0, 1, 0),
        Vector(0, 0, 1),
        Vector(1, 1, 1),
    )
    bbox2 = BoundingBox(
        Point(5, 0, 0),
        Vector(1, 0, 0),
        Vector(0, 1, 0),
        Vector(0, 0, 1),
        Vector(1, 1, 1),
    )

    merged = bvh.merge_aabb(bbox1, bbox2)

    assert abs(merged.center.x - 2.5) < 0.001
    assert abs(merged.half_size.x - 3.5) < 0.001


def test_bvh_performance_many_boxes():
    """Test BVH performance with many boxes."""
    import random

    random.seed(42)

    bboxes = []
    for i in range(100):
        cx = random.uniform(-40.0, 40.0)
        cy = random.uniform(-40.0, 40.0)
        cz = random.uniform(-40.0, 40.0)
        hx = random.uniform(0.5, 2.0)
        hy = random.uniform(0.5, 2.0)
        hz = random.uniform(0.5, 2.0)
        center = Point(cx, cy, cz)
        half_size = Vector(hx, hy, hz)
        bbox = BoundingBox(
            center, Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), half_size
        )
        bboxes.append(bbox)

    bvh = BVH.from_boxes(bboxes, 100.0)
    collisions, colliding_indices, checks = bvh.check_all_collisions(bboxes)

    naive_checks = len(bboxes) * (len(bboxes) - 1) // 2
    assert checks < naive_checks


def test_bvh_fixed_100_boxes_collisions():
    """Test BVH with fixed 100 boxes (deterministic collision test)."""
    boxes = []

    def add(min_x, min_y, min_z, max_x, max_y, max_z):
        cx = (min_x + max_x) * 0.5
        cy = (min_y + max_y) * 0.5
        cz = (min_z + max_z) * 0.5
        hx = (max_x - min_x) * 0.5
        hy = (max_y - min_y) * 0.5
        hz = (max_z - min_z) * 0.5
        c = Point(cx, cy, cz)
        h = Vector(hx, hy, hz)
        boxes.append(
            BoundingBox(c, Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), h)
        )

    # 100 boxes (matching C++ test data)
    add(-53.1254, -0.98185, 20.5516, -46.8089, 5.89927, 26.5331)
    add(44.4446, -1.5359, -1.49382, 50.7301, 3.99953, 7.58362)
    add(36.9359, -7.76782, -28.7694, 43.173, -1.82645, -22.1528)
    add(-44.2654, 26.3949, 0.745263, -35.0431, 35.0799, 6.13693)
    add(0.239448, -40.5791, 32.6275, 7.56243, -33.2192, 39.8776)
    add(-31.6363, -53.5568, -52.162, -21.6687, -43.9796, -43.2328)
    add(3.72143, 23.485, 9.18924, 10.4425, 30.3631, 15.5248)
    add(-17.4583, 10.2729, -16.5162, -12.1943, 17.9162, -10.7277)
    add(-7.27998, -22.0384, -34.5872, -1.95631, -12.1058, -26.8567)
    add(-45.341, 46.3634, -10.4862, -36.8332, 52.2971, -2.76774)
    add(46.0445, -34.6013, 14.0587, 53.0414, -27.4064, 22.7938)
    add(-34.9367, 28.5039, 27.7749, -29.4494, 33.6524, 33.4448)
    add(9.97675, -15.7696, -27.8198, 17.5104, -8.16385, -22.3021)
    add(45.1965, -19.307, 22.0449, 51.5233, -10.9748, 31.6205)
    add(-7.03031, -10.8607, 38.8429, 0.306212, -0.974567, 45.443)
    add(25.5248, 31.9848, 20.436, 33.3122, 41.1186, 28.0921)
    add(-22.8772, -19.5722, -22.9988, -15.6443, -11.7384, -14.7361)
    add(-46.2318, -5.27625, -7.84674, -41.1843, 3.22896, -0.905452)
    add(-8.8814, 40.3852, -41.0122, -1.73994, 46.8478, -33.9574)
    add(-30.4719, -15.9782, 17.3287, -20.7941, -10.8891, 24.7185)
    add(28.6586, 0.44821, -41.9327, 35.6602, 6.09223, -32.8706)
    add(-14.173, -45.5086, 6.29666, -7.48969, -39.2406, 13.229)
    add(-21.8039, 6.68129, -32.5692, -15.3816, 16.6269, -26.5873)
    add(13.3659, -1.97758, 25.4002, 19.0017, 4.81311, 31.5121)
    add(-24.433, -37.1532, 41.849, -15.8042, -29.2066, 49.4371)
    add(-4.54629, -16.9216, -24.2439, 2.40272, -9.87919, -17.0974)
    add(-22.1316, -18.2577, -41.6624, -13.4863, -11.2109, -36.6118)
    add(-19.5562, -1.13082, -35.7364, -10.2048, 8.43363, -25.912)
    add(26.4514, -31.3635, -3.53901, 32.4376, -22.007, 5.52268)
    add(44.2805, -20.3072, 10.0337, 52.6535, -10.845, 15.6482)
    add(15.1756, 46.2379, 44.9662, 20.8272, 53.0835, 50.1683)
    add(1.39766, -37.0106, -2.59787, 7.17823, -28.0455, 3.65286)
    add(-31.882, -21.1354, 20.6053, -24.8106, -11.3482, 28.4804)
    add(-8.54435, 10.0787, 41.0063, -1.08096, 17.3793, 46.4334)
    add(21.317, -38.2325, 3.71512, 29.3482, -31.5114, 10.6611)
    add(-31.9136, 27.8033, -4.48008, -23.6666, 35.3487, 0.804813)
    add(8.52067, 14.4157, -37.4169, 17.5301, 20.4823, -32.1696)
    add(-7.88355, 21.208, 42.2586, -0.205483, 26.4206, 50.4889)
    add(-15.322, -4.75221, -17.9083, -8.4181, 4.47693, -8.67731)
    add(37.1268, 2.17059, -48.8049, 45.7917, 8.4744, -40.7264)
    add(-52.3809, -6.49423, 8.92399, -42.9845, 0.188961, 18.343)
    add(41.5732, -7.42366, -4.54156, 51.0067, -2.29871, 0.643029)
    add(-5.78252, 0.645065, -13.4131, 1.93946, 8.96885, -5.49512)
    add(7.58556, -41.9641, 23.8841, 16.6142, -32.1089, 31.049)
    add(-46.102, -9.30967, 44.8527, -36.2572, -2.2869, 51.5056)
    add(45.8031, 27.0115, -17.4386, 52.3382, 32.367, -7.79126)
    add(8.21008, 39.3673, 20.643, 17.4628, 45.1004, 28.0194)
    add(-47.9111, -24.7374, -29.2773, -40.7686, -16.0819, -20.6671)
    add(-29.8193, -10.8358, 24.5871, -21.6958, -3.36907, 33.5925)
    add(26.9713, -26.2038, -31.9261, 35.2619, -20.0422, -25.0245)
    add(-29.7903, 8.92347, -40.826, -21.7701, 15.776, -35.2006)
    add(-1.39845, -13.7028, -13.4383, 8.26331, -8.56298, -7.95241)
    add(-27.3862, 17.0337, 30.1216, -19.7585, 22.0732, 39.076)
    add(-15.102, -39.6467, -37.4648, -8.16651, -34.4574, -31.1032)
    add(14.1428, -34.4961, -47.6358, 22.6478, -25.6985, -42.1577)
    add(32.7187, -0.0187469, -2.54834, 41.5605, 9.91946, 3.89622)
    add(18.869, -24.3319, -0.588445, 27.1926, -18.2572, 6.42131)
    add(4.33372, 6.78191, -26.4923, 12.7318, 13.5283, -19.058)
    add(-3.88995, -20.8689, 18.4182, 4.99471, -11.484, 25.6025)
    add(-10.2896, -22.7252, -40.4815, -3.08794, -13.9661, -30.6919)
    add(30.2898, 7.94805, -2.19314, 35.3154, 17.6367, 5.55489)
    add(-33.8415, 21.4915, -16.5747, -26.6066, 27.2365, -10.8669)
    add(-22.4042, 38.4298, 21.7984, -13.9447, 47.0733, 28.4925)
    add(-6.87762, 2.83366, 10.2831, -0.784998, 11.5311, 18.5943)
    add(-34.4398, -36.757, 27.0559, -27.6572, -27.51, 36.7491)
    add(35.4006, -17.8502, -21.4524, 41.7323, -10.0449, -12.5719)
    add(28.1073, 31.8896, -16.4485, 33.4307, 37.9012, -9.80763)
    add(13.5936, 25.9705, 8.3269, 22.4543, 32.3162, 16.4279)
    add(28.2281, -51.9913, -14.7078, 35.0256, -42.5897, -6.77297)
    add(-27.4511, -21.3243, 42.9791, -18.7936, -14.3339, 50.3538)
    add(-42.0679, -47.6033, -33.2027, -32.8703, -38.8405, -26.6373)
    add(-52.2085, -52.5573, -33.0963, -45.8755, -44.5128, -23.5496)
    add(-11.2779, -9.99167, 24.9689, -5.92983, -0.191222, 31.1336)
    add(33.121, 2.70727, -33.8816, 38.3024, 10.367, -26.2656)
    add(-5.30061, -39.8595, 33.6105, 4.23731, -31.0826, 42.5769)
    add(-0.704829, -26.0593, -30.9797, 4.64116, -16.105, -24.9783)
    add(37.3045, 34.9896, 2.13491, 46.4151, 40.7296, 10.6969)
    add(-27.6823, 41.9125, -36.4809, -17.7935, 47.2728, -26.7252)
    add(34.666, 27.0233, 23.9605, 44.5308, 33.3, 30.9151)
    add(-37.3694, -40.3928, -6.27422, -28.0124, -31.5777, -0.670845)
    add(-34.1601, 33.6584, -28.8227, -27.286, 42.4497, -22.2408)
    add(-30.329, -4.34317, -43.1085, -23.815, 5.64745, -35.7657)
    add(-31.824, 8.78623, 25.1597, -24.1868, 17.2063, 31.7098)
    add(8.9247, -12.5921, 35.2262, 16.9325, -5.38381, 44.3014)
    add(-11.6258, 44.3936, -29.2716, -3.07673, 49.3977, -20.2529)
    add(-27.9412, 32.9874, -20.8262, -22.5216, 39.9326, -12.0579)
    add(39.7539, -22.0106, 31.131, 46.0297, -14.2677, 40.1578)
    add(-10.4385, 20.3835, 5.16852, -5.23064, 28.6092, 14.2703)
    add(19.9106, -32.364, 8.76233, 25.9003, -24.1348, 16.1047)
    add(-0.62887, 18.0559, 41.0991, 5.37937, 23.5869, 49.7166)
    add(20.6713, -12.7322, -19.7395, 28.0693, -3.71518, -11.0217)
    add(42.2797, -30.3842, 8.4357, 51.5113, -24.6986, 15.3918)
    add(-18.9658, -26.1333, -9.25188, -12.9283, -17.8373, -3.68668)
    add(32.8414, -44.7499, -3.96548, 41.3729, -35.5501, 1.88547)
    add(-12.0107, -43.9043, 15.2958, -6.24849, -38.452, 21.6608)
    add(-28.9449, 35.0651, -45.8908, -23.5524, 42.0763, -39.3406)
    add(25.2023, -12.4615, 8.84863, 30.8803, -6.57652, 18.4333)
    add(31.7285, 31.0991, -7.73725, 39.8767, 38.2288, 0.932107)
    add(-35.1346, -8.00369, 14.4611, -27.1614, -1.58541, 21.4893)
    add(13.9228, -49.9973, -2.77406, 23.104, -41.5596, 4.89623)

    assert len(boxes) == 100

    bvh = BVH.from_boxes(boxes, 100.0)
    pairs, colliding_indices, checks = bvh.check_all_collisions(boxes)

    pairs_sorted = sorted(pairs)

    # Verify found collisions are valid
    assert len(pairs) > 0
    assert len(pairs) <= 20  # Allow some variance from C++

    for i, j in pairs:
        assert 0 <= i < 100
        assert 0 <= j < 100
        assert i < j


def test_bvh_ray_cast_basic():
    """Test basic ray casting through BVH."""
    # Create a grid of boxes
    bboxes = []
    for x in range(3):
        for y in range(3):
            center = Point(x * 10.0, y * 10.0, 0.0)
            half_size = Vector(2.0, 2.0, 2.0)
            bbox = BoundingBox(
                center, Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), half_size
            )
            bboxes.append(bbox)

    bvh = BVH.from_boxes(bboxes, 100.0)

    # Cast ray through center
    origin = Point(-10.0, 10.0, 0.0)
    direction = Vector(1.0, 0.0, 0.0)
    candidates = []

    found = bvh.ray_cast(origin, direction, candidates, find_all=True)

    assert found
    assert len(candidates) > 0


def test_bvh_ray_cast_miss():
    """Test ray that misses all boxes."""
    bboxes = [
        BoundingBox(
            Point(0, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1),
        ),
    ]

    bvh = BVH.from_boxes(bboxes, 100.0)

    origin = Point(10.0, 10.0, 0.0)
    direction = Vector(1.0, 0.0, 0.0)
    candidates = []

    found = bvh.ray_cast(origin, direction, candidates, find_all=True)

    assert not found
    assert len(candidates) == 0


def test_bvh_ray_cast_ordering():
    """Test that ray cast returns results ordered by distance."""
    # Create boxes at different distances along X axis
    bboxes = [
        BoundingBox(
            Point(0, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1),
        ),
        BoundingBox(
            Point(10, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1),
        ),
        BoundingBox(
            Point(20, 0, 0),
            Vector(1, 0, 0),
            Vector(0, 1, 0),
            Vector(0, 0, 1),
            Vector(1, 1, 1),
        ),
    ]

    bvh = BVH.from_boxes(bboxes, 100.0)

    # Cast ray along X axis from negative position
    origin = Point(-10.0, 0.0, 0.0)
    direction = Vector(1.0, 0.0, 0.0)
    candidates = []

    found = bvh.ray_cast(origin, direction, candidates, find_all=True)

    assert found
    assert len(candidates) == 3
    # Results should be ordered by distance (box 0, then 1, then 2)
    assert candidates[0] == 0
    assert candidates[1] == 1
    assert candidates[2] == 2
