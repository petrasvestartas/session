from typing import Optional, List
from .line import Line
from .point import Point
from .boundingbox import BoundingBox


def ray_box(
    line: Line, box: BoundingBox, t0: float, t1: float
) -> Optional[List[Point]]:
    """
    Find intersection points between a ray (represented as a line) and an axis-aligned bounding box.

    Args:
        line: The ray represented as a line (start point + direction)
        box: The bounding box to intersect with
        t0: Minimum parameter value (e.g., 0.0 for ray origin)
        t1: Maximum parameter value (e.g., 1000.0 for max distance)

    Returns:
        List of 2 points [entry, exit] if intersection exists, None otherwise
        Points are sorted from line start (entry first, exit second)
    """
    origin = line.start()
    direction = line.to_vector()

    box_min = box.min_point()
    box_max = box.max_point()

    # Calculate inverse direction (avoid division by zero)
    inv_dir_x = 1.0 / direction.x if direction.x != 0.0 else float("inf")
    inv_dir_y = 1.0 / direction.y if direction.y != 0.0 else float("inf")
    inv_dir_z = 1.0 / direction.z if direction.z != 0.0 else float("inf")

    # Calculate intersections with X slabs
    tx1 = (box_min.x - origin.x) * inv_dir_x
    tx2 = (box_max.x - origin.x) * inv_dir_x

    tmin = min(tx1, tx2)
    tmax = max(tx1, tx2)

    # Calculate intersections with Y slabs
    ty1 = (box_min.y - origin.y) * inv_dir_y
    ty2 = (box_max.y - origin.y) * inv_dir_y

    tmin = max(tmin, min(ty1, ty2))
    tmax = min(tmax, max(ty1, ty2))

    # Calculate intersections with Z slabs
    tz1 = (box_min.z - origin.z) * inv_dir_z
    tz2 = (box_max.z - origin.z) * inv_dir_z

    tmin = max(tmin, min(tz1, tz2))
    tmax = min(tmax, max(tz1, tz2))

    # Clip to valid range
    tmin = max(tmin, t0)
    tmax = min(tmax, t1)

    # Check if intersection exists
    if tmax < tmin:
        return None

    # Calculate actual intersection points
    entry = Point(
        origin.x + direction.x * tmin,
        origin.y + direction.y * tmin,
        origin.z + direction.z * tmin,
    )

    exit_point = Point(
        origin.x + direction.x * tmax,
        origin.y + direction.y * tmax,
        origin.z + direction.z * tmax,
    )

    return [entry, exit_point]
