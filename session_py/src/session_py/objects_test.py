from .objects import Objects
from .point import Point


def test_objects_constructor():
    objects = Objects()
    assert objects.name == "my_objects"
    assert objects.guid is not None
    assert len(objects.points) == 0


def test_objects_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    objects = Objects()
    objects.points.append(Point(1.0, 2.0, 3.0))
    objects.name = "test_objects"

    path = Path(__file__).resolve().parents[2] / "test_objects.json"
    json_dump(objects, path)
    loaded = json_load(path)

    assert isinstance(loaded, Objects)
    assert len(loaded.points) == 1
    assert loaded.points[0].x == 1.0
    assert loaded.name == objects.name
