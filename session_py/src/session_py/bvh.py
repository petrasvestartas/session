"""Boundary Volume Hierarchy (BVH) for spatial acceleration.

This module implements a BVH tree using Morton codes for efficient spatial
partitioning and collision detection. Uses Linear BVH (LBVH) construction
algorithm from Karras 2012.
"""

import uuid
import heapq
import numpy as np
from typing import List, Tuple, Optional, NamedTuple
from .point import Point
from .vector import Vector
from .boundingbox import BoundingBox

# Try to import numba for JIT compilation
try:
    from numba import njit

    HAS_NUMBA = True
except ImportError:
    HAS_NUMBA = False

    # Fallback: no-op decorator
    def njit(*args, **kwargs):
        def decorator(func):
            return func

        if len(args) == 1 and callable(args[0]):
            return args[0]
        return decorator


class BvhAABB(NamedTuple):
    """Lightweight axis-aligned bounding box used internally by BVH."""

    cx: float
    cy: float
    cz: float
    hx: float
    hy: float
    hz: float


class BVHNode:
    """A node in the BVH tree."""

    __slots__ = ("left", "right", "object_id", "aabb")

    def __init__(self):
        self.left: Optional["BVHNode"] = None
        self.right: Optional["BVHNode"] = None
        self.object_id: int = -1
        self.aabb: Optional[BvhAABB] = None

    def is_leaf(self) -> bool:
        return self.object_id != -1


def expand_bits(v: int) -> int:
    """Expand bits for Morton code calculation."""
    v = (v * 0x00010001) & 0xFF0000FF
    v = (v * 0x00000101) & 0x0F00F00F
    v = (v * 0x00000011) & 0xC30C30C3
    v = (v * 0x00000005) & 0x49249249
    return v


def calculate_morton_code(
    x: float, y: float, z: float, world_size: float = 100.0
) -> int:
    """Calculate 3D Morton code (Z-order curve) for spatial hashing."""
    inv_world = 1.0 / world_size
    half_world = world_size * 0.5
    nx = (x + half_world) * inv_world
    ny = (y + half_world) * inv_world
    nz = (z + half_world) * inv_world

    ix = min(int(max(0.0, min(1.0, nx)) * 1023.0), 1023)
    iy = min(int(max(0.0, min(1.0, ny)) * 1023.0), 1023)
    iz = min(int(max(0.0, min(1.0, nz)) * 1023.0), 1023)

    ix = (ix * 0x00010001) & 0xFF0000FF
    ix = (ix * 0x00000101) & 0x0F00F00F
    ix = (ix * 0x00000011) & 0xC30C30C3
    ix = (ix * 0x00000005) & 0x49249249

    iy = (iy * 0x00010001) & 0xFF0000FF
    iy = (iy * 0x00000101) & 0x0F00F00F
    iy = (iy * 0x00000011) & 0xC30C30C3
    iy = (iy * 0x00000005) & 0x49249249

    iz = (iz * 0x00010001) & 0xFF0000FF
    iz = (iz * 0x00000101) & 0x0F00F00F
    iz = (iz * 0x00000011) & 0xC30C30C3
    iz = (iz * 0x00000005) & 0x49249249

    return ix | (iy << 1) | (iz << 2)


def _clz32(x: int) -> int:
    """Count leading zeros in a 32-bit integer."""
    if x == 0:
        return 32
    n = 0
    if x <= 0x0000FFFF:
        n += 16
        x <<= 16
    if x <= 0x00FFFFFF:
        n += 8
        x <<= 8
    if x <= 0x0FFFFFFF:
        n += 4
        x <<= 4
    if x <= 0x3FFFFFFF:
        n += 2
        x <<= 2
    if x <= 0x7FFFFFFF:
        n += 1
    return n


def _radix_sort(objects: List[dict]) -> None:
    """Radix sort objects by Morton code (in-place, 3 passes of 10 bits)."""
    RADIX = 1024
    PASSES = 3

    tmp = [None] * len(objects)

    for pass_num in range(PASSES):
        count = [0] * RADIX
        shift = pass_num * 10

        for obj in objects:
            bucket = (obj["morton_code"] >> shift) & (RADIX - 1)
            count[bucket] += 1

        total = 0
        for i in range(RADIX):
            c = count[i]
            count[i] = total
            total += c

        for obj in objects:
            bucket = (obj["morton_code"] >> shift) & (RADIX - 1)
            tmp[count[bucket]] = obj
            count[bucket] += 1

        objects[:] = tmp


