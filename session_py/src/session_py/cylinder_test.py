import os
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


def test_cylinder_json_serialization():
    line = Line(0.0, 0.0, 0.0, 5.0, 0.0, 0.0)
    cylinder = Cylinder(line, 2.0)

    json_data = cylinder.to_json_data()
    deserialized = Cylinder.from_json_data(json_data)

    assert deserialized.radius == 2.0
    assert deserialized.mesh.number_of_vertices() == 20
    assert deserialized.mesh.number_of_faces() == 20


def test_cylinder_to_json_data():
    line = Line(0.0, 0.0, 0.0, 10.0, 0.0, 0.0)
    cylinder = Cylinder(line, 1.5)

    json_data = cylinder.to_json_data()
    assert json_data["type"] == "Cylinder"
    assert "radius" in json_data
    assert json_data["radius"] == 1.5


def test_cylinder_from_json_data():
    line = Line(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
    cylinder = Cylinder(line, 0.5)

    json_data = cylinder.to_json_data()
    deserialized = Cylinder.from_json_data(json_data)

    assert deserialized.radius == 0.5
    assert deserialized.mesh.number_of_vertices() == 20
    assert deserialized.mesh.number_of_faces() == 20


def test_cylinder_to_json_from_json():
    line = Line(0.0, 0.0, 0.0, 0.0, 0.0, 8.0)
    cylinder = Cylinder(line, 1.0)

    filepath = "test_cylinder.json"
    cylinder.to_json(filepath)

    loaded = Cylinder.from_json(filepath)
    assert loaded.radius == 1.0
    assert loaded.mesh.number_of_vertices() == 20
    assert loaded.mesh.number_of_faces() == 20
