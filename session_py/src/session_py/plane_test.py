import math
from .plane import Plane
from .point import Point
from .vector import Vector


def test_plane_default_constructor():
    plane = Plane()
    assert plane.origin == Point(0.0, 0.0, 0.0)
    assert plane.x_axis == Vector.x_axis()
    assert plane.y_axis == Vector.y_axis()
    assert plane.z_axis == Vector.z_axis()
    assert plane.a == 0.0
    assert plane.b == 0.0
    assert plane.c == 1.0
    assert plane.d == 0.0


def test_plane_constructor_from_origin_and_axes():
    origin = Point(1.0, 2.0, 3.0)
    x = Vector(1.0, 0.0, 0.0)
    y = Vector(0.0, 1.0, 0.0)
    plane = Plane(origin, x, y, "test_plane")
    assert plane.name == "test_plane"
    assert plane.origin == origin
    assert plane.c == 1.0


def test_plane_from_point_normal():
    p = Point(0.0, 0.0, 5.0)
    n = Vector(0.0, 0.0, 1.0)
    plane = Plane.from_point_normal(p, n)
    assert plane.origin == p
    assert abs(plane.z_axis.z - 1.0) < 1e-5
    assert abs(plane.d + 5.0) < 1e-5


def test_plane_from_points():
    points = [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(0.0, 1.0, 0.0)]
    plane = Plane.from_points(points)
    assert abs(plane.c - 1.0) < 1e-5
    assert abs(plane.d) < 1e-5


def test_plane_from_two_points():
    p1 = Point(0.0, 0.0, 0.0)
    p2 = Point(1.0, 0.0, 0.0)
    plane = Plane.from_two_points(p1, p2)
    assert plane.origin == p1


def test_plane_xy_plane():
    plane = Plane.xy_plane()
    assert plane.name == "xy_plane"
    assert plane.a == 0.0
    assert plane.b == 0.0
    assert plane.c == 1.0
    assert plane.d == 0.0


def test_plane_yz_plane():
    plane = Plane.yz_plane()
    assert plane.name == "yz_plane"
    assert plane.a == 1.0
    assert plane.b == 0.0
    assert plane.c == 0.0
    assert plane.d == 0.0


def test_plane_xz_plane():
    plane = Plane.xz_plane()
    assert plane.name == "xz_plane"
    assert plane.a == 0.0
    assert plane.b == 1.0
    assert plane.c == 0.0
    assert plane.d == 0.0


def test_plane_to_string():
    plane = Plane.xy_plane()
    s = str(plane)
    assert "Plane" in s
    assert "xy_plane" in s


def test_plane_operator_getitem():
    plane = Plane()
    assert plane[0] == Vector.x_axis()
    assert plane[1] == Vector.y_axis()
    assert plane[2] == Vector.z_axis()


def test_plane_operator_iadd_translation():
    plane = Plane.xy_plane()
    offset = Vector(1.0, 2.0, 3.0)
    plane += offset
    assert plane.origin.x == 1.0
    assert plane.origin.y == 2.0
    assert plane.origin.z == 3.0
    assert abs(plane.d + 3.0) < 1e-5


def test_plane_operator_isub_translation():
    plane = Plane.xy_plane()
    offset = Vector(1.0, 2.0, 3.0)
    plane -= offset
    assert plane.origin.x == -1.0
    assert plane.origin.y == -2.0
    assert plane.origin.z == -3.0


def test_plane_operator_add_translation():
    plane = Plane.xy_plane()
    offset = Vector(1.0, 2.0, 3.0)
    moved = plane + offset
    assert moved.origin.z == 3.0
    assert plane.origin.z == 0.0


def test_plane_operator_sub_translation():
    plane = Plane.xy_plane()
    offset = Vector(1.0, 2.0, 3.0)
    moved = plane - offset
    assert moved.origin.z == -3.0


def test_plane_reverse():
    plane = Plane.xy_plane()
    orig_x = Vector(plane.x_axis.x, plane.x_axis.y, plane.x_axis.z)
    orig_y = Vector(plane.y_axis.x, plane.y_axis.y, plane.y_axis.z)
    plane.reverse()
    assert plane.x_axis == orig_y
    assert plane.y_axis == orig_x
    assert plane.c == -1.0


def test_plane_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    plane = Plane.xy_plane()
    plane.name = "test_plane"

    path = Path(__file__).resolve().parents[2] / "test_plane.json"
    json_dump(plane, path)
    loaded = json_load(path)

    assert isinstance(loaded, Plane)
    assert loaded.name == plane.name
    assert loaded.c == plane.c


def test_plane_rotate():
    plane = Plane.xy_plane()
    angle = math.pi / 2.0
    plane.rotate(angle)
    assert abs(plane.x_axis.y - 1.0) < 1e-5


def test_plane_is_same_direction_parallel():
    p1 = Plane.xy_plane()
    p2 = Plane.xy_plane()
    assert Plane.is_same_direction(p1, p2, True)


def test_plane_is_same_direction_flipped():
    p1 = Plane.xy_plane()
    p2 = Plane.xy_plane()
    p2.reverse()
    assert Plane.is_same_direction(p1, p2, True)
    assert not Plane.is_same_direction(p1, p2, False)


def test_plane_is_same_position():
    p1 = Plane.xy_plane()
    p2 = Plane.xy_plane()
    assert Plane.is_same_position(p1, p2)
    p2 += Vector(0.0, 0.0, 1.0)
    assert not Plane.is_same_position(p1, p2)


def test_plane_is_coplanar():
    p1 = Plane.xy_plane()
    p2 = Plane.xy_plane()
    assert Plane.is_coplanar(p1, p2, True)
    p2.reverse()
    assert Plane.is_coplanar(p1, p2, True)
    p2 += Vector(0.0, 0.0, 1.0)
    assert not Plane.is_coplanar(p1, p2, True)


def test_plane_is_right_hand():
    plane = Plane.xy_plane()
    assert plane.is_right_hand()
    plane = Plane.yz_plane()
    assert plane.is_right_hand()
    plane = Plane.xz_plane()
    assert plane.is_right_hand()
    plane = Plane()
    assert plane.is_right_hand()
    plane.reverse()
    assert plane.is_right_hand()
    plane.rotate(math.pi / 4.0)
    assert plane.is_right_hand()


def test_plane_translate_by_normal():
    plane = Plane.xy_plane()

    # Translate along positive normal (Z direction)
    translated = plane.translate_by_normal(5.0)
    assert translated.origin.x == 0.0
    assert translated.origin.y == 0.0
    assert translated.origin.z == 5.0

    # Normal should remain the same
    assert translated.z_axis.x == plane.z_axis.x
    assert translated.z_axis.y == plane.z_axis.y
    assert translated.z_axis.z == plane.z_axis.z

    # Translate along negative normal
    translated_neg = plane.translate_by_normal(-3.0)
    assert translated_neg.origin.x == 0.0
    assert translated_neg.origin.y == 0.0
    assert translated_neg.origin.z == -3.0

    # Test with YZ plane
    yz_plane = Plane.yz_plane()
    yz_translated = yz_plane.translate_by_normal(2.0)
    assert yz_translated.origin.x == 2.0
    assert yz_translated.origin.y == 0.0
    assert yz_translated.origin.z == 0.0