@njit(cache=True)
def _check_collisions_jit(
    arena_left, arena_right, arena_object_id, arena_aabb, arena_root, n_boxes
):
    """JIT-compiled collision detection core (Numba-accelerated)."""
    all_collisions = []
    visited = np.zeros(n_boxes, dtype=np.bool_)
    total_checks = 0

    # Stack for traversal (pre-allocate large enough)
    stack = np.zeros((10000, 2), dtype=np.int32)
    stack_ptr = 0
    stack[stack_ptr, 0] = arena_root
    stack[stack_ptr, 1] = arena_root
    stack_ptr += 1

    while stack_ptr > 0:
        stack_ptr -= 1
        a_idx = stack[stack_ptr, 0]
        b_idx = stack[stack_ptr, 1]

        # AABB overlap test
        aabb1 = arena_aabb[a_idx]
        aabb2 = arena_aabb[b_idx]

        min1_x = aabb1[0] - aabb1[3]
        max1_x = aabb1[0] + aabb1[3]
        min1_y = aabb1[1] - aabb1[4]
        max1_y = aabb1[1] + aabb1[4]
        min1_z = aabb1[2] - aabb1[5]
        max1_z = aabb1[2] + aabb1[5]

        min2_x = aabb2[0] - aabb2[3]
        max2_x = aabb2[0] + aabb2[3]
        min2_y = aabb2[1] - aabb2[4]
        max2_y = aabb2[1] + aabb2[4]
        min2_z = aabb2[2] - aabb2[5]
        max2_z = aabb2[2] + aabb2[5]

        if not (
            min1_x <= max2_x
            and max1_x >= min2_x
            and min1_y <= max2_y
            and max1_y >= min2_y
            and min1_z <= max2_z
            and max1_z >= min2_z
        ):
            continue

        total_checks += 1

        a_obj_id = arena_object_id[a_idx]
        b_obj_id = arena_object_id[b_idx]
        a_leaf = a_obj_id >= 0
        b_leaf = b_obj_id >= 0

        # Both leaves
        if a_leaf and b_leaf:
            i = a_obj_id
            j = b_obj_id
            if 0 <= i < j < n_boxes:
                all_collisions.append((i, j))
                visited[i] = True
                visited[j] = True
            continue

        # Same node: expand unique child pairs
        if a_idx == b_idx:
            if not a_leaf:
                left_idx = arena_left[a_idx]
                right_idx = arena_right[a_idx]
                if left_idx >= 0:
                    stack[stack_ptr, 0] = left_idx
                    stack[stack_ptr, 1] = left_idx
                    stack_ptr += 1
                    if right_idx >= 0:
                        stack[stack_ptr, 0] = left_idx
                        stack[stack_ptr, 1] = right_idx
                        stack_ptr += 1
                        stack[stack_ptr, 0] = right_idx
                        stack[stack_ptr, 1] = right_idx
                        stack_ptr += 1
            continue

        # Both internal
        if not a_leaf and not b_leaf:
            a_left = arena_left[a_idx]
            a_right = arena_right[a_idx]
            b_left = arena_left[b_idx]
            b_right = arena_right[b_idx]
            if a_left >= 0 and b_left >= 0:
                stack[stack_ptr, 0] = a_left
                stack[stack_ptr, 1] = b_left
                stack_ptr += 1
            if a_left >= 0 and b_right >= 0:
                stack[stack_ptr, 0] = a_left
                stack[stack_ptr, 1] = b_right
                stack_ptr += 1
            if a_right >= 0 and b_left >= 0:
                stack[stack_ptr, 0] = a_right
                stack[stack_ptr, 1] = b_left
                stack_ptr += 1
            if a_right >= 0 and b_right >= 0:
                stack[stack_ptr, 0] = a_right
                stack[stack_ptr, 1] = b_right
                stack_ptr += 1
        # a is leaf, b is internal
        elif a_leaf and not b_leaf:
            b_left = arena_left[b_idx]
            b_right = arena_right[b_idx]
            if b_left >= 0:
                stack[stack_ptr, 0] = a_idx
                stack[stack_ptr, 1] = b_left
                stack_ptr += 1
            if b_right >= 0:
                stack[stack_ptr, 0] = a_idx
                stack[stack_ptr, 1] = b_right
                stack_ptr += 1
        # a is internal, b is leaf
        elif not a_leaf and b_leaf:
            a_left = arena_left[a_idx]
            a_right = arena_right[a_idx]
            if a_left >= 0:
                stack[stack_ptr, 0] = a_left
                stack[stack_ptr, 1] = b_idx
                stack_ptr += 1
            if a_right >= 0:
                stack[stack_ptr, 0] = a_right
                stack[stack_ptr, 1] = b_idx
                stack_ptr += 1

    return all_collisions, visited, total_checks


