"""
Intersection functions for geometric primitives.

This module provides intersection calculations between various geometric objects
including lines, planes, rays, boxes, spheres, triangles, and meshes.
"""

from typing import Optional, Tuple, List
from .line import Line
from .point import Point
from .boundingbox import BoundingBox
from .mesh import Mesh
from .bvh import BVH


def line_line_parameters(
    line0: Line,
    line1: Line,
    tolerance: float,
    intersect_segments: bool = True,
    near_parallel_as_closest: bool = False,
) -> Optional[Tuple[float, float]]:
    """
    Find parametric values where two lines are closest.

    Args:
        line0: First line
        line1: Second line
        tolerance: Maximum distance to consider intersection
        intersect_segments: If True, clamp parameters to [0,1]; if False, treat as infinite lines
        near_parallel_as_closest: If True, return closest point for near-parallel lines

    Returns:
        Tuple of (t0, t1) parameters if intersection found, None otherwise
        t0 is parameter on line0 (0=start, 1=end)
        t1 is parameter on line1 (0=start, 1=end)
    """
    p0_start = line0.start()
    p0_end = line0.end()
    p1_start = line1.start()
    p1_end = line1.end()

    if p0_start == p1_start:
        return (0.0, 0.0)
    if p0_start == p1_end:
        return (0.0, 1.0)
    if p0_end == p1_start:
        return (1.0, 0.0)
    if p0_end == p1_end:
        return (1.0, 1.0)

    A = line0.to_vector()
    B = line1.to_vector()
    C = p1_start - p0_start

    AA = A.dot(A)
    BB = B.dot(B)
    AB = A.dot(B)
    AC = A.dot(C)
    BC = B.dot(C)

    det = AA * BB - AB * AB

    zero_tol = max(AA, BB) * 1e-15
    if abs(det) < zero_tol:
        if not near_parallel_as_closest:
            return None
        t0 = (AC / AA) if AA > 0.0 else 0.0
        t1 = ((BC + t0 * AB) / BB) if BB > 0.0 else 0.0

        if intersect_segments:
            t0 = max(0.0, min(1.0, t0))
            t1 = max(0.0, min(1.0, t1))

        if tolerance > 0.0:
            pt0 = line0.point_at(t0)
            pt1 = line1.point_at(t1)
            if pt0.distance(pt1) > tolerance:
                return None
        return (t0, t1)

    inv_det = 1.0 / det
    t0 = (BB * AC - AB * BC) * inv_det
    t1 = (AB * AC - AA * BC) * inv_det

    if intersect_segments:
        t0 = max(0.0, min(1.0, t0))
        t1 = max(0.0, min(1.0, t1))

    if tolerance > 0.0:
        pt0 = line0.point_at(t0)
        pt1 = line1.point_at(t1)
        if pt0.distance(pt1) > tolerance:
            return None

    return (t0, t1)


def line_line(line0: Line, line1: Line, tolerance: float) -> Optional[Point]:
    """
    Find intersection point between two 3D lines.

    Args:
        line0: First line
        line1: Second line
        tolerance: Maximum distance between lines to consider them intersecting

    Returns:
        Intersection point (midpoint of closest approach for skew lines) if within tolerance,
        None otherwise
    """
    result = line_line_parameters(line0, line1, tolerance, True, False)

    if result is None:
        return None

    t0, t1 = result
    p0 = line0.point_at(t0)
    p1 = line1.point_at(t1)

    return Point((p0.x + p1.x) * 0.5, (p0.y + p1.y) * 0.5, (p0.z + p1.z) * 0.5)


def plane_plane(plane0, plane1) -> Optional[Line]:
    from .plane import Plane

    d = plane1.z_axis.cross(plane0.z_axis)

    p = Point(
        (plane0.origin.x + plane1.origin.x) * 0.5,
        (plane0.origin.y + plane1.origin.y) * 0.5,
        (plane0.origin.z + plane1.origin.z) * 0.5,
    )

    plane2 = Plane.from_point_normal(p, d)

    output_p = plane_plane_plane(plane0, plane1, plane2)
    if output_p is None:
        return None

    return Line(
        output_p.x,
        output_p.y,
        output_p.z,
        output_p.x + d.x,
        output_p.y + d.y,
        output_p.z + d.z,
    )


def plane_value_at(plane, point: Point) -> float:
    """Calculate the plane equation value at a point"""
    return plane.a * point.x + plane.b * point.y + plane.c * point.z + plane.d


