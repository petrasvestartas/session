import copy
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
    "Plane":        "session_compas.compas_plane",
    "BRep":         "session_compas.compas_brep",
}


def _get_module(type_name):
    return importlib.import_module(_MODULE_MAP[type_name])


def to_compas(obj):
    module = _get_module(type(obj).__name__)
    return module.to_compas(obj)


def view(filepath_or_session):
    from compas_viewer import Viewer
    from compas.colors import Color as CColor
    from session_py.session import Session as PySession
    from session_py.session_config import SESSION_CONFIG
    from session_py.xform import Xform

    if isinstance(filepath_or_session, PySession):
        data = filepath_or_session
    elif str(filepath_or_session).endswith(".pb"):
        data = PySession.pb_load(filepath_or_session)
    else:
        with open(filepath_or_session, "r") as f:
            raw = json.load(f)
        data = PySession.__jsonload__(raw)

    sf = SESSION_CONFIG.scale_factor
    xf = Xform.scale_xyz(sf, sf, sf) if sf != 1.0 else None

    viewer = Viewer()

    # Build viewer groups from session tree
    # Group TreeNodes are direct children of root whose name is not a guid
    viewer_groups = {}  # guid (of geometry obj) -> viewer group
    for group_node in data.tree.root.children:
        vg = viewer.scene.add_group(group_node.name)
        for child in group_node.children:
            # child.name is the guid of a geometry object
            viewer_groups[child.name] = vg

    def _get_parent(obj):
        """Return the viewer group for this object, or None."""
        guid = getattr(obj, "guid", None)
        if guid is None:
            return None
        return viewer_groups.get(guid)

    collections = [
        data.objects.points, data.objects.lines,
        data.objects.polylines, data.objects.meshes,
        data.objects.nurbscurves, data.objects.nurbssurfaces,
        data.objects.breps,
    ]
    for col in collections:
        for obj in col:
            if xf is not None:
                obj.transform(xf)
            type_name = type(obj).__name__
            if type_name not in _MODULE_MAP:
                continue
            module = _get_module(type_name)
            compas_obj = module.to_compas(obj)
            name = getattr(obj, "name", None) or type_name
            parent = _get_parent(obj)
            viewer.scene.add(compas_obj, name=name, parent=parent)
    for plane_obj in data.objects.planes:
        if xf is not None:
            plane_obj.transform(xf)
        module = _get_module("Plane")
        rect, normal = module.to_compas(plane_obj)
        name = getattr(plane_obj, "name", None) or "Plane"
        c = plane_obj.linecolor
        kw = {"linecolor": CColor(c[0]/255, c[1]/255, c[2]/255)}
        parent = _get_parent(plane_obj)
        viewer.scene.add(rect, name=name, parent=parent, **kw)
        viewer.scene.add(normal, name=name + "_normal", parent=parent, **kw)
    for elem in data.objects.elements:
        # The Session owns placement now, so session_geometry takes it as an argument.
        geom = elem.session_geometry(xf if xf is not None else Xform.identity())
        if geom is None:
            continue
        type_name = type(geom).__name__
        if type_name not in _MODULE_MAP:
            continue
        module = _get_module(type_name)
        if type_name == "BRep":
            fallback = getattr(elem, "_fallback_mesh", None)
            if fallback is not None and xf is not None:
                fallback = copy.deepcopy(fallback)
                fallback.transform(xf)
            compas_obj = module.to_compas(geom, fallback_mesh=fallback)
        else:
            compas_obj = module.to_compas(geom)
        name = getattr(elem, "name", None) or "Element"
        parent = _get_parent(elem)
        viewer.scene.add(compas_obj, name=name, parent=parent)
    viewer.show()
