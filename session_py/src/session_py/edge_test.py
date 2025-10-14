from session_py.edge import Edge


def test_edge_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    edge = Edge("v0", "v1", "attribute")

    path = Path(__file__).resolve().parents[2] / "test_edge.json"
    json_dump(edge, path)
    loaded = json_load(path)

    assert isinstance(loaded, Edge)
    assert loaded.v0 == "v0"
    assert loaded.v1 == "v1"
    assert loaded.attribute == "attribute"
