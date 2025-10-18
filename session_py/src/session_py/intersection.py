from typing import Optional, Tuple
from .line import Line
from .point import Point
from .tolerance import Tolerance


def line_line_parameters(
    line0: Line,
    line1: Line,
    tolerance: float,
    intersect_segments: bool = True,
    near_parallel_as_closest: bool = False
) -> Optional[Tuple[float, float]]:
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


def line_line(
    line0: Line,
    line1: Line,
    tolerance: float
) -> Optional[Point]:
    result = line_line_parameters(line0, line1, tolerance, True, False)
    
    if result is None:
        return None
    
    t0, t1 = result
    p0 = line0.point_at(t0)
    p1 = line1.point_at(t1)
    
    return Point(
        (p0.x + p1.x) * 0.5,
        (p0.y + p1.y) * 0.5,
        (p0.z + p1.z) * 0.5
    )


def plane_plane(plane0, plane1) -> Optional[Line]:
    from .plane import Plane
    
    d = plane1.z_axis.cross(plane0.z_axis)
    
    p = Point(
        (plane0.origin.x + plane1.origin.x) * 0.5,
        (plane0.origin.y + plane1.origin.y) * 0.5,
        (plane0.origin.z + plane1.origin.z) * 0.5
    )
    
    plane2 = Plane.from_point_normal(p, d)
    
    output_p = plane_plane_plane(plane0, plane1, plane2)
    if output_p is None:
        return None
    
    return Line(
        output_p.x, output_p.y, output_p.z,
        output_p.x + d.x, output_p.y + d.y, output_p.z + d.z
    )


def plane_plane_plane(plane0, plane1, plane2) -> Optional[Point]:
    from .plane import Plane
    
    n0 = plane0.z_axis
    n1 = plane1.z_axis
    n2 = plane2.z_axis
    
    det = n0.dot(n1.cross(n2))
    
    if abs(det) < 1e-10:
        return None
    
    d0 = plane0.d
    d1 = plane1.d
    d2 = plane2.d
    
    p = (n1.cross(n2) * (-d0) + n2.cross(n0) * (-d1) + n0.cross(n1) * (-d2)) * (1.0 / det)
    
    return Point(p.x, p.y, p.z)
