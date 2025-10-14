from .cylinder import Cylinder
from .line import Line


def test_cylinder_new():
    line = Line(0.0, 0.0, 0.0, 0.0, 0.0, 10.0)
    cylinder = Cylinder(line, 1.0)

    assert cylinder.radius == 1.0
    assert cylinder.mesh.number_of_vertices() == 20
    assert cylinder.mesh.number_of_faces() == 20
    assert len(cylinder.guid) > 0
    assert cylinder.name == "my_cylinder"


def test_cylinder_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    line = Line(0.0, 0.0, 0.0, 0.0, 0.0, 10.0)
    cylinder = Cylinder(line, 1.0)
    cylinder.name = "test_cylinder"

    path = Path(__file__).resolve().parents[2] / "test_cylinder.json"
    json_dump(cylinder, path)
    loaded = json_load(path)

    assert isinstance(loaded, Cylinder)
    assert loaded.radius == cylinder.radius
    assert loaded.line.z1 == cylinder.line.z1
    assert loaded.name == cylinder.name
