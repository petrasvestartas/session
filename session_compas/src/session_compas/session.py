import importlib
import json

_MODULE_MAP = {
    "Point":        "session_compas.compas_point",
    "Line":         "session_compas.compas_line",
    "Polyline":     "session_compas.compas_polyline",
    "Mesh":         "session_compas.compas_mesh",
    "PointCloud":   "session_compas.compas_pointcloud",
    "NurbsCurve":   "session_compas.compas_nurbscurve",
    "NurbsSurface": "session_compas.compas_nurbssurface",
}


def _get_module(type_name):
    return importlib.import_module(_MODULE_MAP[type_name])


def to_compas(obj):
    module = _get_module(type(obj).__name__)
    return module.to_compas(obj)


def view(filepath):
    from compas_viewer import Viewer
    from session_py.session import Session as PySession
    from session_py.session_config import SESSION_CONFIG
    from session_py.xform import Xform

    if str(filepath).endswith(".pb"):
        data = PySession.pb_load(filepath)
    else:
        with open(filepath, "r") as f:
            raw = json.load(f)
        data = PySession.__jsonload__(raw)

    sf = SESSION_CONFIG.scale_factor
    xf = Xform.scale_xyz(sf, sf, sf) if sf != 1.0 else None

    viewer = Viewer()
    collections = [
        data.objects.points, data.objects.lines,
        data.objects.polylines, data.objects.meshes,
        data.objects.nurbscurves, data.objects.nurbssurfaces,
    ]
    for col in collections:
        for obj in col:
            if xf is not None:
                obj.xform = xf
                obj.transform()
            type_name = type(obj).__name__
            if type_name not in _MODULE_MAP:
                continue
            module = _get_module(type_name)
            compas_obj = module.to_compas(obj)
            name = getattr(obj, "name", None) or type_name
            viewer.scene.add(compas_obj, name=name)
    viewer.show()
