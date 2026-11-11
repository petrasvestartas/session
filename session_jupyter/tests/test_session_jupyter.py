import base64
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


def test_show_draws_in_the_page_without_upload_rights(monkeypatch):
    monkeypatch.setattr(session_jupyter, "can_upload", lambda: False)
    monkeypatch.setattr(session_jupyter, "UPLOAD", "")
    monkeypatch.setattr(session_jupyter, "publish", lambda *args: pytest.fail("must not upload"))
    session = Session("net")
    session.add_line(Line(0.0, 0.0, 0.0, 1.0, 0.0, 0.0))

    frame = session_jupyter.show(session, "net")

    assert frame.src.startswith(session_jupyter.INLINE_VIEWER) and "#session=" in frame.src
    payload = frame.src.split("#session=", 1)[1]
    assert base64.b64decode(payload) == session.pb_dumps()


def test_show_uploads_with_upload_rights(monkeypatch):
    monkeypatch.setattr(session_jupyter, "can_upload", lambda: True)
    monkeypatch.setattr(session_jupyter, "publish", lambda session, name, view, grid: f"published:{name}:{view}")

    assert session_jupyter.show(Session("net"), "net", view="top").src == "published:net:top"


def test_can_upload_needs_the_r2_profile(monkeypatch, tmp_path):
    monkeypatch.setattr(session_jupyter.Path, "home", lambda: tmp_path)
    assert session_jupyter.can_upload() is False


def test_show_sends_to_the_upload_service_without_upload_rights(monkeypatch):
    sent = []

    def post(address, body):
        sent.append((address, body))
        return json.dumps({"scene": "tmp/abc.yaml", "url": "https://viewer/?scene=tmp/abc.yaml"})

    monkeypatch.setattr(session_jupyter, "can_upload", lambda: False)
    monkeypatch.setattr(session_jupyter, "UPLOAD", "https://upload.example/upload")
    monkeypatch.setattr(session_jupyter, "_post", post)
    session = Session("net")
    session.add_line(Line(0.0, 0.0, 0.0, 1.0, 0.0, 0.0))

    frame = session_jupyter.show(session, "net", view="top")

    assert frame.src == "https://viewer/?scene=tmp/abc.yaml"
    assert sent[0][0] == "https://upload.example/upload?view=top&grid=0"
    assert gzip.decompress(sent[0][1]) == session_jupyter.canonical(session.pb_dumps())


def test_show_falls_back_to_the_page_when_the_service_fails(monkeypatch):
    def post(address, body):
        raise OSError("offline")

    monkeypatch.setattr(session_jupyter, "can_upload", lambda: False)
    monkeypatch.setattr(session_jupyter, "UPLOAD", "https://upload.example/upload")
    monkeypatch.setattr(session_jupyter, "_post", post)

    assert "#session=" in session_jupyter.show(Session("net"), "net").src


def test_the_same_geometry_gives_the_same_bytes():
    def net():
        session = Session("net")
        session.add_line(Line(0.0, 0.0, 0.0, 1.0, 0.0, 0.0))
        session.add_line(Line(1.0, 0.0, 0.0, 1.0, 1.0, 0.0))
        return session

    first = session_jupyter.canonical(net().pb_dumps())

    for _ in range(30):  # the order of some parts follows the random ids, so try many runs
        assert session_jupyter.canonical(net().pb_dumps()) == first
    restored = Session.pb_loads(first)
    assert len(restored.objects.lines) == 2
