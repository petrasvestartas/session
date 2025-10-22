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

    print(
        f"✓ Python plane_plane: {start.x}, {start.y}, {start.z} -> {end.x}, {end.y}, {end.z}"
    )


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


def test_line_plane_intersection():
    """Test line-plane intersection with real-world values"""
    from . import Line
    from . import Plane
    from . import Point
    from . import Vector
    from . import intersection

    l0 = Line(500.000, -573.576, -819.152, 500.000, 573.576, 819.152)

    plane_origin_0 = Point(213.787107, 513.797811, -24.743845)
    plane_xaxis_0 = Vector(0.907673, -0.258819, 0.330366)
    plane_yaxis_0 = Vector(0.272094, 0.96225, 0.006285)
    pl0 = Plane(plane_origin_0, plane_xaxis_0, plane_yaxis_0)

    lp = intersection.line_plane(l0, pl0, True)

    assert lp is not None
    assert abs(lp.x - 500.0) < 0.1
    assert abs(lp.y - 77.7531) < 0.01
    assert abs(lp.z - 111.043) < 0.01


def test_ray_box_intersection():
    from . import Line
    from . import BoundingBox
    from . import Point
    from . import intersection

    l0 = Line(500.0, -573.576, -819.152, 500.0, 573.576, 819.152)
    min_p = Point(214.0, 192.0, 484.0)
    max_p = Point(694.0, 567.0, 796.0)
    box = BoundingBox.from_points([min_p, max_p])

    points = intersection.ray_box(l0, box, 0.0, 1000.0)

    assert points is not None
    assert len(points) == 2

    # Entry point
    assert abs(points[0].x - 500.0) < 0.1
    assert abs(points[0].y - 338.9) < 0.1
    assert abs(points[0].z - 484.0) < 0.1

    # Exit point
    assert abs(points[1].x - 500.0) < 0.1
    assert abs(points[1].y - 557.365) < 0.1
    assert abs(points[1].z - 796.0) < 0.1


def test_ray_box_no_intersection():
    from . import Line
    from . import BoundingBox
    from . import Point
    from . import intersection

    l0 = Line(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    min_p = Point(10.0, 10.0, 10.0)
    max_p = Point(20.0, 20.0, 20.0)
    box = BoundingBox.from_points([min_p, max_p])

    points = intersection.ray_box(l0, box, 0.0, 1000.0)

    assert points is None


def test_ray_sphere_intersection():
    from . import Line
    from . import Point
    from . import intersection

    l0 = Line(500.0, -573.576, -819.152, 500.0, 573.576, 819.152)
    sphere_center = Point(457.0, 192.0, 207.0)
    radius = 265.0

    points = intersection.ray_sphere(l0, sphere_center, radius)

    assert points is not None
    assert len(points) == 2

    # First intersection point
    assert abs(points[0].x - 500.0) < 0.1
    assert abs(points[0].y - 12.08) < 0.1
    assert abs(points[0].z - 17.25) < 0.1

    # Second intersection point
    assert abs(points[1].x - 500.0) < 0.1
    assert abs(points[1].y - 308.77) < 0.1
    assert abs(points[1].z - 440.97) < 0.1


def test_ray_sphere_no_intersection():
    from . import Line
    from . import Point
    from . import intersection

    l0 = Line(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    sphere_center = Point(0.0, 10.0, 0.0)
    radius = 5.0

    points = intersection.ray_sphere(l0, sphere_center, radius)

    assert points is None


def test_ray_triangle_intersection():
    from . import Line
    from . import Point
    from . import intersection
    from . import Tolerance

    l0 = Line(500.0, -573.576, -819.152, 500.0, 573.576, 819.152)
    p1 = Point(214.0, 567.0, 484.0)
    p2 = Point(214.0, 192.0, 796.0)
    p3 = Point(694.0, 192.0, 484.0)

    triangle_hit = intersection.ray_triangle(l0, p1, p2, p3, Tolerance.APPROXIMATION)

    assert triangle_hit is not None
    assert abs(triangle_hit.x - 500.0) < 0.1
    assert abs(triangle_hit.y - 340.616) < 0.01
    assert abs(triangle_hit.z - 486.451) < 0.01


def test_ray_triangle_no_intersection():
    from . import Line
    from . import Point
    from . import intersection
    from . import Tolerance

    l0 = Line(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    p1 = Point(10.0, 10.0, 10.0)
    p2 = Point(10.0, 20.0, 10.0)
    p3 = Point(10.0, 15.0, 20.0)

    triangle_hit = intersection.ray_triangle(l0, p1, p2, p3, Tolerance.APPROXIMATION)

    assert triangle_hit is None
