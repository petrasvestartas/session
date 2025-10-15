"""Boundary Volume Hierarchy (BVH) for spatial acceleration.

This module implements a BVH tree using Morton codes for efficient spatial
partitioning and collision detection.
"""

import uuid
from typing import List, Tuple, Optional
from .point import Point
from .boundingbox import BoundingBox


class BVHNode:
    """A node in the BVH tree.

    Attributes
    ----------
    guid : str
        Unique identifier for the node.
    left : BVHNode or None
        Left child node.
    right : BVHNode or None
        Right child node.
    object_id : int
        ID of the object (only valid for leaf nodes, -1 for internal nodes).
    aabb : BoundingBox
        Axis-aligned bounding box encompassing this node's volume.
    """

    def __init__(self):
        self.guid = str(uuid.uuid4())
        self.left: Optional[BVHNode] = None
        self.right: Optional[BVHNode] = None
        self.object_id: int = -1
        self.aabb: Optional[BoundingBox] = None

    def is_leaf(self) -> bool:
        """Check if this node is a leaf node.

        Returns
        -------
        bool
            True if this is a leaf node (has an object_id), False otherwise.
        """
        return self.object_id != -1


def expand_bits(v: int) -> int:
    """Expand bits for Morton code calculation.

    Inserts two zero bits after each of the 10 low bits of v.

    Parameters
    ----------
    v : int
        Input value (0-1023).

    Returns
    -------
    int
        Value with expanded bits.
    """
    v = (v * 0x00010001) & 0xFF0000FF
    v = (v * 0x00000101) & 0x0F00F00F
    v = (v * 0x00000011) & 0xC30C30C3
    v = (v * 0x00000005) & 0x49249249
    return v


def calculate_morton_code(
    x: float, y: float, z: float, world_size: float = 100.0
) -> int:
    """Calculate 3D Morton code (Z-order curve) for spatial hashing.

    Morton codes provide a mapping from 3D space to 1D that preserves spatial locality.
    Objects close in 3D space will have similar Morton codes.

    Parameters
    ----------
    x : float
        X coordinate.
    y : float
        Y coordinate.
    z : float
        Z coordinate.
    world_size : float, optional
        Size of the world bounds. Defaults to 100.0.

    Returns
    -------
    int
        30-bit Morton code (10 bits per dimension).
    """
    # Normalize coordinates to [0,1] range
    nx = (x + world_size / 2) / world_size
    ny = (y + world_size / 2) / world_size
    nz = (z + world_size / 2) / world_size

    # Clamp to [0,1]
    nx = max(0.0, min(1.0, nx))
    ny = max(0.0, min(1.0, ny))
    nz = max(0.0, min(1.0, nz))

    # Scale to [0, 1023] for 10-bit encoding
    ix = min(int(nx * 1023), 1023)
    iy = min(int(ny * 1023), 1023)
    iz = min(int(nz * 1023), 1023)

    # Expand bits and interleave
    xx = expand_bits(ix)
    yy = expand_bits(iy)
    zz = expand_bits(iz)

    return xx | (yy << 1) | (zz << 2)


