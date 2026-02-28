#! python3
# venv: session_py

import Rhino
import System

from session_py.reload import reload_all
import session_rhino
reload_all()

from session_rhino.session import Session, _load_guids, _save_guids

filepath = r"c:\pc\3_code\code_rust\session\session_data\dataset5.pb"
data = Session.load(filepath)

scene = Session()

for crv in data.objects.nurbscurves:
    scene.add(crv)

for srf in data.objects.nurbssurfaces:
    scene.add(srf)

for mesh in data.objects.meshes:
    scene.add(mesh)

for ln in data.objects.lines:
    scene.add(ln)

for pt in data.objects.points:
    scene.add(pt)

for pl in data.objects.polylines:
    scene.add(pl)

for brep in data.objects.breps:
    scene.add(brep)

guids = scene.draw(delete=True)

doc = Rhino.RhinoDoc.ActiveDoc
tag_guids = []
for i, mesh in enumerate(data.objects.meshes):
    verts, _ = mesh.to_vertices_and_faces()
    if not verts:
        continue
    cx = sum(v[0] for v in verts) / len(verts)
    cy = sum(v[1] for v in verts) / len(verts)
    cz = sum(v[2] for v in verts) / len(verts)
    g = doc.Objects.AddTextDot(str(i), Rhino.Geometry.Point3d(cx, cy, cz))
    if g != System.Guid.Empty:
        tag_guids.append(str(g))

_save_guids(guids + tag_guids)
doc.Views.Redraw()
print("guids:", len(guids), "meshes:", len(data.objects.meshes))
