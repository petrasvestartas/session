import gzip
import json

import pytest
from session_py import Line
from session_py import Session

import session_jupyter


def test_url():
    assert session_jupyter.url("net") == "https://petrasvestartas.github.io/session/?scene=view_net.yaml"


def test_publish_uploads_geometry_then_scene(monkeypatch):
    uploads = []
    monkeypatch.setattr(session_jupyter, "_is_published", lambda key: False)
    monkeypatch.setattr(session_jupyter, "_put", lambda key, data, *rest: uploads.append((key, data)))
    monkeypatch.setattr(session_jupyter, "_notify", lambda message: None)
    session = Session("net")
    session.add_line(Line(0.0, 0.0, 0.0, 1.0, 0.0, 0.0))

    address = session_jupyter.publish(session, "net", view="top")
    scene = json.loads(uploads[1][1])

    assert address.endswith("?scene=view_net.yaml")
    assert uploads[0][0].startswith("pb/revisions/") and uploads[1][0] == "scenes/view_net.yaml"
    assert scene["items"][0]["file"] == uploads[0][0] and scene["view"] == "top"
    assert scene["items"][0]["size"] == len(gzip.decompress(uploads[0][1]))


def test_publish_refuses_bad_names():
    with pytest.raises(ValueError):
        session_jupyter.publish(Session("net"), "bad name")
    with pytest.raises(ValueError):
        session_jupyter.publish(Session("net"), "net", view="sideways")


def test_show_without_upload():
    frame = session_jupyter.show(Session("net"), "net", height=400, upload=False)

    assert frame.src == session_jupyter.url("net") and frame.height == 400
