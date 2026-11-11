from __future__ import annotations

import importlib
import os
import subprocess
import tempfile
from pathlib import Path

_MODULE_MAP = {
    "Point": "session_compas.compas_point",
    "Line": "session_compas.compas_line",
    "Polyline": "session_compas.compas_polyline",
    "Mesh": "session_compas.compas_mesh",
    "PointCloud": "session_compas.compas_pointcloud",
    "NurbsCurve": "session_compas.compas_nurbscurve",
    "NurbsSurface": "session_compas.compas_nurbssurface",
    "Plane": "session_compas.compas_plane",
    "BRep": "session_compas.compas_brep",
}


def _get_module(type_name):
    return importlib.import_module(_MODULE_MAP[type_name])


def to_compas(obj):
    module = _get_module(type(obj).__name__)
    return module.to_compas(obj)


def _publish_script() -> Path:
    """Return SESSION_PUBLISH_SCRIPT, else the nearest bash/publish-scene.sh above this package."""

    script = os.environ.get("SESSION_PUBLISH_SCRIPT")
    if script:
        return Path(script)
    for parent in Path(__file__).resolve().parents:
        candidate = parent / "bash" / "publish-scene.sh"
        if candidate.is_file():
            return candidate
    raise FileNotFoundError(
        "bash/publish-scene.sh not found above session_compas; set SESSION_PUBLISH_SCRIPT"
    )


def publish(session_or_path, notify: bool = True) -> str:
    """Publish a session_py Session or a .pb file to the live viewer and return the publisher's report line."""

    script = _publish_script()
    with tempfile.TemporaryDirectory() as tmp:
        if isinstance(session_or_path, (str, os.PathLike)):
            pb = Path(session_or_path)
        else:
            pb = Path(tmp) / "view_live.pb"
            session_or_path.pb_dump(pb)
        args = ["bash", str(script), str(pb)]
        if not notify:
            args.append("--no-notify")
        result = subprocess.run(args, capture_output=True, text=True)
    if result.returncode != 0:
        raise RuntimeError(
            result.stderr.strip() or f"{script} exited with {result.returncode}"
        )
    return result.stdout.strip() or result.stderr.strip()
