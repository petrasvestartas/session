from __future__ import annotations

import pytest

FAKE = """#!/usr/bin/env bash
cp "$1" "$(dirname "$0")/published.pb"
echo "$@"
"""


@pytest.fixture
def fake_script(tmp_path, monkeypatch):
    script = tmp_path / "publish-scene.sh"
    script.write_text(FAKE)
    monkeypatch.setenv("SESSION_PUBLISH_SCRIPT", str(script))
    return script


def _session():
    from session_py import Point
    from session_py import PointCloud
    from session_py import Session

    session = Session()
    session.add_point(Point(1, 2, 3))
    session.add_point(Point(4, 5, 6))
    session.add_pointcloud(PointCloud([Point(0, 0, 0), Point(1, 1, 1)]))
    return session


def test_publish_session(fake_script):
    from session_compas import publish
    from session_py import Session

    line = publish(_session())
    loaded = Session.pb_load(fake_script.parent / "published.pb")
    assert line.endswith("view_live.pb")
    assert len(loaded.objects.points) == 2
    assert len(loaded.objects.pointclouds) == 1


def test_publish_path_no_notify(fake_script, tmp_path):
    from session_compas import publish

    pb = tmp_path / "scene.pb"
    _session().pb_dump(pb)
    assert publish(pb, notify=False) == f"{pb} --no-notify"
    assert (fake_script.parent / "published.pb").read_bytes() == pb.read_bytes()


def test_publish_failure(tmp_path, monkeypatch):
    from session_compas import publish

    script = tmp_path / "publish-scene.sh"
    script.write_text("echo 'upload refused' >&2\nexit 3\n")
    monkeypatch.setenv("SESSION_PUBLISH_SCRIPT", str(script))
    with pytest.raises(RuntimeError, match="upload refused"):
        publish(tmp_path / "missing.pb")


def test_publish_script_found_above_package(monkeypatch):
    from session_compas.session import _publish_script

    monkeypatch.delenv("SESSION_PUBLISH_SCRIPT", raising=False)
    assert _publish_script().name == "publish-scene.sh"
