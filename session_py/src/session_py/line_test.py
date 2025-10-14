import pytest
from .line import Line
from .point import Point
from .vector import Vector
from .color import Color


def test_line_default_constructor():
    line = Line()
    assert line.z1 == 1.0
    assert line.name == "my_line"


def test_line_constructor():
    line = Line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
    assert line.x0 == 1.0
    assert line.z1 == 6.0


def test_line_from_points():
    p1 = Point(1.0, 2.0, 3.0)
    p2 = Point(4.0, 5.0, 6.0)
    line = Line.from_points(p1, p2)
    assert line.y0 == 2.0
    assert line.y1 == 5.0


def test_line_with_name():
    line = Line.with_name("custom", 0.0, 0.0, 0.0, 1.0, 1.0, 1.0)
    assert line.name == "custom"


def test_line_to_string():
    line = Line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
    s = str(line)
    assert "1" in s


def test_line_operator_subscript():
    line = Line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
    assert line[0] == 1.0
    assert line[5] == 6.0


def test_line_operator_subscript_mutable():
    line = Line()
    line[0] = 10.0
    assert line.x0 == 10.0


def test_line_operator_add_assign():
    line = Line(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)
    v = Vector(1.0, 2.0, 3.0)
    line += v
    assert line.x0 == 1.0
    assert line.z1 == 4.0


def test_line_operator_sub_assign():
    line = Line(1.0, 2.0, 3.0, 2.0, 3.0, 4.0)
    v = Vector(1.0, 2.0, 3.0)
    line -= v
    assert line.x0 == 0.0


def test_line_operator_mul_assign():
    line = Line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
    line *= 2.0
    assert line.x0 == 2.0
    assert line.z1 == 12.0


def test_line_operator_div_assign():
    line = Line(2.0, 4.0, 6.0, 8.0, 10.0, 12.0)
    line /= 2.0
    assert line.x0 == 1.0
    assert line.z1 == 6.0


def test_line_operator_add():
    line = Line(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)
    v = Vector(1.0, 2.0, 3.0)
    result = line + v
    assert result.y0 == 2.0


def test_line_operator_sub():
    line = Line(1.0, 2.0, 3.0, 2.0, 3.0, 4.0)
    v = Vector(1.0, 2.0, 3.0)
    result = line - v
    assert result.x0 == 0.0


def test_line_operator_mul():
    line = Line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
    result = line * 2.0
    assert result.x0 == 2.0


def test_line_operator_div():
    line = Line(2.0, 4.0, 6.0, 8.0, 10.0, 12.0)
    result = line / 2.0
    assert result.z1 == 6.0


def test_line_to_vector():
    line = Line(1.0, 2.0, 3.0, 4.0, 6.0, 9.0)
    v = line.to_vector()
    assert v.x == 3.0
    assert v.z == 6.0


def test_line_length():
    line = Line(0.0, 0.0, 0.0, 3.0, 4.0, 0.0)
    assert abs(line.length() - 5.0) < 1e-5


def test_line_squared_length():
    line = Line(0.0, 0.0, 0.0, 3.0, 4.0, 0.0)
    assert abs(line.squared_length() - 25.0) < 1e-5


def test_line_point_at():
    line = Line(0.0, 0.0, 0.0, 10.0, 10.0, 10.0)
    p = line.point_at(0.5)
    assert abs(p.x - 5.0) < 1e-5


def test_line_start():
    line = Line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
    p = line.start()
    assert p.x == 1.0


def test_line_end():
    line = Line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
    p = line.end()
    assert p.x == 4.0


def test_line_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    line = Line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
    line.name = "test_line"

    path = Path(__file__).resolve().parents[2] / "test_line.json"
    json_dump(line, path)
    loaded = json_load(path)

    assert isinstance(loaded, Line)
    assert loaded.x0 == line.x0
    assert loaded.z1 == line.z1
    assert loaded.name == line.name
