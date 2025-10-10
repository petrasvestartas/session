from session_py.boundingbox import BoundingBox
from session_py.point import Point
from session_py.vector import Vector
from session_py.plane import Plane


def test_box_default_constructor():
    boundingbox = BoundingBox()
    assert boundingbox.center.x == 0.0
    assert boundingbox.center.y == 0.0
    assert boundingbox.center.z == 0.0
    assert boundingbox.x_axis.x == 1.0
    assert boundingbox.y_axis.y == 1.0
    assert boundingbox.z_axis.z == 1.0
    assert boundingbox.half_size.x == 0.5
    assert boundingbox.half_size.y == 0.5
    assert boundingbox.half_size.z == 0.5
    assert boundingbox.guid is not None


def test_box_constructor_with_parameters():
    center = Point(1.0, 2.0, 3.0)
    x_axis = Vector(1.0, 0.0, 0.0)
    y_axis = Vector(0.0, 1.0, 0.0)
    z_axis = Vector(0.0, 0.0, 1.0)
    half_size = Vector(2.0, 3.0, 4.0)

    boundingbox = BoundingBox(center, x_axis, y_axis, z_axis, half_size)

    assert boundingbox.center.x == 1.0
    assert boundingbox.center.y == 2.0
    assert boundingbox.center.z == 3.0
    assert boundingbox.half_size.x == 2.0
    assert boundingbox.half_size.y == 3.0
    assert boundingbox.half_size.z == 4.0


def test_box_from_plane():
    plane = Plane(Point(0.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0), Vector(0.0, 1.0, 0.0))
    boundingbox = BoundingBox.from_plane(plane, 4.0, 6.0, 8.0)

    assert boundingbox.center.x == 0.0
    assert boundingbox.half_size.x == 2.0
    assert boundingbox.half_size.y == 3.0
    assert boundingbox.half_size.z == 4.0


def test_box_corners():
    center = Point(0.0, 0.0, 0.0)
    x_axis = Vector(1.0, 0.0, 0.0)
    y_axis = Vector(0.0, 1.0, 0.0)
    z_axis = Vector(0.0, 0.0, 1.0)
    half_size = Vector(1.0, 1.0, 1.0)

    boundingbox = BoundingBox(center, x_axis, y_axis, z_axis, half_size)
    corners = boundingbox.corners()

    assert len(corners) == 8
    assert corners[0].x == 1.0
    assert corners[0].y == 1.0
    assert corners[0].z == -1.0


def test_box_two_rectangles():
    center = Point(0.0, 0.0, 0.0)
    x_axis = Vector(1.0, 0.0, 0.0)
    y_axis = Vector(0.0, 1.0, 0.0)
    z_axis = Vector(0.0, 0.0, 1.0)
    half_size = Vector(1.0, 1.0, 1.0)

    boundingbox = BoundingBox(center, x_axis, y_axis, z_axis, half_size)
    rects = boundingbox.two_rectangles()

    assert len(rects) == 10
    assert rects[0].x == rects[4].x
    assert rects[0].y == rects[4].y
    assert rects[0].z == rects[4].z


def test_box_collision_overlapping():
    box1 = BoundingBox(
        Point(0.0, 0.0, 0.0),
        Vector(1.0, 0.0, 0.0),
        Vector(0.0, 1.0, 0.0),
        Vector(0.0, 0.0, 1.0),
        Vector(1.0, 1.0, 1.0),
    )
    box2 = BoundingBox(
        Point(0.5, 0.0, 0.0),
        Vector(1.0, 0.0, 0.0),
        Vector(0.0, 1.0, 0.0),
        Vector(0.0, 0.0, 1.0),
        Vector(1.0, 1.0, 1.0),
    )

    assert box1.collides_with(box2)


def test_box_collision_separated():
    box1 = BoundingBox(
        Point(0.0, 0.0, 0.0),
        Vector(1.0, 0.0, 0.0),
        Vector(0.0, 1.0, 0.0),
        Vector(0.0, 0.0, 1.0),
        Vector(1.0, 1.0, 1.0),
    )
    box2 = BoundingBox(
        Point(5.0, 0.0, 0.0),
        Vector(1.0, 0.0, 0.0),
        Vector(0.0, 1.0, 0.0),
        Vector(0.0, 0.0, 1.0),
        Vector(1.0, 1.0, 1.0),
    )

    assert not box1.collides_with(box2)


def test_box_json_serialization():
    boundingbox = BoundingBox(
        Point(1.0, 2.0, 3.0),
        Vector(1.0, 0.0, 0.0),
        Vector(0.0, 1.0, 0.0),
        Vector(0.0, 0.0, 1.0),
        Vector(2.0, 3.0, 4.0),
    )

    data = boundingbox.to_json_data()

    assert "center" in data
    assert "x_axis" in data
    assert "y_axis" in data
    assert "z_axis" in data
    assert "half_size" in data
    assert "guid" in data
    assert "name" in data


def test_box_json_round_trip():
    original = BoundingBox(
        Point(1.0, 2.0, 3.0),
        Vector(1.0, 0.0, 0.0),
        Vector(0.0, 1.0, 0.0),
        Vector(0.0, 0.0, 1.0),
        Vector(2.0, 3.0, 4.0),
    )

    data = original.to_json_data()
    loaded = BoundingBox.from_json_data(data)

    assert loaded.center.x == original.center.x
    assert loaded.center.y == original.center.y
    assert loaded.center.z == original.center.z
    assert loaded.half_size.x == original.half_size.x
    assert loaded.half_size.y == original.half_size.y
    assert loaded.half_size.z == original.half_size.z
    assert loaded.name == original.name


