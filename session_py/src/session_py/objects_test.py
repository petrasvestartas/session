from .objects import Objects
from .point import Point


def test_objects_constructor():
    objects = Objects()
    assert objects.name == "my_objects"
    assert objects.guid is not None
    assert len(objects.points) == 0


def test_objects_to_json_data():
    objects = Objects()
    point1 = Point(1.0, 2.0, 3.0)
    point2 = Point(4.0, 5.0, 6.0)
    point3 = Point(7.0, 8.0, 9.0)
    objects.points = [point1, point2, point3]
    data = objects.to_json_data()
    assert data["name"] == "my_objects"
    assert "guid" in data
    assert len(data["points"]) == 3
    assert data["points"][0]["x"] == 1.0
    assert data["points"][1]["y"] == 5.0
    assert data["points"][2]["z"] == 9.0


def test_objects_from_json_data():
    objects = Objects()
    point1 = Point(10.0, 20.0, 30.0)
    point2 = Point(40.0, 50.0, 60.0)
    objects.points = [point1, point2]
    data = objects.to_json_data()
    objects2 = Objects.from_json_data(data)
    assert objects2.name == "my_objects"
    assert len(objects2.points) == 2
    assert objects2.points[0].x == 10.0
    assert objects2.points[1].z == 60.0


def test_objects_to_json_from_json():
    objects = Objects()
    point1 = Point(100.0, 200.0, 300.0)
    point2 = Point(400.0, 500.0, 600.0)
    point3 = Point(700.0, 800.0, 900.0)
    objects.points = [point1, point2, point3]
    filename = "test_objects.json"

    objects.to_json(filename)
    loaded_objects = Objects.from_json(filename)

    assert loaded_objects.name == objects.name
    assert len(loaded_objects.points) == len(objects.points)
    assert loaded_objects.points[0].x == objects.points[0].x
    assert loaded_objects.points[2].z == objects.points[2].z