def line_plane(line: Line, plane, is_finite: bool = True) -> Optional[Point]:
    """
    Find intersection point between a line and a plane.

    Args:
        line: Line to intersect
        plane: Plane to intersect
        is_finite: If True, treat line as finite segment; if False, treat as infinite

    Returns:
        Intersection point if exists, None if line is parallel to plane
    """
    pt0 = line.start()
    pt1 = line.end()

    a = plane_value_at(plane, pt0)
    b = plane_value_at(plane, pt1)
    d = a - b

    if d == 0.0:
        if abs(a) < abs(b):
            t = 0.0
        elif abs(b) < abs(a):
            t = 1.0
        else:
            t = 0.5
        rc = False
    else:
        d_inv = 1.0 / d
        fd = abs(d_inv)
        if fd > 1.0 and (abs(a) >= 1e38 / fd or abs(b) >= 1e38 / fd):
            t = 0.5
            rc = False
        else:
            t = a / (a - b)
            rc = True

    s = 1.0 - t

    output = Point(
        pt0.x if line.x0 == line.x1 else s * line.x0 + t * line.x1,
        pt0.y if line.y0 == line.y1 else s * line.y0 + t * line.y1,
        pt0.z if line.z0 == line.z1 else s * line.z0 + t * line.z1,
    )

    if is_finite and (t < 0.0 or t > 1.0):
        return None

    return output if rc else None


def plane_plane_plane(plane0, plane1, plane2) -> Optional[Point]:
    """
    Find intersection point of three planes.

    Args:
        plane0: First plane
        plane1: Second plane
        plane2: Third plane

    Returns:
        Intersection point if planes intersect at a point, None if parallel or degenerate
    """
    n0 = plane0.z_axis
    n1 = plane1.z_axis
    n2 = plane2.z_axis

    det = n0.dot(n1.cross(n2))

    if abs(det) < 1e-10:
        return None

    d0 = plane0.d
    d1 = plane1.d
    d2 = plane2.d

    p = (n1.cross(n2) * (-d0) + n2.cross(n0) * (-d1) + n0.cross(n1) * (-d2)) * (
        1.0 / det
    )

    return Point(p.x, p.y, p.z)


