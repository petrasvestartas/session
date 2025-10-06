"""Tests for Polyline class."""

from session_py.point import Point
from session_py.polyline import Polyline
from session_py.vector import Vector


def test_polyline_new():
    points = [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(0.0, 1.0, 0.0)]
    polyline = Polyline(points)
    assert len(polyline) == 3
    assert polyline.segment_count() == 2


def test_polyline_default():
    polyline = Polyline()
    assert len(polyline) == 0
    assert polyline.is_empty()
    assert polyline.segment_count() == 0


def test_polyline_length():
    points = [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0)]
    polyline = Polyline(points)
    length = polyline.length()
    assert abs(length - 2.0) < 1e-5


def test_polyline_add_point():
    polyline = Polyline([Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0)])
    assert len(polyline) == 2

    polyline.add_point(Point(1.0, 1.0, 0.0))
    assert len(polyline) == 3
    assert polyline.segment_count() == 2


def test_polyline_insert_point():
    polyline = Polyline([Point(0.0, 0.0, 0.0), Point(2.0, 0.0, 0.0)])

    polyline.insert_point(1, Point(1.0, 0.0, 0.0))
    assert len(polyline) == 3
    assert polyline.points[1].x == 1.0


def test_polyline_remove_point():
    points = [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(2.0, 0.0, 0.0)]
    polyline = Polyline(points)

    removed = polyline.remove_point(1)
    assert removed is not None
    assert removed.x == 1.0
    assert len(polyline) == 2


def test_polyline_reverse():
    points = [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(2.0, 0.0, 0.0)]
    polyline = Polyline(points)

    polyline.reverse()
    assert polyline.points[0].x == 2.0
    assert polyline.points[1].x == 1.0
    assert polyline.points[2].x == 0.0


def test_polyline_reversed():
    points = [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(2.0, 0.0, 0.0)]
    polyline = Polyline(points)

    reversed_polyline = polyline.reversed()
    assert reversed_polyline.points[0].x == 2.0
    assert reversed_polyline.points[1].x == 1.0
    assert reversed_polyline.points[2].x == 0.0

    # Original should be unchanged
    assert polyline.points[0].x == 0.0


def test_polyline_add_assign_vector():
    polyline = Polyline([Point(1.0, 2.0, 3.0), Point(4.0, 5.0, 6.0)])
    v = Vector(4.0, 5.0, 6.0)
    polyline += v

    assert polyline.points[0].x == 5.0
    assert polyline.points[0].y == 7.0
    assert polyline.points[0].z == 9.0
    assert polyline.points[1].x == 8.0
    assert polyline.points[1].y == 10.0
    assert polyline.points[1].z == 12.0


def test_polyline_add_vector():
    polyline = Polyline([Point(1.0, 2.0, 3.0), Point(4.0, 5.0, 6.0)])
    v = Vector(4.0, 5.0, 6.0)
    polyline2 = polyline + v

    assert polyline2.points[0].x == 5.0
    assert polyline2.points[0].y == 7.0
    assert polyline2.points[0].z == 9.0


def test_polyline_sub_assign_vector():
    polyline = Polyline([Point(1.0, 2.0, 3.0), Point(4.0, 5.0, 6.0)])
    v = Vector(4.0, 5.0, 6.0)
    polyline -= v

    assert polyline.points[0].x == -3.0
    assert polyline.points[0].y == -3.0
    assert polyline.points[0].z == -3.0
    assert polyline.points[1].x == 0.0
    assert polyline.points[1].y == 0.0
    assert polyline.points[1].z == 0.0


def test_polyline_sub_vector():
    polyline = Polyline([Point(1.0, 2.0, 3.0), Point(4.0, 5.0, 6.0)])
    v = Vector(4.0, 5.0, 6.0)
    polyline2 = polyline - v

    assert polyline2.points[0].x == -3.0
    assert polyline2.points[0].y == -3.0
    assert polyline2.points[0].z == -3.0
    assert polyline2.points[1].x == 0.0
    assert polyline2.points[1].y == 0.0
    assert polyline2.points[1].z == 0.0


def test_polyline_display():
    polyline = Polyline([Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0)])
    display_str = str(polyline)
    assert "Polyline" in display_str
    assert "points=2" in display_str


def test_polyline_to_json_data():
    polyline = Polyline([Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0)])

    json_string = polyline.to_json_data()
    assert "Polyline" in json_string
    assert "points" in json_string


def test_polyline_from_json_data():
    polyline = Polyline([Point(1.0, 2.0, 3.0), Point(4.0, 5.0, 6.0)])

    json_string = polyline.to_json_data()
    deserialized = Polyline.from_json_data(json_string)

    assert len(deserialized) == 2
    assert deserialized.points[0].x == 1.0
    assert deserialized.points[1].x == 4.0


def test_polyline_to_json_from_json():
    points = [Point(1.0, 2.0, 3.0), Point(4.0, 5.0, 6.0), Point(7.0, 8.0, 9.0)]
    polyline = Polyline(points)

    filepath = "test_polyline.json"
    polyline.to_json(filepath)
    loaded = Polyline.from_json(filepath)

    assert len(loaded) == 3
    assert loaded.points[0].x == 1.0
    assert loaded.points[1].y == 5.0
    assert loaded.points[2].z == 9.0


def test_polyline_get_point():
    polyline = Polyline([Point(0.0, 0.0, 0.0), Point(1.0, 2.0, 3.0)])

    point = polyline.get_point(1)
    assert point is not None
    assert point.x == 1.0

    invalid = polyline.get_point(10)
    assert invalid is None
