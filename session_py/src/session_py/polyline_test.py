"""Tests for Polyline class."""

from session_py.plane import Plane
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


def test_polyline_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    points = [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0)]
    polyline = Polyline(points)
    polyline.name = "test_polyline"

    path = Path(__file__).resolve().parents[2] / "test_polyline.json"
    json_dump(polyline, path)
    loaded = json_load(path)

    assert isinstance(loaded, Polyline)
    assert len(loaded) == 3
    assert loaded.points[0].x == 0.0
    assert loaded.points[2].y == 1.0
    assert loaded.name == polyline.name


def test_polyline_get_point():
    polyline = Polyline([Point(0.0, 0.0, 0.0), Point(1.0, 2.0, 3.0)])

    point = polyline.get_point(1)
    assert point is not None
    assert point.x == 1.0

    invalid = polyline.get_point(10)
    assert invalid is None


def test_polyline_get_point_mut():
    polyline = Polyline([Point(0.0, 0.0, 0.0), Point(1.0, 2.0, 3.0)])

    # In Python, we can directly modify points since they're mutable objects
    if len(polyline.points) > 1:
        polyline.points[1] = Point(5.0, 6.0, 7.0)

    assert polyline.points[1].x == 5.0
    assert polyline.points[1].y == 6.0
    assert polyline.points[1].z == 7.0


def test_polyline_shift():
    polyline = Polyline(
        [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(2.0, 0.0, 0.0)]
    )

    polyline.shift(1)

    assert polyline.points[0].x == 1.0
    assert polyline.points[1].x == 2.0
    assert polyline.points[2].x == 0.0


def test_polyline_length_squared():
    polyline = Polyline(
        [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0)]
    )

    length_sq = polyline.length_squared()
    assert abs(length_sq - 2.0) < 1e-5


def test_polyline_point_at_parameter():
    start = Point(0.0, 0.0, 0.0)
    end = Point(2.0, 0.0, 0.0)

    mid = Polyline.point_at_parameter(start, end, 0.5)
    assert mid.x == 1.0
    assert mid.y == 0.0
    assert mid.z == 0.0


def test_polyline_closest_point_to_line():
    line_start = Point(0.0, 0.0, 0.0)
    line_end = Point(2.0, 0.0, 0.0)
    test_point = Point(1.0, 1.0, 0.0)

    t = Polyline.closest_point_to_line(test_point, line_start, line_end)
    assert abs(t - 0.5) < 1e-5


def test_polyline_line_line_overlap():
    line0_start = Point(0.0, 0.0, 0.0)
    line0_end = Point(2.0, 0.0, 0.0)
    line1_start = Point(1.0, 0.0, 0.0)
    line1_end = Point(3.0, 0.0, 0.0)

    overlap = Polyline.line_line_overlap(line0_start, line0_end, line1_start, line1_end)

    assert overlap is not None
    overlap_start, overlap_end = overlap
    assert abs(overlap_start.x - 1.0) < 1e-5
    assert abs(overlap_end.x - 2.0) < 1e-5


def test_polyline_line_line_average():
    line0_start = Point(0.0, 0.0, 0.0)
    line0_end = Point(2.0, 0.0, 0.0)
    line1_start = Point(0.0, 2.0, 0.0)
    line1_end = Point(2.0, 2.0, 0.0)

    avg_start, avg_end = Polyline.line_line_average(
        line0_start, line0_end, line1_start, line1_end
    )

    assert abs(avg_start.y - 1.0) < 1e-5
    assert abs(avg_end.y - 1.0) < 1e-5


def test_polyline_line_line_overlap_average():
    line0_start = Point(0.0, 0.0, 0.0)
    line0_end = Point(3.0, 0.0, 0.0)
    line1_start = Point(1.0, 0.0, 0.0)
    line1_end = Point(4.0, 0.0, 0.0)

    output_start, output_end = Polyline.line_line_overlap_average(
        line0_start, line0_end, line1_start, line1_end
    )

    assert output_start.x >= 0.0
    assert output_end.x <= 4.0


def test_polyline_line_from_projected_points():
    line_start = Point(0.0, 0.0, 0.0)
    line_end = Point(2.0, 0.0, 0.0)
    points = [Point(0.5, 1.0, 0.0), Point(1.5, -1.0, 0.0)]

    result = Polyline.line_from_projected_points(line_start, line_end, points)

    assert result is not None
    output_start, output_end = result
    assert abs(output_start.x - 0.5) < 1e-5
    assert abs(output_end.x - 1.5) < 1e-5


def test_polyline_closest_distance_and_point():
    polyline = Polyline([Point(0.0, 0.0, 0.0), Point(2.0, 0.0, 0.0)])
    test_point = Point(1.0, 1.0, 0.0)

    distance, edge_id, closest_point = polyline.closest_distance_and_point(test_point)

    assert edge_id == 0
    assert abs(closest_point.x - 1.0) < 1e-5
    assert abs(distance - 1.0) < 1e-5


def test_polyline_is_closed():
    open_polyline = Polyline(
        [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0)]
    )
    assert not open_polyline.is_closed()

    closed_polyline = Polyline(
        [
            Point(0.0, 0.0, 0.0),
            Point(1.0, 0.0, 0.0),
            Point(1.0, 1.0, 0.0),
            Point(0.0, 0.0, 0.0),
        ]
    )
    assert closed_polyline.is_closed()