def ray_box(
    line: Line, box: BoundingBox, t0: float, t1: float
) -> Optional[List[Point]]:
    """
    Find intersection points between a line and an axis-aligned bounding box.

    Args:
        line: Line to intersect
        box: Axis-aligned bounding box
        t0: Minimum parameter value to consider (e.g., 0.0 for ray origin)
        t1: Maximum parameter value to consider (e.g., 1000.0 for max distance)

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


def ray_sphere(line: Line, center: Point, radius: float) -> Optional[List[Point]]:
    """
    Find intersection points between a line and a sphere.

    Args:
        line: Line to intersect
        center: Sphere center point
        radius: Sphere radius

    Returns:
        List of 1 point (tangent) or 2 points (entry/exit) if intersection exists,
        None otherwise. Points are sorted from line start.
    """
    origin = line.start()
    direction = line.to_vector()

    # Vector from origin to center
    o_x = origin.x - center.x
    o_y = origin.y - center.y
    o_z = origin.z - center.z

    # Quadratic equation coefficients
    a = (
        direction.x * direction.x
        + direction.y * direction.y
        + direction.z * direction.z
    )
    b = 2.0 * (direction.x * o_x + direction.y * o_y + direction.z * o_z)
    c = o_x * o_x + o_y * o_y + o_z * o_z - radius * radius

    # Discriminant
    disc = b * b - 4.0 * a * c

    if disc < 0.0:
        return None

    # Calculate intersection parameters
    dist_sqrt = disc**0.5
    if b < 0.0:
        q = (-b - dist_sqrt) / 2.0
    else:
        q = (-b + dist_sqrt) / 2.0

    t0 = q / a
    t1 = c / q

    # Sort parameters
    if t0 > t1:
        t0, t1 = t1, t0

    # Calculate intersection points
    points = []

    # First intersection
    p0 = Point(
        origin.x + direction.x * t0,
        origin.y + direction.y * t0,
        origin.z + direction.z * t0,
    )
    points.append(p0)

    # Second intersection (if different from first)
    if abs(t1 - t0) > 1e-10:
        p1 = Point(
            origin.x + direction.x * t1,
            origin.y + direction.y * t1,
            origin.z + direction.z * t1,
        )
        points.append(p1)

    return points


def ray_triangle(
    line: Line, v0: Point, v1: Point, v2: Point, epsilon: float
) -> Optional[Point]:
    """
    Find intersection point between a line and a triangle.

    Args:
        line: Line to intersect (start point used as origin, direction computed internally)
        v0: First vertex of triangle
        v1: Second vertex of triangle
        v2: Third vertex of triangle
        epsilon: Tolerance for parallel detection

    Returns:
        Intersection point if exists, None otherwise
    """
    origin = line.start()
    direction = line.to_vector()

    # Möller-Trumbore algorithm
    edge1_x = v1.x - v0.x
    edge1_y = v1.y - v0.y
    edge1_z = v1.z - v0.z

    edge2_x = v2.x - v0.x
    edge2_y = v2.y - v0.y
    edge2_z = v2.z - v0.z

    # pvec = direction.cross(edge2)
    pvec_x = direction.y * edge2_z - direction.z * edge2_y
    pvec_y = direction.z * edge2_x - direction.x * edge2_z
    pvec_z = direction.x * edge2_y - direction.y * edge2_x

    # det = edge1.dot(pvec)
    det = edge1_x * pvec_x + edge1_y * pvec_y + edge1_z * pvec_z

    if -epsilon < det < epsilon:
        return None  # Parallel

    inv_det = 1.0 / det

    # tvec = origin - v0
    tvec_x = origin.x - v0.x
    tvec_y = origin.y - v0.y
    tvec_z = origin.z - v0.z

    # u = tvec.dot(pvec) * inv_det
    u = (tvec_x * pvec_x + tvec_y * pvec_y + tvec_z * pvec_z) * inv_det

    if u < -epsilon or u > 1.0 + epsilon:
        return None

    # qvec = tvec.cross(edge1)
    qvec_x = tvec_y * edge1_z - tvec_z * edge1_y
    qvec_y = tvec_z * edge1_x - tvec_x * edge1_z
    qvec_z = tvec_x * edge1_y - tvec_y * edge1_x

    # v = direction.dot(qvec) * inv_det
    v = (direction.x * qvec_x + direction.y * qvec_y + direction.z * qvec_z) * inv_det

    if v < -epsilon or u + v > 1.0 + epsilon:
        return None

    # t = edge2.dot(qvec) * inv_det
    t = (edge2_x * qvec_x + edge2_y * qvec_y + edge2_z * qvec_z) * inv_det

    # Calculate intersection point: origin + t * direction
    return Point(
        origin.x + t * direction.x,
        origin.y + t * direction.y,
        origin.z + t * direction.z,
    )


def _mesh_triangles(mesh: Mesh) -> List[Tuple[Point, Point, Point]]:
    vertices, faces = mesh.to_vertices_and_faces()
    tris: List[Tuple[Point, Point, Point]] = []
    for face in faces:
        if len(face) < 3:
            continue
        v0 = vertices[face[0]]
        for i in range(1, len(face) - 1):
            v1 = vertices[face[i]]
            v2 = vertices[face[i + 1]]
            tris.append((v0, v1, v2))
    return tris


def ray_mesh(
    line: Line, mesh: Mesh, epsilon: float = 1e-6, find_all: bool = True
) -> Optional[List[Point]]:
    tris = _mesh_triangles(mesh)
    if not tris:
        return None

    hits: List[Tuple[float, Point]] = []
    origin = line.start()
    direction = line.to_vector().normalize()

    for v0, v1, v2 in tris:
        p = ray_triangle(line, v0, v1, v2, epsilon)
        if p is None:
            continue
        t = (
            (p.x - origin.x) * direction.x
            + (p.y - origin.y) * direction.y
            + (p.z - origin.z) * direction.z
        )
        if t >= 0.0:
            hits.append((t, p))

    if not hits:
        return None

    hits.sort(key=lambda tp: tp[0])
    if find_all:
        return [p for _, p in hits]
    else:
        return [hits[0][1]]


def ray_mesh_bvh(
    line: Line, mesh: Mesh, epsilon: float = 1e-6, find_all: bool = True
) -> Optional[List[Point]]:
    tris = _mesh_triangles(mesh)
    if not tris:
        return None

    # Build AABBs for triangles
    tri_boxes: List[BoundingBox] = []
    for v0, v1, v2 in tris:
        tri_boxes.append(BoundingBox.from_points([v0, v1, v2]))

    world_size = BVH.compute_world_size(tri_boxes)
    bvh = BVH.from_boxes(tri_boxes, world_size)

    origin = line.start()
    direction = line.to_vector().normalize()
    candidate_ids: List[int] = []
    found = bvh.ray_cast(origin, direction, candidate_ids, True)
    if not found:
        return None

    hits: List[Tuple[float, Point]] = []
    for idx in candidate_ids:
        if 0 <= idx < len(tris):
            v0, v1, v2 = tris[idx]
            p = ray_triangle(line, v0, v1, v2, epsilon)
            if p is None:
                continue
            t = (
                (p.x - origin.x) * direction.x
                + (p.y - origin.y) * direction.y
                + (p.z - origin.z) * direction.z
            )
            if t >= 0.0:
                hits.append((t, p))

    if not hits:
        return None

    hits.sort(key=lambda tp: tp[0])
    if find_all:
        return [p for _, p in hits]
    else:
        return [hits[0][1]]
