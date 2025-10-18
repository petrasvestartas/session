from typing import Optional, Tuple
from .line import Line
from .point import Point


def line_line_parameters(
    line0: Line,
    line1: Line,
    tolerance: float = 1e-12,
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
    tolerance: float = 1e-12
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
