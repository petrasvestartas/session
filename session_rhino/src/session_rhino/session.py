import Rhino
import importlib
import System
import json
import tempfile
from pathlib import Path

_GUID_FILE = Path(tempfile.gettempdir()) / "session_rhino_guids.json"

_MODULE_MAP = {
    "Point":      "session_rhino.rhino_point",
    "Line":       "session_rhino.rhino_line",
    "Plane":      "session_rhino.rhino_plane",
    "Mesh":       "session_rhino.rhino_mesh",
    "NurbsCurve": "session_rhino.rhino_nurbscurve",
}


def _get_module(type_name):
    return importlib.import_module(_MODULE_MAP[type_name])


def _load_guids():
    if _GUID_FILE.exists():
        return json.loads(_GUID_FILE.read_text())
    return []


def _save_guids(guids):
    _GUID_FILE.write_text(json.dumps(guids))


class Session:
    def __init__(self):
        self._scene = []

    def add(self, obj_or_list, **kwargs):
        if not isinstance(obj_or_list, list):
            obj_or_list = [obj_or_list]
        for obj in obj_or_list:
            self._scene.append((obj, kwargs))

    def draw(self, delete=True):
        doc = Rhino.RhinoDoc.ActiveDoc
        if delete:
            for guid_str in _load_guids():
                doc.Objects.Delete(System.Guid(guid_str), True)
        new_guids = []
        for obj, kwargs in self._scene:
            module = _get_module(type(obj).__name__)
            guids = module.add(obj, **kwargs)
            new_guids.extend(str(g) for g in guids)
        self._scene.clear()
        if delete:
            _save_guids(new_guids)
        else:
            _save_guids(_load_guids() + new_guids)
        doc.Views.Redraw()
        return new_guids

    def to_rhino(self, obj):
        module = _get_module(type(obj).__name__)
        return module.to_rhino(obj)
