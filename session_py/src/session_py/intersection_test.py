from . import Line
from . import intersection
from .tolerance import Tolerance


def test_line_line_intersection():
    l0 = Line(500.000, -573.576, -819.152, 500.000, 573.576, 819.152)
    l1 = Line(13.195, 234.832, 534.315, 986.805, 421.775, 403.416)
    
    p = intersection.line_line(l0, l1, Tolerance.APPROXIMATION)
    
    assert p is not None
    assert abs(p.x - 500.0) < 0.1
    assert abs(p.y - 328.303) < 0.1
    assert abs(p.z - 468.866) < 0.1
    print(f"✓ Python line_line: {p.x}, {p.y}, {p.z}")


def test_line_line_parameters():
    l0 = Line(500.000, -573.576, -819.152, 500.000, 573.576, 819.152)
    l1 = Line(13.195, 234.832, 534.315, 986.805, 421.775, 403.416)
    
    result = intersection.line_line_parameters(l0, l1, Tolerance.APPROXIMATION)
    
    assert result is not None
    t0, t1 = result
    assert 0.0 <= t0 <= 1.0
    assert 0.0 <= t1 <= 1.0
    print(f"✓ Python line_line_parameters: t0={t0}, t1={t1}")


def test_line_line_with_approximation_tolerance():
    """Test that Tolerance.APPROXIMATION works correctly"""
    l0 = Line(500.000, -573.576, -819.152, 500.000, 573.576, 819.152)
    l1 = Line(13.195, 234.832, 534.315, 986.805, 421.775, 403.416)
    
    # Must explicitly provide Tolerance.APPROXIMATION
    p = intersection.line_line(l0, l1, Tolerance.APPROXIMATION)
    assert p is not None
    
    result = intersection.line_line_parameters(l0, l1, Tolerance.APPROXIMATION)
    assert result is not None


def test_plane_plane_intersection():
    """Test plane-plane intersection with complex real-world values"""
    from .plane import Plane
    from .point import Point
    from .vector import Vector
    
    plane_origin_0 = Point(213.787107, 513.797811, -24.743845)
    plane_xaxis_0 = Vector(0.907673, -0.258819, 0.330366)
    plane_yaxis_0 = Vector(0.272094, 0.96225, 0.006285)
    pl0 = Plane(plane_origin_0, plane_xaxis_0, plane_yaxis_0)
    
    plane_origin_1 = Point(247.17924, 499.115486, 59.619568)
    plane_xaxis_1 = Vector(0.552465, 0.816035, 0.16991)
    plane_yaxis_1 = Vector(0.172987, 0.087156, -0.98106)
    pl1 = Plane(plane_origin_1, plane_xaxis_1, plane_yaxis_1)
    
    intersection_line = intersection.plane_plane(pl0, pl1)
    
    assert intersection_line is not None
    
    start = intersection_line.start()
    end = intersection_line.end()
    
    assert abs(start.x - 252.4632) < 0.01
    assert abs(start.y - 495.32248) < 0.01
    assert abs(start.z - (-10.002656)) < 0.01
    
    assert abs(end.x - 253.01033) < 0.01
    assert abs(end.y - 496.1218) < 0.01
    assert abs(end.z - (-9.888727)) < 0.01
    
    print(f"✓ Python plane_plane: {start.x}, {start.y}, {start.z} -> {end.x}, {end.y}, {end.z}")


def test_plane_plane_plane_intersection():
    """Test plane-plane-plane intersection with real-world values"""
    from .plane import Plane
    from .point import Point
    from .vector import Vector
    
    plane_origin_0 = Point(213.787107, 513.797811, -24.743845)
    plane_xaxis_0 = Vector(0.907673, -0.258819, 0.330366)
    plane_yaxis_0 = Vector(0.272094, 0.96225, 0.006285)
    pl0 = Plane(plane_origin_0, plane_xaxis_0, plane_yaxis_0)
    
    plane_origin_1 = Point(247.17924, 499.115486, 59.619568)
    plane_xaxis_1 = Vector(0.552465, 0.816035, 0.16991)
    plane_yaxis_1 = Vector(0.172987, 0.087156, -0.98106)
    pl1 = Plane(plane_origin_1, plane_xaxis_1, plane_yaxis_1)
    
    plane_origin_2 = Point(221.399816, 605.893667, -54.000116)
    plane_xaxis_2 = Vector(0.903451, -0.360516, -0.231957)
    plane_yaxis_2 = Vector(0.172742, -0.189057, 0.966653)
    pl2 = Plane(plane_origin_2, plane_xaxis_2, plane_yaxis_2)
    
    ppp = intersection.plane_plane_plane(pl0, pl1, pl2)
    
    assert ppp is not None
    assert abs(ppp.x - 300.5) < 0.1
    assert abs(ppp.y - 565.5) < 0.1
    assert abs(ppp.z - 0.0) < 0.1
    
    print(f"✓ Python plane_plane_plane: {ppp.x}, {ppp.y}, {ppp.z}")