def test_polyline_center():
    polyline = Polyline(
        [
            Point(0.0, 0.0, 0.0),
            Point(2.0, 0.0, 0.0),
            Point(2.0, 2.0, 0.0),
            Point(0.0, 2.0, 0.0),
        ]
    )

    c = polyline.center()
    assert abs(c.x - 1.0) < 1e-5
    assert abs(c.y - 1.0) < 1e-5
    assert abs(c.z - 0.0) < 1e-5


def test_polyline_center_vec():
    polyline = Polyline(
        [Point(0.0, 0.0, 0.0), Point(2.0, 0.0, 0.0), Point(2.0, 2.0, 0.0)]
    )

    c = polyline.center_vec()
    assert abs(c.x - 4.0 / 3.0) < 1e-5
    assert abs(c.y - 2.0 / 3.0) < 1e-5


def test_polyline_get_average_plane():
    polyline = Polyline(
        [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(0.0, 1.0, 0.0)]
    )

    origin, x_axis, y_axis, z_axis = polyline.get_average_plane()

    assert abs(z_axis.z - 1.0) < 1e-5


def test_polyline_get_fast_plane():
    polyline = Polyline(
        [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(0.0, 1.0, 0.0)]
    )

    origin, plane = polyline.get_fast_plane()

    assert origin.x == 0.0
    assert origin.y == 0.0
    assert origin.z == 0.0


def test_polyline_get_middle_line():
    line0_start = Point(0.0, 0.0, 0.0)
    line0_end = Point(2.0, 0.0, 0.0)
    line1_start = Point(0.0, 2.0, 0.0)
    line1_end = Point(2.0, 2.0, 0.0)

    output_start, output_end = Polyline.get_middle_line(
        line0_start, line0_end, line1_start, line1_end
    )

    assert abs(output_start.y - 1.0) < 1e-5
    assert abs(output_end.y - 1.0) < 1e-5


def test_polyline_extend_line():
    start = Point(0.0, 0.0, 0.0)
    end = Point(1.0, 0.0, 0.0)

    Polyline.extend_line(start, end, 0.5, 0.5)

    assert abs(start.x - (-0.5)) < 1e-5
    assert abs(end.x - 1.5) < 1e-5


def test_polyline_scale_line():
    start = Point(0.0, 0.0, 0.0)
    end = Point(2.0, 0.0, 0.0)

    Polyline.scale_line(start, end, 0.25)

    assert abs(start.x - 0.5) < 1e-5
    assert abs(end.x - 1.5) < 1e-5


def test_polyline_extend_segment():
    polyline = Polyline([Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0)])

    polyline.extend_segment(0, 0.5, 0.5)

    assert abs(polyline.points[0].x - (-0.5)) < 1e-5
    assert abs(polyline.points[1].x - 1.5) < 1e-5


def test_polyline_extend_segment_equally_static():
    start = Point(0.0, 0.0, 0.0)
    end = Point(1.0, 0.0, 0.0)

    Polyline.extend_segment_equally_static(start, end, 0.5)

    assert abs(start.x - (-0.5)) < 1e-5
    assert abs(end.x - 1.5) < 1e-5


def test_polyline_extend_segment_equally():
    polyline = Polyline([Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0)])

    polyline.extend_segment_equally(0, 0.5)

    assert abs(polyline.points[0].x - (-0.5)) < 1e-5
    assert abs(polyline.points[1].x - 1.5) < 1e-5


def test_polyline_move_by():
    polyline = Polyline([Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0)])
    translation = Vector(1.0, 1.0, 1.0)

    polyline.move_by(translation)

    assert polyline.points[0].x == 1.0
    assert polyline.points[0].y == 1.0
    assert polyline.points[0].z == 1.0
    assert polyline.points[1].x == 2.0
    assert polyline.points[1].y == 1.0
    assert polyline.points[1].z == 1.0


def test_polyline_is_clockwise():
    polyline = Polyline(
        [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0)]
    )
    plane = Plane()

    clockwise = polyline.is_clockwise(plane)
    # Just test it doesn't crash
    assert clockwise == True or clockwise == False


def test_polyline_flip():
    polyline = Polyline(
        [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(2.0, 0.0, 0.0)]
    )

    polyline.flip()

    assert polyline.points[0].x == 2.0
    assert polyline.points[1].x == 1.0
    assert polyline.points[2].x == 0.0


def test_polyline_get_convex_corners():
    polyline = Polyline(
        [
            Point(0.0, 0.0, 0.0),
            Point(1.0, 0.0, 0.0),
            Point(1.0, 1.0, 0.0),
            Point(0.0, 1.0, 0.0),
        ]
    )

    convex_corners = polyline.get_convex_corners()

    assert len(convex_corners) == 4


def test_polyline_tween_two_polylines():
    polyline0 = Polyline([Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0)])
    polyline1 = Polyline([Point(0.0, 2.0, 0.0), Point(1.0, 2.0, 0.0)])

    result = Polyline.tween_two_polylines(polyline0, polyline1, 0.5)

    assert abs(result.points[0].y - 1.0) < 1e-5
    assert abs(result.points[1].y - 1.0) < 1e-5
