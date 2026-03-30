from session_compas.session import _get_module
from session_py.nurbscurve import NurbsCurve
from session_py.nurbssurface import NurbsSurface
from session_py.point import Point
from compas_viewer import Viewer

p = Point
crv = NurbsCurve.create(False, 3, [p(0,0,0), p(1,2,0), p(3,1,0), p(5,3,0), p(7,0,0)])
c0 = NurbsCurve.create(False, 3, [p(0,0,0), p(2,0,0), p(4,0,0), p(6,0,0)])
c1 = NurbsCurve.create(False, 3, [p(0,2,1), p(2,2,2), p(4,2,1), p(6,2,0)])
c2 = NurbsCurve.create(False, 3, [p(0,4,0), p(2,4,1), p(4,4,2), p(6,4,1)])
c3 = NurbsCurve.create(False, 3, [p(0,6,0), p(2,6,0), p(4,6,0), p(6,6,0)])
srf = NurbsSurface.create_loft([c0, c1, c2, c3])

viewer = Viewer()
viewer.scene.add(_get_module("NurbsCurve").to_compas(crv), name="curve")
viewer.scene.add(_get_module("NurbsSurface").to_compas(srf), name="surface")
viewer.show()
