from session_py.vertex import Vertex


def test_vertex_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    vertex = Vertex("v0", "attribute")

    path = Path(__file__).resolve().parents[2] / "test_vertex.json"
    json_dump(vertex, path)
    loaded = json_load(path)

    assert isinstance(loaded, Vertex)
    assert loaded.name == "v0"
    assert loaded.attribute == "attribute"
