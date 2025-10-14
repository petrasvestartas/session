import math
from .mesh import Mesh, NormalWeighting
from .point import Point


def test_mesh_constructor():
    mesh = Mesh()
    assert mesh.number_of_vertices() == 0
    assert mesh.number_of_faces() == 0
    assert mesh.is_empty()
    assert mesh.euler() == 0


def test_add_vertex():
    mesh = Mesh()
    v_key = mesh.add_vertex(Point(1.0, 2.0, 3.0))
    assert mesh.number_of_vertices() == 1
    assert not mesh.is_empty()

    pos = mesh.vertex_position(v_key)
    assert pos is not None
    assert pos.x == 1.0
    assert pos.y == 2.0
    assert pos.z == 3.0


def test_add_vertex_with_key():
    mesh = Mesh()
    v_key = mesh.add_vertex(Point(0.0, 0.0, 0.0), 42)
    assert v_key == 42
    assert mesh.number_of_vertices() == 1


def test_add_face():
    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0))
    v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0))
    v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0))

    f_key = mesh.add_face([v0, v1, v2])
    assert f_key is not None
    assert mesh.number_of_faces() == 1
    assert mesh.number_of_edges() == 3
    assert mesh.euler() == 1


def test_add_face_invalid():
    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0))
    v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0))

    assert mesh.add_face([v0, v1]) is None
    assert mesh.add_face([v0, v1, 999]) is None
    assert mesh.add_face([v0, v1, v0]) is None


def test_face_vertices():
    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0))
    v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0))
    v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0))

    f = mesh.add_face([v0, v1, v2])
    vertices = mesh.face_vertices(f)
    assert vertices == [v0, v1, v2]


def test_vertex_neighbors():
    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0))
    v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0))
    v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0))

    mesh.add_face([v0, v1, v2])

    neighbors = mesh.vertex_neighbors(v0)
    assert len(neighbors) == 2
    assert v1 in neighbors
    assert v2 in neighbors


def test_vertex_faces():
    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0))
    v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0))
    v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0))
    v3 = mesh.add_vertex(Point(1.0, 1.0, 0.0))

    f1 = mesh.add_face([v0, v1, v2])
    f2 = mesh.add_face([v1, v3, v2])

    faces = mesh.vertex_faces(v1)
    assert len(faces) == 2
    assert f1 in faces
    assert f2 in faces


def test_is_vertex_on_boundary():
    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0))
    v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0))
    v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0))

    mesh.add_face([v0, v1, v2])

    assert mesh.is_vertex_on_boundary(v0)
    assert mesh.is_vertex_on_boundary(v1)
    assert mesh.is_vertex_on_boundary(v2)


def test_face_normal():
    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0))
    v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0))
    v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0))

    f = mesh.add_face([v0, v1, v2])
    normal = mesh.face_normal(f)

    assert normal is not None
    assert abs(normal.z - 1.0) < 1e-10
    assert abs(normal.x) < 1e-10
    assert abs(normal.y) < 1e-10


def test_vertex_normal():
    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0))
    v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0))
    v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0))

    mesh.add_face([v0, v1, v2])
    normal = mesh.vertex_normal(v0)

    assert normal is not None
    assert abs(normal.z - 1.0) < 1e-10


def test_face_area():
    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0))
    v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0))
    v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0))

    f = mesh.add_face([v0, v1, v2])
    area = mesh.face_area(f)

    assert area is not None
    assert abs(area - 0.5) < 1e-10


def test_vertex_angle_in_face():
    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0))
    v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0))
    v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0))

    f = mesh.add_face([v0, v1, v2])
    angle = mesh.vertex_angle_in_face(v0, f)

    assert angle is not None
    assert abs(angle - math.pi / 2.0) < 1e-6