class BVH:
    """Boundary Volume Hierarchy for spatial acceleration.

    A BVH organizes objects in a tree structure based on their spatial positions,
    using Morton codes for efficient construction. This enables fast collision
    detection and spatial queries.

    Attributes
    ----------
    guid : str
        Unique identifier for the BVH.
    name : str
        Name of the BVH.
    root : BVHNode or None
        Root node of the BVH tree.
    world_size : float
        Size of the world bounds for Morton code calculation.
    """

    def __init__(self, world_size: float):
        """Initialize an empty BVH.

        Parameters
        ----------
        world_size : float
            Size of the world bounds.
        """
        self.guid = str(uuid.uuid4())
        self.name = "my_bvh"
        self.root: Optional[BVHNode] = None
        self.world_size = world_size

    @classmethod
    def from_boxes(cls, bounding_boxes: List[BoundingBox], world_size: float) -> "BVH":
        """Create a BVH from a list of bounding boxes.

        Parameters
        ----------
        bounding_boxes : list of BoundingBox
            List of bounding boxes to build the BVH with.
        world_size : float
            Size of the world bounds.

        Returns
        -------
        BVH
            A new BVH with the tree already built.
        """
        bvh = cls(world_size)
        bvh.build(bounding_boxes)
        return bvh

    def build(self, bounding_boxes: List[BoundingBox]) -> None:
        """Build the BVH tree from a list of bounding boxes.

        Parameters
        ----------
        bounding_boxes : list of BoundingBox
            List of bounding boxes to organize in the BVH.
        """
        if not bounding_boxes:
            self.root = None
            return

        # Create list of objects with their Morton codes
        objects = []
        for i, bbox in enumerate(bounding_boxes):
            center = bbox.center
            morton_code = calculate_morton_code(
                center.x, center.y, center.z, self.world_size
            )
            objects.append({"id": i, "morton_code": morton_code, "bbox": bbox})

        # Sort by Morton code for spatial locality
        objects.sort(key=lambda obj: obj["morton_code"])

        # Build tree recursively
        self.root = self._create_subtree(objects, 0, len(objects) - 1)

    def _create_subtree(self, objects: List[dict], begin: int, end: int) -> BVHNode:
        """Recursively create a subtree of the BVH.

        Parameters
        ----------
        objects : list of dict
            Sorted list of objects with their Morton codes and bounding boxes.
        begin : int
            Start index in the objects list.
        end : int
            End index in the objects list.

        Returns
        -------
        BVHNode
            Root node of the created subtree.
        """
        if begin == end:
            # Create leaf node
            node = BVHNode()
            node.object_id = objects[begin]["id"]
            node.aabb = objects[begin]["bbox"]
            return node
        else:
            # Create internal node
            mid = (begin + end) // 2
            node = BVHNode()

            # Recursively create children
            node.left = self._create_subtree(objects, begin, mid)
            node.right = self._create_subtree(objects, mid + 1, end)

            # Merge children's AABBs
            node.aabb = self._merge_aabb(node.left.aabb, node.right.aabb)

            return node

    def _merge_aabb(self, aabb1: BoundingBox, aabb2: BoundingBox) -> BoundingBox:
        """Merge two AABBs into a single encompassing AABB.

        Parameters
        ----------
        aabb1 : BoundingBox
            First bounding box.
        aabb2 : BoundingBox
            Second bounding box.

        Returns
        -------
        BoundingBox
            Merged bounding box encompassing both inputs.
        """
        from .vector import Vector

        # Calculate min and max corners
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

        # Calculate new center and half_size
        center = Point((min_x + max_x) / 2, (min_y + max_y) / 2, (min_z + max_z) / 2)

        half_size = Vector(
            (max_x - min_x) / 2, (max_y - min_y) / 2, (max_z - min_z) / 2
        )

        return BoundingBox(
            center, Vector(1, 0, 0), Vector(0, 1, 0), Vector(0, 0, 1), half_size
        )

    def find_collisions(
        self, object_id: int, query_bbox: BoundingBox, bounding_boxes: List[BoundingBox]
    ) -> Tuple[List[int], int]:
        """Find all objects that collide with a query bounding box.

        Parameters
        ----------
        object_id : int
            ID of the query object (to avoid self-collision).
        query_bbox : BoundingBox
            Query bounding box.
        bounding_boxes : list of BoundingBox
            List of all bounding boxes in the scene.

        Returns
        -------
        tuple
            (collisions, check_count) where collisions is a list of object IDs
            that collide with the query, and check_count is the number of
            AABB intersection tests performed.
        """
        if self.root is None:
            return [], 0

        collisions = []
        check_count = [0]  # Use list to allow modification in nested function

        self._find_collisions_recursive(
            object_id, query_bbox, self.root, bounding_boxes, collisions, check_count
        )

        return collisions, check_count[0]

    def _find_collisions_recursive(
        self,
        object_id: int,
        query_bbox: BoundingBox,
        node: BVHNode,
        bounding_boxes: List[BoundingBox],
        collisions: List[int],
        check_count: List[int],
    ) -> None:
        """Recursively traverse the BVH to find collisions.

        Parameters
        ----------
        object_id : int
            ID of the query object.
        query_bbox : BoundingBox
            Query bounding box.
        node : BVHNode
            Current node being checked.
        bounding_boxes : list of BoundingBox
            List of all bounding boxes.
        collisions : list of int
            List to accumulate collision results.
        check_count : list of int
            Counter for number of checks performed (mutable).
        """
        check_count[0] += 1

        # Early exit if query doesn't intersect this node's AABB
        if not self._aabb_intersect(query_bbox, node.aabb):
            return

        # If leaf node, check for collision
        if node.is_leaf():
            # Don't check collision with self
            if node.object_id != object_id:
                if self._aabb_intersect(query_bbox, bounding_boxes[node.object_id]):
                    collisions.append(node.object_id)
            return

        # Recurse through children
        if node.left:
            self._find_collisions_recursive(
                object_id,
                query_bbox,
                node.left,
                bounding_boxes,
                collisions,
                check_count,
            )
        if node.right:
            self._find_collisions_recursive(
                object_id,
                query_bbox,
                node.right,
                bounding_boxes,
                collisions,
                check_count,
            )

    def _aabb_intersect(self, aabb1: BoundingBox, aabb2: BoundingBox) -> bool:
        """Check if two AABBs intersect.

        Parameters
        ----------
        aabb1 : BoundingBox
            First bounding box.
        aabb2 : BoundingBox
            Second bounding box.

        Returns
        -------
        bool
            True if the AABBs intersect, False otherwise.
        """
        # Calculate min/max for both boxes
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

        # Check for overlap on all three axes
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
        """Check for all pairwise collisions in the scene.

        Parameters
        ----------
        bounding_boxes : list of BoundingBox
            List of all bounding boxes to check.

        Returns
        -------
        tuple
            (collisions, indices_of_all_objects_colliding, check_count) where:
            - collisions is a list of (id1, id2) tuples representing colliding pairs
            - indices_of_all_objects_colliding is a list of all object indices that are involved in any collision
            - check_count is the total number of AABB intersection tests performed
        """
        all_collisions = []
        colliding_objects = set()
        total_checks = 0

        for i, bbox in enumerate(bounding_boxes):
            collisions, checks = self.find_collisions(i, bbox, bounding_boxes)
            total_checks += checks

            # Add unique collision pairs (avoid duplicates)
            for j in collisions:
                if i < j:  # Only add each pair once
                    all_collisions.append((i, j))
                    colliding_objects.add(i)
                    colliding_objects.add(j)

        return all_collisions, sorted(list(colliding_objects)), total_checks
