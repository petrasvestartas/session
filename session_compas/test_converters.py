from __future__ import annotations

import pytest

pytest.importorskip("compas")


def test_point():
    from session_compas import to_compas
    from session_py import Point

    cp = to_compas(Point(1, 2, 3))
    assert [cp.x, cp.y, cp.z] == [1, 2, 3]


def test_line():
    from session_compas import to_compas
    from session_py import Line

    cl = to_compas(Line(0, 0, 0, 1, 2, 3))
    assert list(cl.end) == [1, 2, 3]


def test_polyline():
    from session_compas import to_compas
    from session_py import Point
    from session_py import Polyline

    cp = to_compas(Polyline([Point(0, 0, 0), Point(1, 0, 0), Point(1, 1, 0)]))
    assert len(cp.points) == 3
    assert list(cp.points[2]) == [1, 1, 0]


def test_pointcloud():
    from session_compas import to_compas
    from session_py import Point
    from session_py import PointCloud

    cp = to_compas(PointCloud([Point(0, 0, 0), Point(1, 2, 3)]))
    assert len(cp.points) == 2
    assert list(cp.points[1]) == [1, 2, 3]


def test_mesh():
    from session_compas import to_compas
    from session_py import Mesh

    cm = to_compas(Mesh.create_box(1, 1, 1))
    assert cm.number_of_vertices() == 8
    assert cm.number_of_faces() == 6


def test_plane():
    from session_compas import to_compas
    from session_py import Plane

    rect, normal = to_compas(Plane())
    assert len(rect.points) == 5
    assert list(normal.end) == [0, 0, 1]


def test_nurbscurve():
    from session_compas import to_compas
    from session_py import NurbsCurve
    from session_py import Point

    crv = NurbsCurve.create(False, 3, [Point(0, 0, 0), Point(1, 2, 0), Point(3, 1, 0), Point(5, 3, 0), Point(7, 0, 0)])
    cp = to_compas(crv)
    assert len(cp.points) == 101
    assert list(cp.points[0]) == pytest.approx([0, 0, 0])
    assert list(cp.points[-1]) == pytest.approx([7, 0, 0])


def test_nurbssurface():
    from session_compas import to_compas
    from session_py import NurbsCurve
    from session_py import Point
    from session_py import Primitives

    c0 = NurbsCurve.create(False, 3, [Point(0, 0, 0), Point(2, 0, 0), Point(4, 0, 0), Point(6, 0, 0)])
    c1 = NurbsCurve.create(False, 3, [Point(0, 2, 1), Point(2, 2, 2), Point(4, 2, 1), Point(6, 2, 0)])
    c2 = NurbsCurve.create(False, 3, [Point(0, 4, 0), Point(2, 4, 1), Point(4, 4, 2), Point(6, 4, 1)])
    c3 = NurbsCurve.create(False, 3, [Point(0, 6, 0), Point(2, 6, 0), Point(4, 6, 0), Point(6, 6, 0)])
    cm = to_compas(Primitives.create_loft([c0, c1, c2, c3]))
    assert cm.number_of_vertices() == 31 * 31
    assert cm.number_of_faces() == 30 * 30