def test_vertex_normal_weighted_area():
    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0))
    v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0))
    v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0))

    mesh.add_face([v0, v1, v2])
    normal = mesh.vertex_normal_weighted(v0, NormalWeighting.AREA)

    normal_default = mesh.vertex_normal(v0)
    assert abs(normal.x - normal_default.x) < 1e-10
    assert abs(normal.y - normal_default.y) < 1e-10
    assert abs(normal.z - normal_default.z) < 1e-10


def test_vertex_normal_weighted_angle():
    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0))
    v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0))
    v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0))

    mesh.add_face([v0, v1, v2])
    normal = mesh.vertex_normal_weighted(v0, NormalWeighting.ANGLE)

    assert abs(normal.z - 1.0) < 1e-10
    assert abs(normal.x) < 1e-10
    assert abs(normal.y) < 1e-10


def test_vertex_normal_weighted_uniform():
    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0))
    v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0))
    v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0))

    mesh.add_face([v0, v1, v2])
    normal = mesh.vertex_normal_weighted(v0, NormalWeighting.UNIFORM)

    assert abs(normal.z - 1.0) < 1e-10
    assert abs(normal.x) < 1e-10
    assert abs(normal.y) < 1e-10


def test_from_polygons_simple():
    triangle = [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(0.0, 1.0, 0.0)]

    mesh = Mesh.from_polygons([triangle])
    assert mesh.number_of_vertices() == 3
    assert mesh.number_of_faces() == 1
    assert mesh.number_of_edges() == 3


def test_from_polygons_vertex_merging():
    triangle1 = [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(0.0, 1.0, 0.0)]
    triangle2 = [Point(1.0, 0.0, 0.0), Point(0.0, 1.0, 0.0), Point(1.0, 1.0, 0.0)]

    mesh = Mesh.from_polygons([triangle1, triangle2])
    assert mesh.number_of_vertices() == 4
    assert mesh.number_of_faces() == 2


def test_from_polygons_precision():
    triangle1 = [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(0.0, 1.0, 0.0)]
    triangle2 = [
        Point(1.0000001, 0.0, 0.0),
        Point(0.0, 1.0000001, 0.0),
        Point(1.0, 1.0, 0.0),
    ]

    mesh = Mesh.from_polygons([triangle1, triangle2], 1e-6)
    assert mesh.number_of_vertices() == 4
    assert mesh.number_of_faces() == 2


def test_clear():
    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0.0, 0.0, 0.0))
    v1 = mesh.add_vertex(Point(1.0, 0.0, 0.0))
    v2 = mesh.add_vertex(Point(0.0, 1.0, 0.0))
    mesh.add_face([v0, v1, v2])

    assert not mesh.is_empty()
    mesh.clear()
    assert mesh.is_empty()
    assert mesh.number_of_vertices() == 0
    assert mesh.number_of_faces() == 0


def test_vertex_data_color():
    mesh = Mesh()
    v = mesh.add_vertex(Point(0.0, 0.0, 0.0))

    mesh.vertex[v].set_color(1.0, 0.5, 0.0)
    color = mesh.vertex[v].color()
    assert color == [1.0, 0.5, 0.0]


def test_vertex_data_normal():
    mesh = Mesh()
    v = mesh.add_vertex(Point(0.0, 0.0, 0.0))

    mesh.vertex[v].set_normal(0.0, 0.0, 1.0)
    normal = mesh.vertex[v].normal()
    assert normal == [0.0, 0.0, 1.0]


def test_mesh_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    mesh = Mesh()
    v0 = mesh.add_vertex(Point(0, 0, 0))
    v1 = mesh.add_vertex(Point(1, 0, 0))
    v2 = mesh.add_vertex(Point(0, 1, 0))
    mesh.add_face([v0, v1, v2])
    mesh.name = "test_mesh"

    path = Path(__file__).resolve().parents[2] / "test_mesh.json"
    json_dump(mesh, path)
    loaded = json_load(path)

    assert isinstance(loaded, Mesh)
    assert loaded.number_of_vertices() == 3
    assert loaded.number_of_faces() == 1
    assert loaded.name == mesh.name