def _ray_aabb_intersect(
    origin: Point, direction: Vector, box: BvhAABB
) -> Tuple[bool, float, float]:
    """Check if a ray intersects an AABB."""
    min_x = box.cx - box.hx
    max_x = box.cx + box.hx
    min_y = box.cy - box.hy
    max_y = box.cy + box.hy
    min_z = box.cz - box.hz
    max_z = box.cz + box.hz

    def inv(v):
        return float("inf") if v == 0.0 else 1.0 / v

    invx = inv(direction.x)
    invy = inv(direction.y)
    invz = inv(direction.z)

    tx1 = (min_x - origin.x) * invx
    tx2 = (max_x - origin.x) * invx
    tmin = min(tx1, tx2)
    tmax = max(tx1, tx2)

    ty1 = (min_y - origin.y) * invy
    ty2 = (max_y - origin.y) * invy
    tmin = max(tmin, min(ty1, ty2))
    tmax = min(tmax, max(ty1, ty2))

    tz1 = (min_z - origin.z) * invz
    tz2 = (max_z - origin.z) * invz
    tmin = max(tmin, min(tz1, tz2))
    tmax = min(tmax, max(tz1, tz2))

    return tmax >= tmin, tmin, tmax


class BVH:
    """Boundary Volume Hierarchy for spatial acceleration."""

    def __init__(self, world_size: float = 1000.0):
        self.guid = str(uuid.uuid4())
        self.name = "my_bvh"
        self.root: Optional[BVHNode] = None
        self.world_size = world_size
        self.object_guids: List[str] = []
        # Flat arena for fast queries (NumPy arrays)
        self.arena_left: Optional[np.ndarray] = None  # int32
        self.arena_right: Optional[np.ndarray] = None  # int32
        self.arena_object_id: Optional[np.ndarray] = None  # int32
        self.arena_aabb: Optional[np.ndarray] = (
            None  # float64, shape (n, 6) for cx,cy,cz,hx,hy,hz
        )
        self.arena_root: int = -1

    @staticmethod
    def compute_world_size(bounding_boxes: List[BoundingBox]) -> float:
        """Compute world size from bounding boxes."""
        if not bounding_boxes:
            return 1000.0

        max_extent = 0.0
        for bbox in bounding_boxes:
            x_extent = max(
                abs(bbox.center.x + bbox.half_size.x),
                abs(bbox.center.x - bbox.half_size.x),
            )
            y_extent = max(
                abs(bbox.center.y + bbox.half_size.y),
                abs(bbox.center.y - bbox.half_size.y),
            )
            z_extent = max(
                abs(bbox.center.z + bbox.half_size.z),
                abs(bbox.center.z - bbox.half_size.z),
            )
            max_extent = max(max_extent, x_extent, y_extent, z_extent)

        return max(max_extent * 2.2, 10.0)

    @classmethod
    def from_boxes(cls, bounding_boxes: List[BoundingBox], world_size: float) -> "BVH":
        """Create a BVH from a list of bounding boxes."""
        bvh = cls(world_size)
        bvh.build(bounding_boxes)
        return bvh

    def build_with_guids(self, boxes_with_guids: List[Tuple[BoundingBox, str]]):
        """Build BVH from bounding boxes with GUIDs."""
        if not boxes_with_guids:
            self.root = None
            self.object_guids = []
            return

        bounding_boxes = [bbox for bbox, _ in boxes_with_guids]
        self.object_guids = [guid for _, guid in boxes_with_guids]
        self.world_size = self.compute_world_size(bounding_boxes)
        self.build(bounding_boxes)

    def build(self, bounding_boxes: List[BoundingBox]) -> None:
        """Build the BVH tree from bounding boxes using LBVH algorithm."""
        if not bounding_boxes:
            self.root = None
            self.arena_root = -1
            self.arena_left = None
            self.arena_right = None
            self.arena_object_id = None
            self.arena_aabb = None
            return

        N = len(bounding_boxes)

        # Create objects with Morton codes
        objects = []
        for i, bbox in enumerate(bounding_boxes):
            morton_code = calculate_morton_code(
                bbox.center.x, bbox.center.y, bbox.center.z, self.world_size
            )
            aabb = BvhAABB(
                bbox.center.x,
                bbox.center.y,
                bbox.center.z,
                bbox.half_size.x,
                bbox.half_size.y,
                bbox.half_size.z,
            )
            objects.append({"id": i, "morton_code": morton_code, "aabb": aabb})

        # Radix sort by Morton code
        _radix_sort(objects)

        # Single leaf case
        if N == 1:
            # Build arena only (no tree)
            obj = objects[0]
            self.arena_left = np.array([-1], dtype=np.int32)
            self.arena_right = np.array([-1], dtype=np.int32)
            self.arena_object_id = np.array([obj["id"]], dtype=np.int32)
            aabb = obj["aabb"]
            self.arena_aabb = np.array(
                [[aabb.cx, aabb.cy, aabb.cz, aabb.hx, aabb.hy, aabb.hz]],
                dtype=np.float64,
            )
            self.arena_root = 0
            self.root = None
            return

        # Extract sorted codes
        codes = [obj["morton_code"] for obj in objects]

        def common_prefix(i: int, j: int) -> int:
            """Calculate common prefix length between two codes."""
            if j < 0 or j >= N:
                return -1
            ci = codes[i]
            cj = codes[j]
            if ci != cj:
                return _clz32(ci ^ cj)
            return 32 + _clz32(i ^ j)

        def determine_range(i: int) -> Tuple[int, int]:
            """Determine the range of keys covered by internal node i."""
            d = 1 if common_prefix(i, i + 1) - common_prefix(i, i - 1) > 0 else -1
            delta_min = common_prefix(i, i - d)

            length = 1
            while common_prefix(i, i + length * d) > delta_min:
                length <<= 1

            bound = 0
            t = length >> 1
            while t > 0:
                if common_prefix(i, i + (bound + t) * d) > delta_min:
                    bound += t
                t >>= 1

            j = i + bound * d
            return (min(i, j), max(i, j))

        def find_split(first: int, last: int) -> int:
            """Find split position for range [first, last]."""
            common = common_prefix(first, last)
            split = first
            step = last - first

            while step > 1:
                step = (step + 1) >> 1
                new_split = split + step
                if new_split < last:
                    split_prefix = common_prefix(first, new_split)
                    if split_prefix > common:
                        split = new_split

            return split

        # Allocate leaves
        leaves = []
        for i in range(N):
            leaf = BVHNode()
            leaf.object_id = objects[i]["id"]
            leaf.aabb = objects[i]["aabb"]
            leaves.append(leaf)

        # Allocate internal nodes
        internals = []
        for i in range(N - 1):
            node = BVHNode()
            internals.append(node)

        # Build topology
        has_parent = [False] * (N - 1)
        for i in range(N - 1):
            first, last = determine_range(i)
            split = find_split(first, last)

            if split == first:
                internals[i].left = leaves[split]
            else:
                internals[i].left = internals[split]
                has_parent[split] = True

            if split + 1 == last:
                internals[i].right = leaves[split + 1]
            else:
                internals[i].right = internals[split + 1]
                has_parent[split + 1] = True

        # Find root
        root_idx = 0
        for i in range(N - 1):
            if not has_parent[i]:
                root_idx = i
                break
        self.root = internals[root_idx]

        # Post-order compute internal AABBs
        def compute_aabb(node: BVHNode) -> None:
            if not node or node.is_leaf():
                return

            compute_aabb(node.left)
            compute_aabb(node.right)

            a = node.left.aabb
            b = node.right.aabb

            min_x = min(a.cx - a.hx, b.cx - b.hx)
            min_y = min(a.cy - a.hy, b.cy - b.hy)
            min_z = min(a.cz - a.hz, b.cz - b.hz)
            max_x = max(a.cx + a.hx, b.cx + b.hx)
            max_y = max(a.cy + a.hy, b.cy + b.hy)
            max_z = max(a.cz + a.hz, b.cz + b.hz)

            node.aabb = BvhAABB(
                (min_x + max_x) * 0.5,
                (min_y + max_y) * 0.5,
                (min_z + max_z) * 0.5,
                (max_x - min_x) * 0.5,
                (max_y - min_y) * 0.5,
                (max_z - min_z) * 0.5,
            )

        compute_aabb(self.root)

        # Build flat arena for fast queries (NumPy arrays)
        total_nodes = N + (N - 1)  # leaves + internals
        self.arena_left = np.full(total_nodes, -1, dtype=np.int32)
        self.arena_right = np.full(total_nodes, -1, dtype=np.int32)
        self.arena_object_id = np.full(total_nodes, -1, dtype=np.int32)
        self.arena_aabb = np.zeros((total_nodes, 6), dtype=np.float64)

        arena_idx = [0]  # Use list to allow mutation in nested function

        def build_arena(node: Optional[BVHNode]) -> int:
            """Build flat arena from tree, return index."""
            if not node:
                return -1

            idx = arena_idx[0]
            arena_idx[0] += 1

            # Store AABB
            if node.aabb:
                self.arena_aabb[idx] = [
                    node.aabb.cx,
                    node.aabb.cy,
                    node.aabb.cz,
                    node.aabb.hx,
                    node.aabb.hy,
                    node.aabb.hz,
                ]

            # Leaf node
            if node.is_leaf():
                self.arena_object_id[idx] = node.object_id
                return idx

            # Internal node - build children first
            self.arena_object_id[idx] = -1
            left_idx = build_arena(node.left) if node.left else -1
            right_idx = build_arena(node.right) if node.right else -1
            self.arena_left[idx] = left_idx
            self.arena_right[idx] = right_idx

            return idx

        self.arena_root = build_arena(self.root)
        # Don't build Box tree - arena is sufficient
        self.root = None

    def merge_aabb(self, aabb1: BoundingBox, aabb2: BoundingBox) -> BoundingBox:
        """Merge two AABBs into a single encompassing AABB."""
        min_x = min(
            aabb1.center.x - aabb1.half_size.x, aabb2.center.x - aabb2.half_size.x
        )
        min_y = min(
            aabb1.center.y - aabb1.half_size.y, aabb2.center.y - aabb2.half_size.y
        )
        min_z = min(
            aabb1.center.z - aabb1.half_size.z, aabb2.center.z - aabb2.half_size.z
        )

        max_x = max(
            aabb1.center.x + aabb1.half_size.x, aabb2.center.x + aabb2.half_size.x
        )
        max_y = max(
            aabb1.center.y + aabb1.half_size.y, aabb2.center.y + aabb2.half_size.y
        )
        max_z = max(
            aabb1.center.z + aabb1.half_size.z, aabb2.center.z + aabb2.half_size.z
        )

        center = Point((min_x + max_x) / 2, (min_y + max_y) / 2, (min_z + max_z) / 2)
        half_size = Vector(
            (max_x - min_x) / 2, (max_y - min_y) / 2, (max_z - min_z) / 2
        )

        return BoundingBox(
            center, Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), half_size
        )

    def aabb_intersect(self, aabb1: BoundingBox, aabb2: BoundingBox) -> bool:
        """Check if two AABBs intersect."""
        min1_x = aabb1.center.x - aabb1.half_size.x
        max1_x = aabb1.center.x + aabb1.half_size.x
        min1_y = aabb1.center.y - aabb1.half_size.y
        max1_y = aabb1.center.y + aabb1.half_size.y
        min1_z = aabb1.center.z - aabb1.half_size.z
        max1_z = aabb1.center.z + aabb1.half_size.z

        min2_x = aabb2.center.x - aabb2.half_size.x
        max2_x = aabb2.center.x + aabb2.half_size.x
        min2_y = aabb2.center.y - aabb2.half_size.y
        max2_y = aabb2.center.y + aabb2.half_size.y
        min2_z = aabb2.center.z - aabb2.half_size.z
        max2_z = aabb2.center.z + aabb2.half_size.z

        return (
            min1_x <= max2_x
            and max1_x >= min2_x
            and min1_y <= max2_y
            and max1_y >= min2_y
            and min1_z <= max2_z
            and max1_z >= min2_z
        )

    def _aabb_intersect_internal(self, aabb1: BvhAABB, aabb2: BvhAABB) -> bool:
        """Check if two internal AABBs intersect."""
        min1_x = aabb1.cx - aabb1.hx
        max1_x = aabb1.cx + aabb1.hx
        min1_y = aabb1.cy - aabb1.hy
        max1_y = aabb1.cy + aabb1.hy
        min1_z = aabb1.cz - aabb1.hz
        max1_z = aabb1.cz + aabb1.hz

        min2_x = aabb2.cx - aabb2.hx
        max2_x = aabb2.cx + aabb2.hx
        min2_y = aabb2.cy - aabb2.hy
        max2_y = aabb2.cy + aabb2.hy
        min2_z = aabb2.cz - aabb2.hz
        max2_z = aabb2.cz + aabb2.hz

        return (
            min1_x <= max2_x
            and max1_x >= min2_x
            and min1_y <= max2_y
            and max1_y >= min2_y
            and min1_z <= max2_z
            and max1_z >= min2_z
        )

    def _aabb_intersect_fast(self, idx1: int, idx2: int) -> bool:
        """Fast AABB intersection check using NumPy arena."""
        aabb1 = self.arena_aabb[idx1]
        aabb2 = self.arena_aabb[idx2]

        min1_x = aabb1[0] - aabb1[3]
        max1_x = aabb1[0] + aabb1[3]
        min1_y = aabb1[1] - aabb1[4]
        max1_y = aabb1[1] + aabb1[4]
        min1_z = aabb1[2] - aabb1[5]
        max1_z = aabb1[2] + aabb1[5]

        min2_x = aabb2[0] - aabb2[3]
        max2_x = aabb2[0] + aabb2[3]
        min2_y = aabb2[1] - aabb2[4]
        max2_y = aabb2[1] + aabb2[4]
        min2_z = aabb2[2] - aabb2[5]
        max2_z = aabb2[2] + aabb2[5]

        return (
            min1_x <= max2_x
            and max1_x >= min2_x
            and min1_y <= max2_y
            and max1_y >= min2_y
            and min1_z <= max2_z
            and max1_z >= min2_z
        )

    def check_all_collisions(
        self, bounding_boxes: List[BoundingBox]
    ) -> Tuple[List[Tuple[int, int]], List[int], int]:
        """Check for all pairwise collisions in the scene using fast NumPy arena."""
        if self.arena_root < 0 or self.arena_aabb is None:
            return [], [], 0

        # Use Numba-JIT version if available for C++-level speed
        if HAS_NUMBA:
            all_collisions, visited, total_checks = _check_collisions_jit(
                self.arena_left,
                self.arena_right,
                self.arena_object_id,
                self.arena_aabb,
                self.arena_root,
                len(bounding_boxes),
            )
            colliding_indices = [i for i in range(len(visited)) if visited[i]]
            return all_collisions, colliding_indices, total_checks

        # Fallback: Pure Python version (slower)
        all_collisions = []
        visited = [False] * len(bounding_boxes)
        total_checks = 0
        stack = [(self.arena_root, self.arena_root)]

        while stack:
            a_idx, b_idx = stack.pop()

            # AABB overlap test
            aabb1 = self.arena_aabb[a_idx]
            aabb2 = self.arena_aabb[b_idx]
            min1_x, max1_x = aabb1[0] - aabb1[3], aabb1[0] + aabb1[3]
            min1_y, max1_y = aabb1[1] - aabb1[4], aabb1[1] + aabb1[4]
            min1_z, max1_z = aabb1[2] - aabb1[5], aabb1[2] + aabb1[5]
            min2_x, max2_x = aabb2[0] - aabb2[3], aabb2[0] + aabb2[3]
            min2_y, max2_y = aabb2[1] - aabb2[4], aabb2[1] + aabb2[4]
            min2_z, max2_z = aabb2[2] - aabb2[5], aabb2[2] + aabb2[5]

            if not (
                min1_x <= max2_x
                and max1_x >= min2_x
                and min1_y <= max2_y
                and max1_y >= min2_y
                and min1_z <= max2_z
                and max1_z >= min2_z
            ):
                continue

            total_checks += 1
            a_obj_id, b_obj_id = (
                self.arena_object_id[a_idx],
                self.arena_object_id[b_idx],
            )
            a_leaf, b_leaf = a_obj_id >= 0, b_obj_id >= 0

            if a_leaf and b_leaf:
                i, j = a_obj_id, b_obj_id
                if 0 <= i < j < len(bounding_boxes):
                    all_collisions.append((i, j))
                    visited[i], visited[j] = True, True
                continue

            if a_idx == b_idx:
                if not a_leaf:
                    left_idx, right_idx = (
                        self.arena_left[a_idx],
                        self.arena_right[a_idx],
                    )
                    if left_idx >= 0:
                        stack.append((left_idx, left_idx))
                        if right_idx >= 0:
                            stack.extend(
                                [(left_idx, right_idx), (right_idx, right_idx)]
                            )
                continue

            if not a_leaf and not b_leaf:
                a_left, a_right = self.arena_left[a_idx], self.arena_right[a_idx]
                b_left, b_right = self.arena_left[b_idx], self.arena_right[b_idx]
                if a_left >= 0 and b_left >= 0:
                    stack.append((a_left, b_left))
                if a_left >= 0 and b_right >= 0:
                    stack.append((a_left, b_right))
                if a_right >= 0 and b_left >= 0:
                    stack.append((a_right, b_left))
                if a_right >= 0 and b_right >= 0:
                    stack.append((a_right, b_right))
            elif a_leaf and not b_leaf:
                b_left, b_right = self.arena_left[b_idx], self.arena_right[b_idx]
                if b_left >= 0:
                    stack.append((a_idx, b_left))
                if b_right >= 0:
                    stack.append((a_idx, b_right))
            elif not a_leaf and b_leaf:
                a_left, a_right = self.arena_left[a_idx], self.arena_right[a_idx]
                if a_left >= 0:
                    stack.append((a_left, b_idx))
                if a_right >= 0:
                    stack.append((a_right, b_idx))

        colliding_indices = [i for i, v in enumerate(visited) if v]
        return all_collisions, colliding_indices, total_checks

    def check_all_collisions_guids(
        self, bounding_boxes: List[BoundingBox]
    ) -> List[Tuple[str, str]]:
        """Check for all collisions and return GUID pairs."""
        collisions, _, _ = self.check_all_collisions(bounding_boxes)
        guid_collisions = []
        for i, j in collisions:
            if i < len(self.object_guids) and j < len(self.object_guids):
                guid_collisions.append((self.object_guids[i], self.object_guids[j]))
        return guid_collisions

    def ray_cast(
        self,
        origin: Point,
        direction: Vector,
        candidate_leaf_ids: List[int],
        find_all: bool = False,
    ) -> bool:
        """Cast a ray through the BVH and return candidate leaf IDs ordered by distance."""
        candidate_leaf_ids.clear()

        if self.arena_root < 0 or self.arena_aabb is None:
            return False

        heap = []

        # Test root node
        root_aabb_data = self.arena_aabb[self.arena_root]
        root_aabb = BvhAABB(*root_aabb_data)
        intersects, rtmin, rtmax = _ray_aabb_intersect(origin, direction, root_aabb)
        if not intersects or rtmax < 0.0:
            return False

        heapq.heappush(heap, (rtmin, self.arena_root))

        any_found = False
        while heap:
            tmin, idx = heapq.heappop(heap)

            if idx < 0:
                continue

            obj_id = self.arena_object_id[idx]
            is_leaf = obj_id >= 0

            if is_leaf:
                candidate_leaf_ids.append(obj_id)
                any_found = True
                if not find_all and len(candidate_leaf_ids) >= 1:
                    pass  # Continue for ordering
                continue

            # Internal node - test children
            left_idx = self.arena_left[idx]
            if left_idx >= 0:
                left_aabb_data = self.arena_aabb[left_idx]
                left_aabb = BvhAABB(*left_aabb_data)
                intersects, cmin, cmax = _ray_aabb_intersect(
                    origin, direction, left_aabb
                )
                if intersects and cmax >= 0.0:
                    heapq.heappush(heap, (cmin, left_idx))

            right_idx = self.arena_right[idx]
            if right_idx >= 0:
                right_aabb_data = self.arena_aabb[right_idx]
                right_aabb = BvhAABB(*right_aabb_data)
                intersects, cmin, cmax = _ray_aabb_intersect(
                    origin, direction, right_aabb
                )
                if intersects and cmax >= 0.0:
                    heapq.heappush(heap, (cmin, right_idx))

        return any_found