def test_box_to_json_from_json():
    import os

    original = BoundingBox(
        Point(1.0, 2.0, 3.0),
        Vector(1.0, 0.0, 0.0),
        Vector(0.0, 1.0, 0.0),
        Vector(0.0, 0.0, 1.0),
        Vector(2.0, 3.0, 4.0),
    )
    filename = "test_boundingbox.json"

    original.to_json(filename)
    loaded = BoundingBox.from_json(filename)

    assert loaded.center.x == original.center.x
    assert loaded.center.y == original.center.y
    assert loaded.center.z == original.center.z
    assert loaded.half_size.x == original.half_size.x
    assert loaded.half_size.y == original.half_size.y
    assert loaded.half_size.z == original.half_size.z
    assert loaded.name == original.name
    assert loaded.guid == original.guid


def test_box_inflate():
    boundingbox = BoundingBox(
        Point(0.0, 0.0, 0.0),
        Vector(1.0, 0.0, 0.0),
        Vector(0.0, 1.0, 0.0),
        Vector(0.0, 0.0, 1.0),
        Vector(1.0, 2.0, 3.0),
    )

    boundingbox.inflate(0.5)

    assert boundingbox.half_size.x == 1.5
    assert boundingbox.half_size.y == 2.5
    assert boundingbox.half_size.z == 3.5


def test_box_from_point():
    pt = Point(1.0, 2.0, 3.0)
    boundingbox = BoundingBox.from_point(pt)

    assert boundingbox.center.x == 1.0
    assert boundingbox.center.y == 2.0
    assert boundingbox.center.z == 3.0
    assert boundingbox.half_size.x == 0.0
    assert boundingbox.half_size.y == 0.0
    assert boundingbox.half_size.z == 0.0


def test_box_from_point_with_inflate():
    pt = Point(1.0, 2.0, 3.0)
    boundingbox = BoundingBox.from_point(pt, 0.5)

    assert boundingbox.center.x == 1.0
    assert boundingbox.center.y == 2.0
    assert boundingbox.center.z == 3.0
    assert boundingbox.half_size.x == 0.5
    assert boundingbox.half_size.y == 0.5
    assert boundingbox.half_size.z == 0.5


def test_box_from_points():
    points = [Point(0.0, 0.0, 0.0), Point(2.0, 4.0, 6.0)]
    boundingbox = BoundingBox.from_points(points)

    assert boundingbox.center.x == 1.0
    assert boundingbox.center.y == 2.0
    assert boundingbox.center.z == 3.0
    assert boundingbox.half_size.x == 1.0
    assert boundingbox.half_size.y == 2.0
    assert boundingbox.half_size.z == 3.0


def test_box_from_points_with_inflate():
    points = [Point(0.0, 0.0, 0.0), Point(2.0, 4.0, 6.0)]
    boundingbox = BoundingBox.from_points(points, 0.5)

    assert boundingbox.center.x == 1.0
    assert boundingbox.center.y == 2.0
    assert boundingbox.center.z == 3.0
    assert boundingbox.half_size.x == 1.5
    assert boundingbox.half_size.y == 2.5
    assert boundingbox.half_size.z == 3.5


def test_box_from_line():
    from session_py.line import Line

    line = Line(0.0, 0.0, 0.0, 10.0, 0.0, 0.0)
    boundingbox = BoundingBox.from_line(line)

    assert boundingbox.center.x == 5.0
    assert boundingbox.center.y == 0.0
    assert boundingbox.center.z == 0.0
    assert boundingbox.half_size.x == 5.0


def test_box_from_line_with_inflate():
    from session_py.line import Line

    line = Line(0.0, 0.0, 0.0, 10.0, 0.0, 0.0)
    boundingbox = BoundingBox.from_line(line, 1.0)

    assert boundingbox.center.x == 5.0
    assert boundingbox.center.y == 0.0
    assert boundingbox.center.z == 0.0
    assert boundingbox.half_size.x == 6.0
    assert boundingbox.half_size.y == 1.0
    assert boundingbox.half_size.z == 1.0


def test_box_from_polyline():
    from session_py.polyline import Polyline

    points = [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0)]
    polyline = Polyline(points)
    boundingbox = BoundingBox.from_polyline(polyline)

    assert boundingbox.center.x == 0.5
    assert boundingbox.center.y == 0.5
    assert boundingbox.center.z == 0.0
    assert boundingbox.half_size.x == 0.5
    assert boundingbox.half_size.y == 0.5
    assert boundingbox.half_size.z == 0.0


def test_box_from_polyline_with_inflate():
    from session_py.polyline import Polyline

    points = [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0)]
    polyline = Polyline(points)
    boundingbox = BoundingBox.from_polyline(polyline, 0.5)

    assert boundingbox.center.x == 0.5
    assert boundingbox.center.y == 0.5
    assert boundingbox.center.z == 0.0
    assert boundingbox.half_size.x == 1.0
    assert boundingbox.half_size.y == 1.0
    assert boundingbox.half_size.z == 0.5
